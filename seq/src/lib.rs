use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Span, TokenStream as TokenStream2};
use syn::{parse::Parse, Error, LitInt, Token};

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let seq = syn::parse_macro_input!(input as Seq);
    TokenStream::from(seq.expand())
}

#[allow(dead_code)]
struct Seq {
    ident:       Ident,
    in_token:    Token![in],
    lhs:         LitInt,
    dot2_token:  Token![..],
    eq_token:    Option<Token![=]>,
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
                 eq_token:    input.parse().ok(),
                 rhs:         input.parse()?,
                 brace_token: syn::braced!(content in input),
                 tokens:      content.parse()?, })
    }
}

impl Seq {
    fn expand(self) -> TokenStream2 {
        let Seq { ident, lhs, rhs, tokens, eq_token, .. } = self;
        let buffer = syn::buffer::TokenBuffer::new2(tokens);
        let cursor = buffer.begin();
        match (lhs.base10_parse(), rhs.base10_parse()) {
            (Ok(lhs), Ok(rhs)) => {
                let range = if eq_token.is_some() { (lhs..=rhs).into() } else { (lhs..rhs).into() };
                repeat::SeqToken::new(cursor, &ident, range).token_stream()
            }
            (Err(err), _) => lit_err(err, lhs.span()),
            (Ok(_), Err(err)) => lit_err(err, rhs.span()),
        }
    }
}

mod range;
mod repeat;
mod replace;

// 把 Group 内的 TokenStream 替换掉（保留 delimiter 和 span）
fn new_group(g: &Group, ts: TokenStream2) -> Group {
    let mut group = Group::new(g.delimiter(), ts);
    group.set_span(g.span());
    group
}

fn lit_err(err: Error, span: Span) -> TokenStream2 {
    let err = format!("`{}`\nOnly support `usize` type for now!", err);
    Error::new(span, err).to_compile_error()
}
