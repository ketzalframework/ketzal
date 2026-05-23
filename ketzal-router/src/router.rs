use crate::handler::HandlerFuture;
use crate::params::{match_path, Params};
use crate::route::Route;
use crate::route_definition::{RouteDefinition, RouteScope};

use http::{HeaderValue, Method};

use ketzal_http::{Request, Response};

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
        if *method == Method::HEAD {
            return self.handle_head(path, &req);
        }

        if *method == Method::OPTIONS {
            return self.handle_options(path);
        }

        for route in &self.routes {
            if route.method != *method {
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

    fn handle_head(&self, path: &str, req: &Request) -> Option<HandlerFuture> {
        for route in &self.routes {
            if route.method != Method::GET {
                continue;
            }

            if route.path == path {
                return Some(self.strip_body(route.call(&Params::new(), Some(req.clone()))));
            }

            if let Some(params) = match_path(&route.path, path) {
                return Some(self.strip_body(route.call(&params, Some(req.clone()))));
            }
        }

        None
    }

    fn handle_options(&self, path: &str) -> Option<HandlerFuture> {
        let mut methods: Vec<String> = Vec::new();

        for route in &self.routes {
            let matches = route.path == path || match_path(&route.path, path).is_some();

            if !matches {
                continue;
            }

            let method = route.method.as_str().to_string();

            if !methods.contains(&method) {
                methods.push(method);
            }

            if route.method == Method::GET {
                let head = "HEAD".to_string();

                if !methods.contains(&head) {
                    methods.push(head);
                }
            }
        }

        if methods.is_empty() {
            return None;
        }

        let options = "OPTIONS".to_string();

        if !methods.contains(&options) {
            methods.push(options);
        }

        methods.sort();

        let allow_header = methods.join(", ");

        Some(Box::pin(async move {
            let mut response = Response::new(http::StatusCode::OK);

            response
                .headers
                .insert(http::header::ALLOW, HeaderValue::from_str(&allow_header).unwrap());

            response
                .headers
                .insert(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));

            response.headers.insert(
                http::header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_str(&allow_header).unwrap(),
            );

            response
                .headers
                .insert(http::header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));

            response
        }))
    }

    fn strip_body(&self, future: HandlerFuture) -> HandlerFuture {
        Box::pin(async move {
            let mut response = future.await;

            // HEAD no debe retornar body
            response.body.clear();

            response
        })
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}
