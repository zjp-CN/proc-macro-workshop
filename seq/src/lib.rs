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
        let buffer = syn::buffer::TokenBuffer::new2(tokens);
        let cursor = buffer.begin();
        let range = lhs.base10_parse::<usize>().unwrap()..rhs.base10_parse::<usize>().unwrap();
        repeat::SeqToken::new(cursor, &ident, range).token_stream()
    }
}

mod repeat;
mod replace;

// 把 Group 内的 TokenStream 替换掉（保留 delimiter 和 span）
fn new_group(g: &Group, ts: TokenStream2) -> Group {
    let mut group = Group::new(g.delimiter(), ts);
    group.set_span(g.span());
    group
}
