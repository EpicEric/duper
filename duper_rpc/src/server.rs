use std::{convert::Infallible, marker::PhantomData, pin::Pin, task::Poll};

use duper::{DuperValue, serde::ser::Serializer};
use futures::future::join_all;
use serde_core::Serialize;
use tower::Service;

use crate::{Error, Request, RequestCall, Response, ResponseResult, Result, handler::Handler};

pub struct State<S>(pub S);

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

pub fn server<S>() -> Server<S> {
    Server {
        _marker: Default::default(),
    }
}

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

pub struct ServerService<I> {
    inner: I,
}

fn handle_call<I>(
    server: I,
    call: RequestCall,
) -> Pin<Box<dyn Future<Output = Option<ResponseResult>>>>
where
    I: ServerPart<()> + Clone + Send + 'static,
{
    match call {
        RequestCall::Valid { id, method, params } => match id {
            Some(id) => Box::pin(async move {
                match server.serve((), method, params).await {
                    Ok(resp) => Some(ResponseResult::Valid { id, result: resp }),
                    Err(error) => Some(ResponseResult::Invalid {
                        id: Some(id),
                        error,
                    }),
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
            Box::pin(async { Some(ResponseResult::Invalid { id, error }) })
        }
    }
}

impl<I> Service<Request> for ServerService<I>
where
    I: ServerPart<()> + Clone + Send + 'static,
{
    type Response = Option<Response>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>>>>;

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
                        .filter_map(|elem| elem)
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

pub trait ServerPart<S>: private::Sealed {
    fn serve(
        &self,
        state: S,
        name: String,
        params: DuperValue<'static>,
    ) -> Pin<Box<dyn Future<Output = Result<DuperValue<'static>>> + Send + 'static>>;

    fn method<H, R, T>(self, name: impl AsRef<str>, handler: H) -> Method<H, R, T, Self>
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

    fn with_state<S2>(self, state: S2) -> WithState<S2, Self>
    where
        Self: Sized,
        S2: Clone + Send + 'static,
    {
        WithState { state, next: self }
    }
}

mod private {
    pub trait Sealed {}

    impl<S> Sealed for super::Server<S> {}
    impl<H, R, T, N> Sealed for super::Method<H, R, T, N> {}
    impl<S, N> Sealed for super::WithState<S, N> {}
}

pub trait IntoService: ServerPart<()> + Clone + Send + Sized + 'static {
    fn into_service(self) -> ServerService<Self>
    where
        Self: Sized + ServerPart<()> + Clone + Send + 'static,
    {
        ServerService { inner: self }
    }

    fn handle(
        self,
        req: Request,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<Option<Response>, Infallible>> + 'static>>
    where
        Self: Sized + ServerPart<()> + Clone + Send + 'static,
    {
        self.into_service().call(req)
    }
}

impl<T> IntoService for T where T: ServerPart<()> + Clone + Send + Sized + 'static {}

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
mod rpc_tests {
    use std::borrow::Cow;

    use duper::{DuperFloat, DuperKey, DuperObject};

    use crate::RequestId;

    use super::*;

    #[tokio::test]
    async fn hello_world() {
        let Ok(response) = server()
            .method("hello", async || Ok("Hello, world!"))
            .method("bye", async || Ok("Goodbye!"))
            .handle(Request::Single(RequestCall::Valid {
                id: Some(RequestId::I64(1)),
                method: "hello".into(),
                params: DuperValue::Null { identifier: None },
            }))
            .await;

        let Some(Response::Single(ResponseResult::Valid { id, result })) = response else {
            panic!("Invalid response {:?}", response);
        };
        assert_eq!(id, RequestId::I64(1));
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

        let Ok(response) = server()
            .method("args", args)
            .handle(Request::Single(RequestCall::Valid {
                id: Some(RequestId::String("some-id".into())),
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

        let Some(Response::Single(ResponseResult::Valid { id, result })) = response else {
            panic!("Invalid response {:?}", response);
        };
        assert_eq!(id, RequestId::String("some-id".into()));
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

        let Ok(response) = server()
            .method("stateful", stateful)
            .with_state("hi".to_string())
            .handle(Request::Single(RequestCall::Valid {
                id: Some(RequestId::String("aaa".into())),
                method: "stateful".into(),
                params: DuperValue::Integer {
                    identifier: None,
                    inner: 3,
                },
            }))
            .await;

        let Some(Response::Single(ResponseResult::Valid { id, result })) = response else {
            panic!("Invalid response {:?}", response);
        };
        assert_eq!(id, RequestId::String("aaa".into()));
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
        let Ok(response) = server()
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

        let mut service = server().method("unwrap_bool", unwrap_bool).into_service();

        let result = service
            .call(Request::Batch(vec![
                RequestCall::Valid {
                    id: Some(RequestId::String("ok".into())),
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
                    id: Some(RequestId::String("inexistent".into())),
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
                    id: Some(RequestId::String("invalid_params".into())),
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

        let ResponseResult::Valid { id, result } = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(id, RequestId::String("ok".into()));
        let DuperValue::Boolean { inner, .. } = result else {
            panic!("Invalid result {:?}", result);
        };
        assert!(inner);

        let ResponseResult::Invalid { id, error } = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(id, Some(RequestId::String("inexistent".into())));
        assert!(matches!(error, Error::MethodNotFound));

        let ResponseResult::Invalid { id, error } = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(id, Some(RequestId::String("invalid_params".into())));
        assert!(matches!(error, Error::InvalidParams));

        let ResponseResult::Invalid { id, error } = responses.remove(0) else {
            panic!("Invalid response result");
        };
        assert_eq!(id, None);
        assert!(matches!(error, Error::ParseError));

        assert!(responses.is_empty());
        drop(service);
    }
}
