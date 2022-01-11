use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Item);
    TokenStream::from(bit::expand(input))
}

mod bit;
mod seq;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    TokenStream::from(syn::parse_macro_input!(input as seq::Seq).finish())
}
