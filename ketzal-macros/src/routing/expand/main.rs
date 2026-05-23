use crate::routing::parser::main::ParsedMain;
use proc_macro::TokenStream;
use quote::quote;

pub fn expand_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed = match crate::routing::parser::main::parse_main(_attr, item) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    expand_main_impl(parsed)
}

fn expand_main_impl(parsed: ParsedMain) -> TokenStream {
    let ParsedMain { signature } = parsed;

    let body = &signature.block;
    let vis = &signature.vis;

    let ret = match &signature.sig.output {
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
