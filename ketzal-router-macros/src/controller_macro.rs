use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ImplItem, Item, ItemImpl, LitStr};

use crate::utils::normalize_path;

pub fn expand_controller(scope: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_path = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as Item);

    match input {
        Item::Impl(mut imp) => {
            let self_ty: syn::Type = (*imp.self_ty).clone();

            let registrations =
                extract_registrations(&mut imp, &base_path.value(), scope, &self_ty);

            quote! {
                // Genera el struct automáticamente
                pub struct #self_ty;

                // impl limpio, solo métodos
                #imp

                // Funciones factory e inventory::submit! al nivel del módulo
                #(#registrations)*
            }
        }

        _ => syn::Error::new_spanned(input, "controller expects an impl block")
            .to_compile_error(),
    }
    .into()
}

pub fn expand_api_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_controller("Api", attr, item)
}

fn extract_registrations(
    imp: &mut ItemImpl,
    base: &str,
    scope: &str,
    self_ty: &syn::Type,
) -> Vec<proc_macro2::TokenStream> {
    let mut registrations = Vec::new();

    for item in imp.items.iter_mut() {
        let ImplItem::Fn(method) = item else {
            continue;
        };

        let fn_name = method.sig.ident.clone();

        let mut http_method: Option<String> = None;
        let mut full_path: Option<String> = None;

        // Eliminar el atributo de ruta y capturar sus datos
        method.attrs.retain(|attr| {
            let Some(ident) = attr.path().get_ident() else {
                return true;
            };

            let name = ident.to_string();

            if !matches!(name.as_str(), "get" | "post" | "put" | "delete" | "patch") {
                return true;
            }

            if let Ok(lit) = attr.parse_args::<LitStr>() {
                http_method = Some(name.to_uppercase());
                full_path = Some(normalize_path(base, &lit.value()));
            }

            false // eliminar el atributo
        });

        let Some(method_str) = http_method else {
            continue;
        };

        let path_str = full_path.unwrap();
        let factory = format_ident!("__ketzal_{}_factory", fn_name);

        let scope_tokens = match scope {
            "Api" => quote! { ::ketzal::RouteScope::Api },
            _ => quote! { ::ketzal::RouteScope::Web },
        };

        registrations.push(quote! {
            // Función factory al nivel del módulo, llama al método del impl
            #[doc(hidden)]
            fn #factory() -> Box<dyn ::ketzal::BoxedHandler> {
                ::ketzal::into_boxed(#self_ty::#fn_name)
            }

            ::ketzal::inventory::submit! {
                ::ketzal::RouteDefinition {
                    method:  #method_str,
                    path:    #path_str,
                    scope:   #scope_tokens,
                    handler: #factory,
                }
            }
        });
    }

    registrations
}
