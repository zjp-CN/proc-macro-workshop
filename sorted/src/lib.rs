use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn sorted(arg: TokenStream, input: TokenStream) -> TokenStream {
    use quote::ToTokens;
    use syn::{parse_macro_input, spanned::Spanned, Error, Item};

    let arg = TokenStream2::from(arg);
    let input = parse_macro_input!(input as Item);

    if matches!(&input, Item::Enum(_)) {
        TokenStream::from(input.to_token_stream())
    } else {
        TokenStream::from(Error::new(arg.span(), "expected enum or match expression").to_compile_error())
    }
}
