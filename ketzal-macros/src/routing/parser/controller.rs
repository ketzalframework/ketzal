use crate::routing::utils::normalize_path;
use proc_macro::TokenStream;
use syn::{ImplItem, Item, ItemImpl, LitStr};

pub struct RouteMethodInfo {
    pub http_method: String,
    pub full_path: String,
    pub fn_name: syn::Ident,
}

pub struct ParsedController {
    pub scope: &'static str,
    pub self_ty: syn::Type,
    pub routes: Vec<RouteMethodInfo>,
    pub impl_block: ItemImpl,
}

pub fn parse_controller(
    attr: TokenStream,
    item: TokenStream,
    scope: &'static str,
) -> Result<ParsedController, syn::Error> {
    let base_path: LitStr = syn::parse(attr)?;
    let input: Item = syn::parse(item)?;

    let Item::Impl(mut imp) = input else {
        return Err(syn::Error::new_spanned(
            input,
            "attribute 'controller' can only be applied to an impl block",
        ));
    };

    let self_ty = (*imp.self_ty).clone();
    let routes = extract_route_methods(&imp, &base_path.value())?;

    strip_route_attributes(&mut imp);

    Ok(ParsedController { scope, self_ty: self_ty.clone(), routes, impl_block: imp })
}

fn extract_route_methods(imp: &ItemImpl, base: &str) -> Result<Vec<RouteMethodInfo>, syn::Error> {
    let mut routes = Vec::new();

    for item in &imp.items {
        let ImplItem::Fn(method) = item else {
            continue;
        };

        let fn_name = method.sig.ident.clone();

        for attr in &method.attrs {
            let Some(ident) = attr.path().get_ident() else {
                continue;
            };

            let name = ident.to_string();

            if !matches!(name.as_str(), "get" | "post" | "put" | "delete" | "patch") {
                continue;
            }

            let lit: LitStr = attr.parse_args()?;
            let http_method = name.to_uppercase();
            let full_path = normalize_path(base, &lit.value());

            routes.push(RouteMethodInfo { http_method, full_path, fn_name: fn_name.clone() });
        }
    }

    Ok(routes)
}

fn strip_route_attributes(imp: &mut ItemImpl) {
    // strip_routes: metodos que el router generó en inventory (se incluyen por
    // defecto: HEAD replica GET, OPTIONS siempre existe) no deben permanecer en
    // el impl re-emitido porque el usuario no los escribe ni los importa.
    const STRIP_METHODS: [&str; 7] = ["get", "post", "put", "delete", "patch", "head", "options"];

    for item in &mut imp.items {
        let ImplItem::Fn(method) = item else {
            continue;
        };

        method.attrs.retain(|attr| {
            let Some(ident) = attr.path().get_ident() else {
                return true;
            };

            !STRIP_METHODS.contains(&ident.to_string().as_str())
        });
    }
}
