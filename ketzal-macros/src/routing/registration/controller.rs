use crate::routing::parser::RouteMethodInfo;
use quote::quote;

pub fn register_controller_routes(
    self_ty: &syn::Type,
    routes: &[RouteMethodInfo],
    scope: &str,
) -> proc_macro2::TokenStream {
    let mut registrations = proc_macro2::TokenStream::new();

    for route in routes {
        let RouteMethodInfo { http_method, full_path, fn_name } = route;

        let factory = quote::format_ident!("__ketzal_{}_factory", fn_name);

        let scope_tokens = match scope {
            "Api" => quote! { ::ketzal::RouteScope::Api },
            _ => quote! { ::ketzal::RouteScope::Web },
        };

        let reg = quote! {
            #[doc(hidden)]
            fn #factory() -> Box<dyn ::ketzal::BoxedHandler> {
                ::ketzal::into_boxed(#self_ty::#fn_name)
            }

            ::ketzal::inventory::submit! {
                ::ketzal::RouteDefinition {
                    method:  #http_method,
                    path:    #full_path,
                    scope:   #scope_tokens,
                    handler: #factory,
                }
            }
        };

        registrations.extend(reg);
    }

    registrations
}
