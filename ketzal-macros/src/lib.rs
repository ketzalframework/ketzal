mod routing;
use proc_macro::TokenStream;

use routing::{
    expand_api_controller, expand_api_route, expand_controller, expand_main, expand_route,
};

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_main(attr, item)
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route("GET", "Web", attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route("POST", "Web", attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route("PUT", "Web", attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route("DELETE", "Web", attr, item)
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route("PATCH", "Web", attr, item)
}

#[proc_macro_attribute]
pub fn api_get(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_route("GET", attr, item)
}

#[proc_macro_attribute]
pub fn api_post(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_route("POST", attr, item)
}

#[proc_macro_attribute]
pub fn api_put(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_route("PUT", attr, item)
}

#[proc_macro_attribute]
pub fn api_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_route("DELETE", attr, item)
}

#[proc_macro_attribute]
pub fn api_patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_route("PATCH", attr, item)
}

#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_controller("Web", attr, item)
}

#[proc_macro_attribute]
pub fn api_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_controller(attr, item)
}
