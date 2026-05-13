use crate::handler::BoxedHandler;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RouteScope {
    Web,
    Api,
}

pub struct RouteDefinition {
    pub method: &'static str,
    pub path: &'static str,
    pub scope: RouteScope,
    pub handler: fn() -> Box<dyn BoxedHandler>,
}

inventory::collect!(RouteDefinition);
