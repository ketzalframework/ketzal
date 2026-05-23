pub mod config;
pub mod routes;
pub mod server;

// ── External re-exports ───────────────────────────────────────────────────────

pub use ctor;
pub use inventory;
pub use tokio;

// ── ketzal-http re-exports ────────────────────────────────────────────────────

pub use ketzal_http::{HTTPException, Request, Response};

// ── ketzal-router re-exports ──────────────────────────────────────────────────

pub use ketzal_router::handler::{into_boxed, BoxedHandler};
pub use ketzal_router::route_definition::{RouteDefinition, RouteScope};

// ── Internal re-exports ───────────────────────────────────────────────────────

pub use config::Bootstrap;

// ── Route macros ──────────────────────────────────────────────────────────────

#[cfg(feature = "web")]
pub use ketzal_macros::{controller, delete, get, patch, post, put};

#[cfg(feature = "api")]
pub use ketzal_macros::{api_controller, api_delete, api_get, api_patch, api_post, api_put};

pub use ketzal_macros::main;
