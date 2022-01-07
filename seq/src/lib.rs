#![allow(unused)]

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Literal, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{parse::Parse, LitInt, Token};

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let seq = syn::parse_macro_input!(input as Seq);
    TokenStream::from(seq.expan())
}

struct Seq {
    ident:       Ident,
    in_token:    Token![in],
    lhs:         LitInt,
    dot2_token:  Token![..],
    rhs:         LitInt,
    brace_token: syn::token::Brace,
    tokens:      TokenStream2,
}

impl Parse for Seq {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Seq { ident:       input.parse()?,
                 in_token:    input.parse()?,
                 lhs:         input.parse()?,
                 dot2_token:  input.parse()?,
                 rhs:         input.parse()?,
                 brace_token: syn::braced!(content in input),
                 tokens:      content.parse()?, })
    }
}

impl Seq {
    fn expan(self) -> TokenStream2 {
        let Seq { ident, lhs, rhs, tokens, .. } = self;
        let ts = tokens.into_iter();
        let range = lhs.base10_parse::<usize>().unwrap()..rhs.base10_parse::<usize>().unwrap();
        let tokens: Vec<TokenStream2> =
            range.map(|lit| ts.clone().map(|t| replace(t, &ident, lit)).collect()).collect();
        quote! { #(#tokens)* }
    }
}

// 将所有 ident 替换成字面值
fn replace(token: TokenTree, ident: &Ident, lit: usize) -> TokenTree {
    match token {
        TokenTree::Ident(ref i) if i == ident => Literal::usize_unsuffixed(lit).into(),
        TokenTree::Group(g) => {
            let mut group =
                Group::new(g.delimiter(), g.stream().into_iter().map(|t| replace(t, ident, lit)).collect());
            group.set_span(g.span());
            group.into()
        }
        t => t,
    }
}
