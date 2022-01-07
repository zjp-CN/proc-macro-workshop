#![allow(unused)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse::Parse, Ident, Lit, Token};

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as Seq);

    TokenStream::new()
}

struct Seq {
    ident:      Ident,
    in_token:   Token![in],
    lhs:        Lit,
    dot2_token: Token![..],
    rhs:        Lit,
    tokens:     TokenStream2,
}

impl Parse for Seq {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Seq { ident:      input.parse()?,
                 in_token:   input.parse()?,
                 lhs:        input.parse()?,
                 dot2_token: input.parse()?,
                 rhs:        input.parse()?,
                 tokens:     input.parse()?, })
    }
}
