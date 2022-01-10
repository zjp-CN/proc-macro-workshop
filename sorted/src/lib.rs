use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Item);
    TokenStream::from(_enum::process(&input))
}

// workaround for procedural macro invocations (like `#[sorted]`) on expressions
// see test/05-match-expr.rs
#[proc_macro_attribute]
pub fn check(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);
    TokenStream::from(_fn::process(&mut input))
}

mod _enum;
mod _fn;
