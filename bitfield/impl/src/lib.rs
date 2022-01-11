use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}

mod seq;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as seq::Seq);
    dbg!(input.tokens).into()
}
