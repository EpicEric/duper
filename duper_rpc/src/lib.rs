use std::{convert::Infallible, marker::PhantomData, pin::Pin, task::Poll};

use duper::{
    DuperValue,
    serde::{de::from_value, ser::Serializer},
};
use futures::future::join_all;
use serde_core::{Deserialize, Serialize};
use tower::Service;

#[derive(Debug)]
pub enum Request {
    Single(RequestCall),
    Batch(Vec<RequestCall>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestId {
    String(String),
    I64(i64),
}

#[derive(Debug)]
pub enum RequestCall {
    Valid {
        id: Option<RequestId>,
        method: String,
        params: DuperValue<'static>,
    },
    Invalid {
        id: Option<RequestId>,
        error: Error,
    },
}

#[derive(Debug)]
pub enum Response {
    Single(ResponseResult),
    Batch(Vec<ResponseResult>),
}

#[derive(Debug)]
pub enum ResponseResult {
    Valid {
        id: RequestId,
        result: DuperValue<'static>,
    },
    Invalid {
        id: Option<RequestId>,
        error: Error,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    Custom { code: i64, message: String },
}

pub type Result<T> = std::result::Result<T, Error>;

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
    I: private::ServerPart<()> + Clone + Send + 'static,
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
                    compile_error!("No async runtime selected!");
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
    I: private::ServerPart<()> + Clone + Send + 'static,
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

mod private {
    use super::*;

    pub trait ServerPart<S> {
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
}

pub trait IntoService: private::ServerPart<()> + Clone + Send + Sized + 'static {
    fn into_service(self) -> ServerService<Self>
    where
        Self: Sized + private::ServerPart<()> + Clone + Send + 'static,
    {
        ServerService { inner: self }
    }

    fn handle(
        self,
        req: Request,
    ) -> Pin<Box<dyn Future<Output = std::result::Result<Option<Response>, Infallible>> + 'static>>
    where
        Self: Sized + private::ServerPart<()> + Clone + Send + 'static,
    {
        self.into_service().call(req)
    }
}

impl<T> IntoService for T where T: private::ServerPart<()> + Clone + Send + Sized + 'static {}

impl<S> private::ServerPart<S> for Server<S> {
    fn serve(
        &self,
        _state: S,
        _name: String,
        _params: DuperValue<'static>,
    ) -> Pin<Box<dyn Future<Output = Result<DuperValue<'static>>> + Send + 'static>> {
        Box::pin(async { Err(Error::MethodNotFound) })
    }
}

impl<S, H, N, R, T> private::ServerPart<S> for Method<H, R, T, N>
where
    H: Handler<S, R, T>,
    N: private::ServerPart<S>,
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

impl<S, S2, N> private::ServerPart<S2> for WithState<S, N>
where
    S: Clone + Send + 'static,
    N: private::ServerPart<S>,
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

trait HandlerArgs {
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self>
    where
        Self: Sized;
}

impl<T1> HandlerArgs for (T1,)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 1 => {
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1,))
            }
            other => {
                let t1: T1 = from_value(other).map_err(|_| Error::InvalidParams)?;
                Ok((t1,))
            }
        }
    }
}

impl<T1, T2> HandlerArgs for (T1, T2)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 2 => {
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2))
            }
            _ => Err(Error::InvalidParams),
        }
    }
}

impl<T1, T2, T3> HandlerArgs for (T1, T2, T3)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
    T3: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 3 => {
                let t3: T3 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2, t3))
            }
            _ => Err(Error::InvalidParams),
        }
    }
}

pub trait Handler<S, R, T>: Clone + Send + Sync + Sized + 'static {
    type Future: Future<Output = Result<R>> + Send + 'static;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future;
}

impl<F, Fut, S, R> Handler<S, R, ((), ())> for F
where
    F: Fn() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, _params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move { self().await })
    }
}

impl<F, Fut, S, R, T1> Handler<S, R, ((), (T1,))> for F
where
    F: Fn(T1) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1,) = HandlerArgs::get_handler_args(params)?;
            self(args.0).await
        })
    }
}

impl<F, Fut, S, R, T1, T2> Handler<S, R, ((), (T1, T2))> for F
where
    F: Fn(T1, T2) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2) = HandlerArgs::get_handler_args(params)?;
            self(args.0, args.1).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3> Handler<S, R, ((), (T1, T2, T3))> for F
where
    F: Fn(T1, T2, T3) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3) = HandlerArgs::get_handler_args(params)?;
            self(args.0, args.1, args.2).await
        })
    }
}

impl<F, Fut, S, R> Handler<S, R, (State<S>, ())> for F
where
    F: Fn(State<S>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, _params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move { self(State(state)).await })
    }
}

impl<F, Fut, S, R, T1> Handler<S, R, (State<S>, (T1,))> for F
where
    F: Fn(State<S>, T1) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1,) = HandlerArgs::get_handler_args(params)?;
            self(State(state), args.0).await
        })
    }
}

impl<F, Fut, S, R, T1, T2> Handler<S, R, (State<S>, (T1, T2))> for F
where
    F: Fn(State<S>, T1, T2) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2) = HandlerArgs::get_handler_args(params)?;
            self(State(state), args.0, args.1).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3> Handler<S, R, (State<S>, (T1, T2, T3))> for F
where
    F: Fn(State<S>, T1, T2, T3) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3) = HandlerArgs::get_handler_args(params)?;
            self(State(state), args.0, args.1, args.2).await
        })
    }
}

#[cfg(test)]
mod rpc_tests {
    use std::borrow::Cow;

    use duper::{DuperFloat, DuperKey, DuperObject};

    use crate::private::ServerPart;

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
