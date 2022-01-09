use crate::range::Range;
use proc_macro2::{Group, Ident, TokenStream as TokenStream2, TokenTree as TT};
use syn::buffer::{Cursor, TokenBuffer};

// 区分是否需要重复
pub struct SeqToken<'c, 'i> {
    output: Vec<TokenStream2>,
    cursor: Cursor<'c>,
    range:  Range,
    ident:  &'i Ident,
}

impl<'c, 'i> SeqToken<'c, 'i> {
    pub fn new(cursor: Cursor<'c>, ident: &'i Ident, range: Range) -> Self {
        SeqToken { output: Vec::with_capacity(32),
                   cursor,
                   range,
                   ident }
    }

    // 如果存在 `#()*`，则不重复整个块
    pub fn token_stream(mut self) -> TokenStream2 {
        if self.is_repeat() {
            self.search_and_replace();
        } else {
            self.repeat_and_replace(self.cursor);
        }
        TokenStream2::from_iter(self.output)
    }

    fn repeat_and_replace(&mut self, cursor: Cursor) {
        let iter = self.range.clone().map(|lit| crate::replace::replace(cursor, self.ident, lit));
        self.output.push(quote::quote! { #(#iter)* });
    }

    // 查找是否存在 `#()*`
    fn is_repeat(&self) -> bool {
        let mut cursor = self.cursor;
        while let Some((token, cur)) = cursor.token_tree() {
            match token {
                TT::Punct(p) if p.as_char() == '#' => {
                    if let Some((TT::Group(_), c_star)) = cur.token_tree() {
                        match c_star.punct() {
                            Some((p, _)) if p.as_char() == '*' => return true,
                            _ => (),
                        }
                    }
                }
                TT::Group(g) => {
                    if SeqToken::new(TokenBuffer::new2(g.stream()).begin(),
                                              self.ident,
                                              self.range.clone()).is_repeat() { return true; }
                }
                _ => (),
            }
            cursor = cur;
        }
        false
    }

    // 查找并替换 `#()*`
    fn search_and_replace(&mut self) {
        while let Some((token, cur)) = self.cursor.token_tree() {
            self.cursor = cur;
            match token {
                TT::Punct(p) if p.as_char() == '#' => {
                    if !self.search_group(cur) {
                        self.output.push(TokenStream2::from(TT::Punct(p)));
                    }
                }
                TT::Group(g) => self.output.push(SeqToken::group(g, self.ident, self.range.clone())),
                t => self.output.push(t.into()),
            }
        }
    }

    fn search_group(&mut self, cur_group: Cursor<'c>) -> bool {
        if let Some((TT::Group(g), c_star)) = cur_group.token_tree() {
            match c_star.token_tree() {
                Some((token, c_next)) if matches!(&token, TT::Punct(p) if p.as_char() == '*') => {
                    self.repeat_and_replace(TokenBuffer::new2(g.stream()).begin());
                    self.cursor = c_next;
                    return true;
                }
                _ => (),
            }
        }
        false
    }

    // 替换后的新标记
    fn output(mut self) -> TokenStream2 {
        self.search_and_replace();
        TokenStream2::from_iter(self.output)
    }

    // 替换后的新标记（只针对 Group）
    fn group(g: Group, ident: &'i Ident, range: Range) -> TokenStream2 {
        let ts = SeqToken::new(TokenBuffer::new2(g.stream()).begin(), ident, range).output();
        TokenStream2::from(TT::Group(crate::new_group(&g, ts)))
    }
}
