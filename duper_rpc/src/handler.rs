// TO-DO: Use declarative macros to avoid code repetition.

use std::pin::Pin;

use duper::{DuperValue, serde::de::from_value};
use serde_core::{Deserialize, Serialize};

use crate::{Error, Result, server::State};

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

impl<T1, T2, T3, T4> HandlerArgs for (T1, T2, T3, T4)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
    T3: for<'de> Deserialize<'de> + Send + 'static,
    T4: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 4 => {
                let t4: T4 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t3: T3 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2, t3, t4))
            }
            _ => Err(Error::InvalidParams),
        }
    }
}

impl<T1, T2, T3, T4, T5> HandlerArgs for (T1, T2, T3, T4, T5)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
    T3: for<'de> Deserialize<'de> + Send + 'static,
    T4: for<'de> Deserialize<'de> + Send + 'static,
    T5: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 5 => {
                let t5: T5 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t4: T4 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t3: T3 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2, t3, t4, t5))
            }
            _ => Err(Error::InvalidParams),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6> HandlerArgs for (T1, T2, T3, T4, T5, T6)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
    T3: for<'de> Deserialize<'de> + Send + 'static,
    T4: for<'de> Deserialize<'de> + Send + 'static,
    T5: for<'de> Deserialize<'de> + Send + 'static,
    T6: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 6 => {
                let t6: T6 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t5: T5 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t4: T4 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t3: T3 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2, t3, t4, t5, t6))
            }
            _ => Err(Error::InvalidParams),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> HandlerArgs for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
    T3: for<'de> Deserialize<'de> + Send + 'static,
    T4: for<'de> Deserialize<'de> + Send + 'static,
    T5: for<'de> Deserialize<'de> + Send + 'static,
    T6: for<'de> Deserialize<'de> + Send + 'static,
    T7: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 7 => {
                let t7: T7 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t6: T6 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t5: T5 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t4: T4 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t3: T3 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2, t3, t4, t5, t6, t7))
            }
            _ => Err(Error::InvalidParams),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> HandlerArgs for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: for<'de> Deserialize<'de> + Send + 'static,
    T2: for<'de> Deserialize<'de> + Send + 'static,
    T3: for<'de> Deserialize<'de> + Send + 'static,
    T4: for<'de> Deserialize<'de> + Send + 'static,
    T5: for<'de> Deserialize<'de> + Send + 'static,
    T6: for<'de> Deserialize<'de> + Send + 'static,
    T7: for<'de> Deserialize<'de> + Send + 'static,
    T8: for<'de> Deserialize<'de> + Send + 'static,
{
    fn get_handler_args(params: DuperValue<'static>) -> Result<Self> {
        match params {
            DuperValue::Tuple { mut inner, .. } if inner.len() == 8 => {
                let t8: T8 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t7: T7 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t6: T6 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t5: T5 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t4: T4 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t3: T3 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t2: T2 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                let t1: T1 = from_value(inner.pop().expect("length checked"))
                    .map_err(|_| Error::InvalidParams)?;
                Ok((t1, t2, t3, t4, t5, t6, t7, t8))
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

impl<F, Fut, S, R, T1, T2, T3, T4> Handler<S, R, ((), (T1, T2, T3, T4))> for F
where
    F: Fn(T1, T2, T3, T4) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4) = HandlerArgs::get_handler_args(params)?;
            self(args.0, args.1, args.2, args.3).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3, T4, T5> Handler<S, R, ((), (T1, T2, T3, T4, T5))> for F
where
    F: Fn(T1, T2, T3, T4, T5) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5) = HandlerArgs::get_handler_args(params)?;
            self(args.0, args.1, args.2, args.3, args.4).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3, T4, T5, T6> Handler<S, R, ((), (T1, T2, T3, T4, T5, T6))> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T6: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5, T6) = HandlerArgs::get_handler_args(params)?;
            self(args.0, args.1, args.2, args.3, args.4, args.5).await
        })
    }
}
impl<F, Fut, S, R, T1, T2, T3, T4, T5, T6, T7> Handler<S, R, ((), (T1, T2, T3, T4, T5, T6, T7))>
    for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T6: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T7: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5, T6, T7) = HandlerArgs::get_handler_args(params)?;
            self(args.0, args.1, args.2, args.3, args.4, args.5, args.6).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3, T4, T5, T6, T7, T8>
    Handler<S, R, ((), (T1, T2, T3, T4, T5, T6, T7, T8))> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7, T8) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T6: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T7: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T8: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, _state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5, T6, T7, T8) = HandlerArgs::get_handler_args(params)?;
            self(
                args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7,
            )
            .await
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

impl<F, Fut, S, R, T1, T2, T3, T4> Handler<S, R, (State<S>, (T1, T2, T3, T4))> for F
where
    F: Fn(State<S>, T1, T2, T3, T4) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4) = HandlerArgs::get_handler_args(params)?;
            self(State(state), args.0, args.1, args.2, args.3).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3, T4, T5> Handler<S, R, (State<S>, (T1, T2, T3, T4, T5))> for F
where
    F: Fn(State<S>, T1, T2, T3, T4, T5) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5) = HandlerArgs::get_handler_args(params)?;
            self(State(state), args.0, args.1, args.2, args.3, args.4).await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3, T4, T5, T6> Handler<S, R, (State<S>, (T1, T2, T3, T4, T5, T6))> for F
where
    F: Fn(State<S>, T1, T2, T3, T4, T5, T6) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T6: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5, T6) = HandlerArgs::get_handler_args(params)?;
            self(State(state), args.0, args.1, args.2, args.3, args.4, args.5).await
        })
    }
}
impl<F, Fut, S, R, T1, T2, T3, T4, T5, T6, T7>
    Handler<S, R, (State<S>, (T1, T2, T3, T4, T5, T6, T7))> for F
where
    F: Fn(State<S>, T1, T2, T3, T4, T5, T6, T7) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T6: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T7: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5, T6, T7) = HandlerArgs::get_handler_args(params)?;
            self(
                State(state),
                args.0,
                args.1,
                args.2,
                args.3,
                args.4,
                args.5,
                args.6,
            )
            .await
        })
    }
}

impl<F, Fut, S, R, T1, T2, T3, T4, T5, T6, T7, T8>
    Handler<S, R, (State<S>, (T1, T2, T3, T4, T5, T6, T7, T8))> for F
where
    F: Fn(State<S>, T1, T2, T3, T4, T5, T6, T7, T8) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<R>> + Send,
    R: Serialize + Send + 'static,
    S: Send + Sync + 'static,
    T1: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T2: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T3: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T4: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T5: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T6: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T7: for<'de> Deserialize<'de> + Send + Sync + 'static,
    T8: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<R>> + Send>>;

    fn call(self, state: S, params: DuperValue<'static>) -> Self::Future {
        Box::pin(async move {
            let args: (T1, T2, T3, T4, T5, T6, T7, T8) = HandlerArgs::get_handler_args(params)?;
            self(
                State(state),
                args.0,
                args.1,
                args.2,
                args.3,
                args.4,
                args.5,
                args.6,
                args.7,
            )
            .await
        })
    }
}
