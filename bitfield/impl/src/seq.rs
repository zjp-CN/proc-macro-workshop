use proc_macro2::{TokenStream as TokenStream2, TokenTree as TT};
use syn::{buffer::Cursor, Ident, LitInt, Token};

#[allow(dead_code)]
pub struct Seq {
    ident:       Ident,
    in_token:    Token![in],
    lhs:         LitInt,
    dot2_token:  Token![..],
    eq_token:    Token![=],
    rhs:         LitInt,
    brace_token: syn::token::Brace,
    pub tokens:  TokenStream2,
}

impl syn::parse::Parse for Seq {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let in_token = input.parse()?;
        let lhs: LitInt = input.parse()?;
        let dot2_token = input.parse()?;
        let eq_token = input.parse()?;
        let rhs: LitInt = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);

        let range = match (lhs.base10_parse(), rhs.base10_parse()) {
            (Ok(l), Ok(r)) => l..=r,
            _ => unreachable!("请输入 u8 范围"),
        };

        let cursor = content.cursor();
        let tokens = range.map(|lit| process(cursor, &ident, lit)).collect();

        Ok(Seq { ident,
                 in_token,
                 lhs,
                 dot2_token,
                 eq_token,
                 rhs,
                 brace_token,
                 tokens })
    }
}

fn process(mut cursor: Cursor, ident: &Ident, lit: u8) -> TokenStream2 {
    let mut ts: Vec<TT> = Vec::with_capacity(8);

    while let Some((token, cur)) = cursor.token_tree() {
        cursor = cur;
        match token {
            TT::Ident(i) => {
                if &i == ident {
                    ts.push(proc_macro2::Literal::u8_unsuffixed(lit).into());
                } else if let Some(c) = search_tidle_lit(cur, ident) {
                    cursor = c;
                    ts.push(quote::format_ident!("{}{}", i, lit).into());
                } else {
                    ts.push(i.into())
                }
            }
            TT::Group(ref g) => {
                let buf = syn::buffer::TokenBuffer::new2(g.stream());
                let mut group = proc_macro2::Group::new(g.delimiter(), process(buf.begin(), ident, lit));
                group.set_span(g.span());
                ts.push(group.into());
            }
            t => ts.push(t),
        }
    }

    dbg!(ts.into_iter().collect())
}

fn search_tidle_lit<'c>(cur: Cursor<'c>, ident: &Ident) -> Option<Cursor<'c>> {
    cur.token_tree().and_then(|(token, c)| match token {
                        TT::Punct(p) if p.as_char() == '~' => {
                            c.ident().and_then(|(i, c)| if &i == ident { Some(c) } else { None })
                        }
                        _ => None,
                    })
}
