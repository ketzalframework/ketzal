pub mod registry;

#[cfg(feature = "web")]
pub use registry::get_web_router;

#[cfg(feature = "api")]
pub use registry::get_api_router;
