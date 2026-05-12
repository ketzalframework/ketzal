// ketzal-router/src/route.rs

use crate::handler::{into_boxed, BoxedHandler, Handler, HandlerFuture};
use crate::params::Params;
use http::Method;
use ketzal_http::Request;
use std::sync::Arc;

#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Arc<dyn BoxedHandler>,
    pub name: Option<String>,
}

impl Route {
    pub fn new<M: 'static>(method: Method, path: &str, handler: impl Handler<M>) -> Self {
        Self { method, path: path.to_string(), handler: Arc::from(into_boxed(handler)), name: None }
    }

    pub fn get<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::GET, path, handler)
    }

    pub fn post<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::POST, path, handler)
    }

    pub fn put<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::PUT, path, handler)
    }

    pub fn delete<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::DELETE, path, handler)
    }

    pub fn patch<M: 'static>(path: &str, handler: impl Handler<M>) -> Self {
        Self::new(Method::PATCH, path, handler)
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn call(&self, params: &Params, req: Option<Request>) -> HandlerFuture {
        self.handler.call(params, req)
    }
}
