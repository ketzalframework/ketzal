use proc_macro2::TokenStream;
use quote::quote;

pub fn register_route(
    method: &str,
    path: &str,
    scope: &str,
    factory_name: &syn::Ident,
) -> TokenStream {
    let scope_tokens = match scope {
        "Api" => quote! { ketzal_router::RouteScope::Api },
        _ => quote! { ketzal_router::RouteScope::Web },
    };

    quote! {
        inventory::submit! {
            ketzal_router::RouteDefinition {
                method: #method,
                path: #path,
                scope: #scope_tokens,
                handler: #factory_name,
            }
        }
    }
}
