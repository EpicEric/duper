//! Duper RPC server implementation.
//!
//! This exposes a [`Server`] type, which can be built up
//! with a familiar builder-like interface via the [`ServerPart`] trait.
//!
//! ```
//! use duper::DuperValue;
//! use duper_rpc::server::{Server, ServerPart, State};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Params {
//!     text: String,
//! }
//!
//! async fn handle_only_state(
//!     State(state): State<AppState>
//! ) -> duper_rpc::Result<u64> {
//!     Ok(state.0)
//! }
//!
//! async fn handle_params(
//!     params: Params,
//!     flag: bool
//! ) -> duper_rpc::Result<String> {
//!     if flag {
//!         Ok(params.text)
//!     } else {
//!         Err(duper_rpc::Error::Custom(DuperValue::String {
//!             identifier: None,
//!             inner: "Flag is false".into()
//!         }))
//!     }
//! }
//!
//! #[derive(Clone)]
//! struct AppState(u64);
//!
//! Server::new()
//!     .method("foo", handle_only_state)
//!     .method("bar", handle_params)
//!     .method("healthy", async || Ok(true))
//!     .with_state(AppState(42))
//! #   .into_service();
//! ```
//!
//! In order to actually turn it into a proper handler (and make the compiler
//! happy), we must turn it into a [`tower`] service, which can be done
//! with [`Method::handle`] for immediate consumption, or returned via
//! [`Method::into_service`].
//!
//! ```
//! use duper_rpc::server::Server;
//!
//! async fn handle_rpc_request(request: duper_rpc::Request) -> Option<duper_rpc::Response> {
//!     Server::new()
//!         // ... your methods here ...
//! # .method("h", async || Ok("h"))
//!         .handle(request)
//!         .await
//!         // Service returns `Result<Option<duper_rpc::Response>, Infallible>`
//!         // - therefore, unwrapping the response is safe.
//!         .unwrap()
//! }
//! ```

use std::{convert::Infallible, marker::PhantomData, pin::Pin, task::Poll};

use duper::{DuperValue, serde::ser::Serializer};
use futures::future::join_all;
use serde_core::Serialize;
use tower::Service;

use crate::{
    Error, Request, RequestCall, Response, ResponseError, ResponseResult, ResponseSuccess, Result,
    handler::Handler,
};

/// A wrapper around a shared state.
///
/// Used in handlers to signal that it must receive a clone
/// of the current state (specified via [`ServerPart::with_state`]).
pub struct State<S>(pub S);

/// The basis of the Duper RPC server, instatiated via [`Server::new`].
pub struct Server<S> {
    _marker: PhantomData<S>,
}

impl<S> Clone for Server<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<S> Default for Server<S> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<S> Server<S> {
    /// Create a new Duper RPC server.
    pub fn new() -> Self {
        Server::default()
    }

    /// Add a method to the server.
    pub fn method<H, R, T>(self, name: impl AsRef<str>, handler: H) -> Method<H, R, T, Self>
    where
        Self: Sized,
    {
        Method {
            name: name.as_ref().to_string(),
            handler,
            next: self,
            _marker: Default::default(),
        }
    }
}

/// A method in a Duper RPC server.
pub struct Method<H, R, T, N> {
    name: String,
    handler: H,
    next: N,
    _marker: PhantomData<fn(T) -> R>,
}

impl<H, R, T, N> Clone for Method<H, R, T, N>
where
    H: Clone,
    N: Clone,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            handler: self.handler.clone(),
            next: self.next.clone(),
            _marker: Default::default(),
        }
    }
}

impl<H, R, T, N> Method<H, R, T, N> {
    /// Add a method to the server.
    pub fn method<H2, R2, T2>(self, name: impl AsRef<str>, handler: H2) -> Method<H2, R2, T2, Self>
    where
        Self: Sized,
    {
        Method {
            name: name.as_ref().to_string(),
            handler,
            next: self,
            _marker: Default::default(),
        }
    }

    /// Add a stateful layer to the server, which is applied
    /// to all methods defined before it.
    pub fn with_state<S2>(self, state: S2) -> WithState<S2, Self>
    where
        Self: Sized,
        S2: Clone + Send + 'static,
    {
        WithState { state, next: self }
    }

    /// Convert this server into a [`tower`] service.
    pub fn into_service(self) -> ServerService<Self>
    where
        Self: Sized + ServerPart<()> + Clone + Send + 'static,
    {
        ServerService { inner: self }
    }

    /// Handle a single RPC [`Request`].
    pub fn handle(
        self,
        req: Request,
    ) -> Pin<
        Box<
            dyn Future<Output = std::result::Result<Option<Response>, Infallible>> + Send + 'static,
        >,
    >
    where
        Self: Sized + ServerPart<()> + Clone + Send + 'static,
    {
        self.into_service().call(req)
    }
}

/// A stateful layer in a Duper RPC server.
pub struct WithState<S, N> {
    state: S,
    next: N,
}

impl<S, N> Clone for WithState<S, N>
where
    S: Clone,
    N: Clone,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            next: self.next.clone(),
        }
    }
}

impl<S, N> WithState<S, N> {
    /// Add a method to the server.
    pub fn method<H, R, T>(self, name: impl AsRef<str>, handler: H) -> Method<H, R, T, Self>
    where
        Self: Sized,
    {
        Method {
            name: name.as_ref().to_string(),
            handler,
            next: self,
            _marker: Default::default(),
        }
    }

    /// Convert this server into a [`tower`] service.
    pub fn into_service(self) -> ServerService<Self>
    where
        Self: Sized + ServerPart<()> + Clone + Send + 'static,
    {
        ServerService { inner: self }
    }

    /// Handle a single RPC [`Request`].
    pub fn handle(
        self,
        req: Request,
    ) -> Pin<
        Box<
            dyn Future<Output = std::result::Result<Option<Response>, Infallible>> + Send + 'static,
        >,
    >
    where
        Self: Sized + ServerPart<()> + Clone + Send + 'static,
    {
        self.into_service().call(req)
    }
}

/// A [`tower`] service created from a layered [`Server`].
pub struct ServerService<I> {
    inner: I,
}

fn handle_call<I>(
    server: I,
    call: RequestCall,
) -> Pin<Box<dyn Future<Output = Option<ResponseResult>> + Send + 'static>>
where
    I: ServerPart<()> + Clone + Send + 'static,
{
    match call {
        RequestCall::Valid { id, method, params } => match id {
            Some(id) => Box::pin(async move {
                match server.serve((), method, params).await {
                    Ok(resp) => Some(ResponseResult::Ok(ResponseSuccess { id, result: resp })),
                    Err(error) => Some(ResponseResult::Err(ResponseError {
                        id: Some(id),
                        error,
                    })),
                }
            }),
            None => {
                #[cfg(feature = "tokio")]
                {
                    tokio::spawn(async move { server.serve((), method, params).await });
                }
                #[cfg(all(not(feature = "tokio"), feature = "smol"))]
                {
                    smol::spawn(async move { server.serve((), method, params).await })
                }
                #[cfg(all(not(feature = "tokio"), not(feature = "smol")))]
                {
                    compile_error!("duper_rpc requires an async runtime");
                }
                Box::pin(async { None })
            }
        },
        RequestCall::Invalid { id, error } => {
            Box::pin(async { Some(ResponseResult::Err(ResponseError { id, error })) })
        }
    }
}

impl<I> Service<Request> for ServerService<I>
where
    I: ServerPart<()> + Clone + Send + 'static,
{
    type Response = Option<Response>;
    type Error = Infallible;
    type Future = Pin<
        Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send + 'static>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), Infallible>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        match req {
            Request::Single(call) => {
                let server = self.inner.clone();
                Box::pin(async move { Ok(handle_call(server, call).await.map(Response::Single)) })
            }
            Request::Batch(request_calls) => {
                let server = self.inner.clone();
                Box::pin(async move {
                    let resp: Vec<ResponseResult> =
                        join_all(request_calls.into_iter().map(|call| {
                            let server = server.clone();
                            handle_call(server, call)
                        }))
                        .await
                        .into_iter()
                        .flatten()
                        .collect();
                    if resp.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(Response::Batch(resp)))
                    }
                })
            }
        }
    }
}

/// A trait to allow composing layers on a [`Server`].
pub trait ServerPart<S>: private::Sealed {
    fn serve(
        &self,
        state: S,
        name: String,
        params: DuperValue<'static>,
    ) -> Pin<Box<dyn Future<Output = Result<DuperValue<'static>>> + Send + 'static>>;
}

mod private {
    pub trait Sealed {}

    impl<S> Sealed for super::Server<S> {}
    impl<H, R, T, N> Sealed for super::Method<H, R, T, N> {}
    impl<S, N> Sealed for super::WithState<S, N> {}
}

impl<S> ServerPart<S> for Server<S> {
    fn serve(
        &self,
        _state: S,
        _name: String,
        _params: DuperValue<'static>,
    ) -> Pin<Box<dyn Future<Output = Result<DuperValue<'static>>> + Send + 'static>> {
        Box::pin(async { Err(Error::MethodNotFound) })
    }
}

impl<S, H, N, R, T> ServerPart<S> for Method<H, R, T, N>
where
    H: Handler<S, R, T>,
    N: ServerPart<S>,
    R: Serialize + Send + 'static,
    S: Clone + Send + 'static,
{
    fn serve(
        &self,
        state: S,
        name: String,
        params: DuperValue<'static>,
    ) -> Pin<Box<dyn Future<Output = Result<DuperValue<'static>>> + Send + 'static>> {
        if self.name == name {
            let handler = self.handler.clone();
            Box::pin(async move {
                let r = handler.call(state, params).await;
                match r.map(|resp| resp.serialize(&mut Serializer::new())) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(_)) => Err(Error::InternalError),
                    Err(error) => Err(error),
                }
            })
        } else {
            self.next.serve(state, name, params)
        }
    }
}

impl<S, S2, N> ServerPart<S2> for WithState<S, N>
where
    S: Clone + Send + 'static,
    N: ServerPart<S>,
{
    fn serve(
        &self,
        _state: S2,
        name: String,
        params: DuperValue<'static>,
    ) -> Pin<Box<dyn Future<Output = Result<DuperValue<'static>>> + Send + 'static>> {
        let state = self.state.clone();
        self.next.serve(state, name, params)
    }
}

#[cfg(test)]
mod rpc_server_tests {
    use std::borrow::Cow;

    use duper::{DuperFloat, DuperKey, DuperObject};

    use crate::RequestId;

    use super::*;

    #[tokio::test]
    async fn hello_world() {
        let Ok(response) = Server::new()
            .method("hello", async || Ok("Hello, world!"))
            .method("bye", async || Ok("Goodbye!"))
            .handle(Request::Single(RequestCall::Valid {
                id: Some(RequestId::Integer {
                    identifier: None,
                    inner: 1,
                }),
                method: "hello".into(),
                params: DuperValue::Null { identifier: None },
            }))
            .await;

        let Some(Response::Single(ResponseResult::Ok(ResponseSuccess { id, result }))) = response
        else {
            panic!("Invalid response {:?}", response);
        };
        assert_eq!(
            id,
            RequestId::Integer {
                identifier: None,
                inner: 1,
            }
        );
        let DuperValue::String { inner, .. } = result else {
            panic!("Invalid result {:?}", result);
        };
        assert_eq!(inner, "Hello, world!");
    }

    #[tokio::test]
    async fn args() {
        async fn args(base: f64, pow: i32) -> Result<f64> {
            Ok(base.powi(pow))
        }

        let Ok(response) = Server::new()
            .method("args", args)
            .handle(Request::Single(RequestCall::Valid {
                id: Some(RequestId::String {
                    identifier: None,
                    inner: "some-id".into(),
                }),
                method: "args".into(),
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![
                        DuperValue::Float {
                            identifier: None,
                            inner: DuperFloat::try_from(2.0).expect("finite float"),
                        },
                        DuperValue::Integer {
                            identifier: None,
                            inner: 10,
                        },
                    ],
                },
            }))
            .await;

        let Some(Response::Single(ResponseResult::Ok(ResponseSuccess { id, result }))) = response
        else {
            panic!("Invalid response {:?}", response);
        };
        assert_eq!(
            id,
            RequestId::String {
                identifier: None,
                inner: "some-id".into(),
            }
        );
        let DuperValue::Float { inner, .. } = result else {
            panic!("Invalid result {:?}", result);
        };
        assert_eq!(inner, 1024.0);
    }

    #[tokio::test]
    async fn state() {
        #[derive(serde::Serialize)]
        struct StateResponse {
            state: String,
            output: String,
        }

        async fn stateful(State(state): State<String>, times: usize) -> Result<StateResponse> {
            Ok(StateResponse {
                output: state.repeat(times),
                state: state,
            })
        }

        let Ok(response) = Server::new()
            .method("stateful", stateful)
            .with_state("hi".to_string())
            .handle(Request::Single(RequestCall::Valid {
                id: Some(RequestId::String {
                    identifier: None,
                    inner: "aaa".into(),
                }),
                method: "stateful".into(),
                params: DuperValue::Integer {
                    identifier: None,
                    inner: 3,
                },
            }))
            .await;

        let Some(Response::Single(ResponseResult::Ok(ResponseSuccess { id, result }))) = response
        else {
            panic!("Invalid response {:?}", response);
        };
        assert_eq!(
            id,
            RequestId::String {
                identifier: None,
                inner: "aaa".into(),
            }
        );
        let DuperValue::Object { inner, .. } = result else {
            panic!("Invalid result {:?}", result);
        };
        assert_eq!(
            inner.get(&DuperKey::from("state")),
            Some(&DuperValue::String {
                identifier: None,
                inner: Cow::Borrowed("hi")
            })
        );
        assert_eq!(
            inner.get(&DuperKey::from("output")),
            Some(&DuperValue::String {
                identifier: None,
                inner: Cow::Borrowed("hihihi")
            })
        );
    }

    #[tokio::test]
    async fn notification() {
        async fn sleep_for_10_seconds() -> Result<()> {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            Ok(())
        }

        let started = std::time::Instant::now();
        let Ok(response) = Server::new()
            .method("sleep", sleep_for_10_seconds)
            .handle(Request::Single(RequestCall::Valid {
                id: None,
                method: "sleep".into(),
                params: DuperValue::Null { identifier: None },
            }))
            .await;
        assert!(response.is_none());
        assert!(started.elapsed() < std::time::Duration::from_secs(10));
    }

    #[tokio::test]
    async fn tower_service() {
        #[derive(serde::Deserialize)]
        struct Params {
            _some_old_field: i64,
            data: Option<bool>,
        }

        async fn unwrap_bool(params: Params) -> Result<bool> {
            Ok(params.data.is_some_and(|boolean| boolean))
        }

        let mut service = Server::new()
            .method("unwrap_bool", unwrap_bool)
            .into_service();

        let result = service
            .call(Request::Batch(vec![
                RequestCall::Valid {
                    id: Some(RequestId::String {
                        identifier: None,
                        inner: "ok".into(),
                    }),
                    method: "unwrap_bool".into(),
                    params: DuperValue::Object {
                        identifier: None,
                        inner: DuperObject::try_from(vec![
                            (
                                DuperKey::from("_some_old_field"),
                                DuperValue::Integer {
                                    identifier: None,
                                    inner: 0,
                                },
                            ),
                            (
                                DuperKey::from("data"),
                                DuperValue::Boolean {
                                    identifier: None,
                                    inner: true,
                                },
                            ),
                        ])
                        .expect("no duplicate keys"),
                    },
                },
                RequestCall::Valid {
                    id: Some(RequestId::String {
                        identifier: None,
                        inner: "inexistent".into(),
                    }),
                    method: "".into(),
                    params: DuperValue::Object {
                        identifier: None,
                        inner: DuperObject::try_from(vec![
                            (
                                DuperKey::from("_some_old_field"),
                                DuperValue::Integer {
                                    identifier: None,
                                    inner: 0,
                                },
                            ),
                            (
                                DuperKey::from("data"),
                                DuperValue::Boolean {
                                    identifier: None,
                                    inner: true,
                                },
                            ),
                        ])
                        .expect("no duplicate keys"),
                    },
                },
                RequestCall::Valid {
                    id: Some(RequestId::String {
                        identifier: None,
                        inner: "invalid_params".into(),
                    }),
                    method: "unwrap_bool".into(),
                    params: DuperValue::Boolean {
                        identifier: None,
                        inner: true,
                    },
                },
                RequestCall::Invalid {
                    id: None,
                    error: Error::ParseError,
                },
            ]))
            .await;

        let Ok(Some(Response::Batch(mut responses))) = result else {
            panic!("Invalid response {:?}", result);
        };
        assert_eq!(responses.len(), 4);

        let ResponseResult::Ok(ResponseSuccess { id, result }) = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(
            id,
            RequestId::String {
                identifier: None,
                inner: "ok".into(),
            }
        );
        let DuperValue::Boolean { inner, .. } = result else {
            panic!("Invalid result {:?}", result);
        };
        assert!(inner);

        let ResponseResult::Err(ResponseError { id, error }) = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(
            id,
            Some(RequestId::String {
                identifier: None,
                inner: "inexistent".into(),
            })
        );
        assert!(matches!(error, Error::MethodNotFound));

        let ResponseResult::Err(ResponseError { id, error }) = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(
            id,
            Some(RequestId::String {
                identifier: None,
                inner: "invalid_params".into(),
            })
        );
        assert!(matches!(error, Error::InvalidParams));

        let ResponseResult::Err(ResponseError { id, error }) = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(id, None);
        assert!(matches!(error, Error::ParseError));

        assert!(responses.is_empty());
        drop(service);
    }
}
