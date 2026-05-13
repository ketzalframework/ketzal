use crate::handler::HandlerFuture;
use crate::params::{match_path, Params};
use crate::route::Route;
use crate::route_definition::{RouteDefinition, RouteScope};
use http::Method;
use ketzal_http::Request;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_inventory(scope: RouteScope) -> Self {
        let mut router = Self::new();

        for route in inventory::iter::<RouteDefinition> {
            if route.scope != scope {
                continue;
            }

            router.register(Route {
                method: route.method.parse().unwrap(),
                path: route.path.to_string(),
                handler: Arc::from((route.handler)()),
                name: None,
            });
        }

        router
    }

    pub fn register(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn handle(&self, method: &Method, path: &str, req: Request) -> Option<HandlerFuture> {
        for route in &self.routes {
            if route.method != method {
                continue;
            }

            if route.path == path {
                return Some(route.call(&Params::new(), Some(req)));
            }

            if let Some(params) = match_path(&route.path, path) {
                return Some(route.call(&params, Some(req)));
            }
        }

        None
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}
