use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::params::Params;
use ketzal_http::{Request, Response};

pub type HandlerFuture = Pin<Box<dyn Future<Output = Response> + Send>>;

// ── FromParam ────────────────────────────────────────────────────────────────

pub trait FromParam: Sized {
    fn from_param(value: &str) -> Result<Self, Response>;
}

impl FromParam for i32 {
    fn from_param(v: &str) -> Result<Self, Response> {
        v.parse().map_err(|_| Response::bad_request("Invalid i32"))
    }
}

impl FromParam for i64 {
    fn from_param(v: &str) -> Result<Self, Response> {
        v.parse().map_err(|_| Response::bad_request("Invalid i64"))
    }
}

impl FromParam for u32 {
    fn from_param(v: &str) -> Result<Self, Response> {
        v.parse().map_err(|_| Response::bad_request("Invalid u32"))
    }
}

impl FromParam for u64 {
    fn from_param(v: &str) -> Result<Self, Response> {
        v.parse().map_err(|_| Response::bad_request("Invalid u64"))
    }
}

impl FromParam for String {
    fn from_param(v: &str) -> Result<Self, Response> {
        Ok(v.to_string())
    }
}

// ── FromParams ───────────────────────────────────────────────────────────────

pub trait FromParams: Sized {
    fn from_params(params: &Params) -> Result<Self, Response>;
}

impl FromParams for () {
    fn from_params(_: &Params) -> Result<Self, Response> {
        Ok(())
    }
}

impl<T: FromParam> FromParams for (T,) {
    fn from_params(params: &Params) -> Result<Self, Response> {
        let mut it = params.all().values();

        let t = T::from_param(
            it.next().ok_or_else(|| Response::bad_request("Missing param"))?,
        )?;

        Ok((t,))
    }
}

impl<A: FromParam, B: FromParam> FromParams for (A, B) {
    fn from_params(params: &Params) -> Result<Self, Response> {
        let mut it = params.all().values();

        let a = A::from_param(
            it.next().ok_or_else(|| Response::bad_request("Missing param"))?,
        )?;

        let b = B::from_param(
            it.next().ok_or_else(|| Response::bad_request("Missing param"))?,
        )?;

        Ok((a, b))
    }
}

impl<A: FromParam, B: FromParam, C: FromParam> FromParams for (A, B, C) {
    fn from_params(params: &Params) -> Result<Self, Response> {
        let mut it = params.all().values();

        let a = A::from_param(
            it.next().ok_or_else(|| Response::bad_request("Missing param"))?,
        )?;

        let b = B::from_param(
            it.next().ok_or_else(|| Response::bad_request("Missing param"))?,
        )?;

        let c = C::from_param(
            it.next().ok_or_else(|| Response::bad_request("Missing param"))?,
        )?;

        Ok((a, b, c))
    }
}

// ── Marker types ─────────────────────────────────────────────────────────────

pub struct Zero;
pub struct WithReq;
pub struct One<T>(PhantomData<T>);
pub struct WithReqOne<T>(PhantomData<T>);
pub struct Two<A, B>(PhantomData<(A, B)>);
pub struct WithReqTwo<A, B>(PhantomData<(A, B)>);

// ── Handler trait ─────────────────────────────────────────────────────────────

pub trait Handler<M>: Send + Sync + 'static {
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture;
}

// fn() -> Response
impl<F, Fut> Handler<Zero> for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    fn call(&self, _: &Params, _: Option<Request>) -> HandlerFuture {
        Box::pin(self())
    }
}

// fn(Request) -> Response
impl<F, Fut> Handler<WithReq> for F
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    fn call(&self, _: &Params, req: Option<Request>) -> HandlerFuture {
        Box::pin(self(req.expect("Request required")))
    }
}

// fn(T) -> Response
impl<F, Fut, T> Handler<One<T>> for F
where
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
{
    fn call(&self, params: &Params, _: Option<Request>) -> HandlerFuture {
        match <(T,)>::from_params(params) {
            Ok((t,)) => Box::pin(self(t)),
            Err(r)   => Box::pin(async move { r }),
        }
    }
}

// fn(Request, T) -> Response
impl<F, Fut, T> Handler<WithReqOne<T>> for F
where
    F: Fn(Request, T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    T: FromParam + 'static,
{
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        let req = req.expect("Request required");
        match <(T,)>::from_params(params) {
            Ok((t,)) => Box::pin(self(req, t)),
            Err(r)   => Box::pin(async move { r }),
        }
    }
}

// fn(A, B) -> Response
impl<F, Fut, A, B> Handler<Two<A, B>> for F
where
    F: Fn(A, B) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    A: FromParam + 'static,
    B: FromParam + 'static,
{
    fn call(&self, params: &Params, _: Option<Request>) -> HandlerFuture {
        match <(A, B)>::from_params(params) {
            Ok((a, b)) => Box::pin(self(a, b)),
            Err(r)     => Box::pin(async move { r }),
        }
    }
}

// fn(Request, A, B) -> Response
impl<F, Fut, A, B> Handler<WithReqTwo<A, B>> for F
where
    F: Fn(Request, A, B) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
    A: FromParam + 'static,
    B: FromParam + 'static,
{
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        let req = req.expect("Request required");
        match <(A, B)>::from_params(params) {
            Ok((a, b)) => Box::pin(self(req, a, b)),
            Err(r)     => Box::pin(async move { r }),
        }
    }
}

// ── BoxedHandler ──────────────────────────────────────────────────────────────

pub trait BoxedHandler: Send + Sync + 'static {
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture;
}

struct HandlerWrapper<F, M> {
    f:  F,
    _m: PhantomData<M>,
}

unsafe impl<F: Send, M> Send for HandlerWrapper<F, M> {}
unsafe impl<F: Sync, M> Sync for HandlerWrapper<F, M> {}

impl<F, M> BoxedHandler for HandlerWrapper<F, M>
where
    F: Handler<M> + 'static,
    M: 'static,
{
    fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        self.f.call(params, req)
    }
}

pub fn into_boxed<M: 'static>(f: impl Handler<M>) -> Box<dyn BoxedHandler> {
    Box::new(HandlerWrapper { f, _m: PhantomData })
}