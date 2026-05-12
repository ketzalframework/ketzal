mod controller_macro;
mod route_macro;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use controller_macro::{expand_api_controller, expand_controller};
use route_macro::{expand_api_route, expand_route};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let body = &input.block;
    let vis = &input.vis;

    // Extraer el return type
    let ret = match &input.sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    quote! {
        #vis fn main() -> #ret {
            ::ketzal::tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    #body
                })
        }
    }
    .into()
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
