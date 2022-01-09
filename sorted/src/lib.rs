use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(_: TokenStream, input: TokenStream) -> TokenStream {
    use quote::ToTokens;
    let input = syn::parse_macro_input!(input as syn::Item);
    TokenStream::from(input.to_token_stream())
}
