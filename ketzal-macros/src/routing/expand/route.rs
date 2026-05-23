use crate::routing::parser::route::ParsedRoute;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn expand_route(
    method: &str,
    scope: &str,
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let parsed = match crate::routing::parser::route::parse_route(method, scope, attr) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    expand_route_impl(&parsed, item)
}

pub fn expand_api_route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route(method, "Api", attr, item)
}

fn expand_route_impl(parsed: &ParsedRoute, item: TokenStream) -> TokenStream {
    use crate::routing::registration::route::register_route;

    let function: syn::ItemFn = syn::parse(item).unwrap();
    let fn_name = function.sig.ident.clone();

    let factory_name = format_ident!("__ketzal_{}_factory", fn_name);

    let function_tokens = quote! { #function };

    let factory_tokens = quote! {
        #[doc(hidden)]
        fn #factory_name() -> Box<dyn ketzal_router::BoxedHandler> {
            ketzal_router::into_boxed(#fn_name)
        }
    };

    let registration_tokens =
        register_route(&parsed.method, &parsed.path, &parsed.scope, &factory_name);

    quote! {
        #function_tokens
        #factory_tokens
        #registration_tokens
    }
    .into()
}
