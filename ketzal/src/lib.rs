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
pub use ketzal_router_macros::{controller, delete, get, patch, post, put};

#[cfg(feature = "api")]
pub use ketzal_router_macros::{api_controller, api_delete, api_get, api_patch, api_post, api_put};

pub use ketzal_router_macros::main;

// ── Validation macros ─────────────────────────────────────────────────────────

#[macro_export]
macro_rules! form_request {
    (
        $name:ident {
            rules: {
                $($field:expr => $rule:expr),* $(,)?
            }
            $(, messages: {
                $($msg_key:expr => $msg_val:expr),* $(,)?
            })?
            $(, attributes: {
                $($attr_key:expr => $attr_val:expr),* $(,)?
            })?
        }
    ) => {
        #[derive(Default)]
        pub struct $name;

        impl ketzal_validation::FormRequest for $name {
            fn rules(&self) -> std::collections::HashMap<&'static str, &'static str> {
                let mut map = std::collections::HashMap::new();
                $(map.insert($field, $rule);)*
                map
            }

            $(
                fn messages(&self) -> std::collections::HashMap<&'static str, &'static str> {
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($msg_key, $msg_val);)*
                    map
                }
            )?

            $(
                fn attributes(&self) -> std::collections::HashMap<&'static str, &'static str> {
                    let mut map = std::collections::HashMap::new();
                    $(map.insert($attr_key, $attr_val);)*
                    map
                }
            )?
        }
    };
}

/// Validates an `application/json` request body.
#[macro_export]
macro_rules! validate_json {
    ($req:expr => {
        $($field:literal => $rule:literal),* $(,)?
    }) => {{
        let __req = &$req;
        match __req.validate_json([$(($field, $rule),)*]) {
            ::std::ops::ControlFlow::Continue(val) => val,
            ::std::ops::ControlFlow::Break(resp)   => return resp,
        }
    }};
}

/// Validates an `application/x-www-form-urlencoded` request body.
#[macro_export]
macro_rules! validate_form {
    ($req:expr => {
        $($field:literal => $rule:literal),* $(,)?
    }) => {{
        let __req = &$req;
        match __req.validate_form([$(($field, $rule),)*]) {
            ::std::ops::ControlFlow::Continue(val) => val,
            ::std::ops::ControlFlow::Break(resp)   => return resp,
        }
    }};
}
