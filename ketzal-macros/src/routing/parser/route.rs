use proc_macro::TokenStream;
use syn::LitStr;

pub struct ParsedRoute {
    pub method: String,
    pub scope: String,
    pub path: String,
}

pub fn parse_route(
    method: &str,
    scope: &str,
    attr: TokenStream,
) -> Result<ParsedRoute, syn::Error> {
    let path: LitStr = syn::parse(attr)?;
    Ok(ParsedRoute { method: method.to_string(), scope: scope.to_string(), path: path.value() })
}
