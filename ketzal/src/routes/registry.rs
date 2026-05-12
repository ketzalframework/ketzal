use ketzal_router::{RouteScope, Router};
use std::sync::{Arc, OnceLock};

#[cfg(feature = "web")]
static WEB_ROUTER: OnceLock<Arc<Router>> = OnceLock::new();

#[cfg(feature = "api")]
static API_ROUTER: OnceLock<Arc<Router>> = OnceLock::new();

#[cfg(feature = "web")]
pub fn get_web_router() -> Arc<Router> {
    WEB_ROUTER.get_or_init(|| Arc::new(Router::from_inventory(RouteScope::Web))).clone()
}

#[cfg(feature = "api")]
pub fn get_api_router() -> Arc<Router> {
    API_ROUTER.get_or_init(|| Arc::new(Router::from_inventory(RouteScope::Api))).clone()
}
