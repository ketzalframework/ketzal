use proc_macro::TokenStream;
use quote::quote;

pub fn expand_controller(scope: &'static str, attr: TokenStream, item: TokenStream) -> TokenStream {
    match crate::routing::parser::controller::parse_controller(attr, item, scope) {
        Ok(parsed) => expand_controller_impl(parsed),
        Err(e) => e.to_compile_error().into(),
    }
}

pub fn expand_api_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_controller("Api", attr, item)
}

fn expand_controller_impl(
    parsed: crate::routing::parser::controller::ParsedController,
) -> TokenStream {
    use crate::routing::registration::controller::register_controller_routes;

    let registrations = register_controller_routes(&parsed.self_ty, &parsed.routes, parsed.scope);

    let impl_block = parsed.impl_block;
    let self_ty = parsed.self_ty;

    quote! {
        pub struct #self_ty;

        #impl_block

        #registrations
    }
    .into()
}
