use proc_macro::TokenStream;
use syn::ItemFn;

pub struct ParsedMain {
    pub signature: ItemFn,
}

pub fn parse_main(_attr: TokenStream, item: TokenStream) -> Result<ParsedMain, syn::Error> {
    let input: ItemFn = syn::parse(item)?;
    Ok(ParsedMain { signature: input })
}
