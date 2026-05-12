use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitStr};

pub fn expand_route(
    method: &str,
    scope: &str,
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let function = parse_macro_input!(item as ItemFn);
    let fn_name = function.sig.ident.clone();

    let factory_name = format_ident!("__ketzal_{}_factory", fn_name);

    let scope_tokens = match scope {
        "Api" => quote! { ketzal_router::RouteScope::Api },
        _ => quote! { ketzal_router::RouteScope::Web },
    };

    let expanded = quote! {
        #function

        #[doc(hidden)]
        fn #factory_name() -> Box<dyn ketzal_router::BoxedHandler> {
            ketzal_router::into_boxed(#fn_name)
        }

        inventory::submit! {
            ketzal_router::RouteDefinition {
                method: #method,
                path: #path,
                scope: #scope_tokens,
                handler: #factory_name,
            }
        }
    };

    expanded.into()
}

pub fn expand_api_route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_route(method, "Api", attr, item)
}
