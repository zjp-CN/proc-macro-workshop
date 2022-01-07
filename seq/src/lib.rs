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
        use syn::buffer::TokenBuffer;
        let Seq { ident, lhs, rhs, tokens, .. } = self;
        let buffer = TokenBuffer::new2(tokens);
        let cursor = buffer.begin();
        let range = lhs.base10_parse::<usize>().unwrap()..rhs.base10_parse::<usize>().unwrap();
        let tokens: Vec<TokenStream2> = range.map(|lit| replace(cursor, &ident, lit)).collect();
        quote! { #(#tokens)* }
    }
}

// 将所有 ident 替换成字面值、`prefix~ident` 替换成 `prefix字面值`
fn replace(mut cursor: syn::buffer::Cursor, ident: &Ident, lit: usize) -> TokenStream2 {
    fn reset(prefix: &mut Option<Ident>, tidle: &mut bool) {
        *prefix = None;
        *tidle = false;
    }
    let (mut ts, mut prefix, mut tidle) = (Vec::with_capacity(128), None, false);
    while let Some((token, cur)) = cursor.token_tree() {
        cursor = cur;
        match token {
            TokenTree::Ident(i) => {
                let matched = &i == ident;
                if tidle && matched {
                    if let (Some(id), [.., last]) = (prefix.take(), &mut ts[..]) {
                        *last = quote::format_ident!("{}{}", id, lit).into();
                    } else {
                        ts.push(i.into());
                    }
                } else if matched {
                    prefix = None;
                    ts.push(Literal::usize_unsuffixed(lit).into());
                } else {
                    prefix = Some(i.clone());
                    ts.push(i.into());
                }
                tidle = false;
            }
            TokenTree::Group(g) => {
                reset(&mut prefix, &mut tidle);
                let buf = syn::buffer::TokenBuffer::new2(g.stream());
                let mut group = Group::new(g.delimiter(), replace(buf.begin(), ident, lit));
                group.set_span(g.span());
                ts.push(group.into());
            }
            TokenTree::Punct(p) if p.as_char() == '~' => {
                tidle = true;
            }
            t => {
                reset(&mut prefix, &mut tidle);
                ts.push(t);
            }
        }
    }
    TokenStream2::from_iter(ts)
}
