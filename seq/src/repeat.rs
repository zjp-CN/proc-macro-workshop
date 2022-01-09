use proc_macro2::{Group, Ident, Literal, Punct, TokenStream as TokenStream2, TokenTree as TT};
use quote::quote;
use syn::buffer::{Cursor, TokenBuffer};

// 区分是否需要重复
pub struct SeqToken<'c, 'i, 't> {
    output: Vec<TokenStream2>,
    cursor: Cursor<'c>,
    raw:    Cursor<'c>,
    range:  Range,
    ident:  &'i Ident,
    count:  &'t mut u8,
}

type Range = std::ops::Range<usize>;

impl<'c, 'i, 't> SeqToken<'c, 'i, 't> {
    pub fn new(cursor: Cursor<'c>, ident: &'i Ident, range: Range, count: &'t mut u8) -> Self {
        SeqToken { output: Vec::with_capacity(32),
                   cursor,
                   raw: cursor,
                   range,
                   ident,
                   count }
    }

    fn repeat_and_replace(&mut self, cursor: Cursor) {
        let iter = self.range.clone().map(|lit| crate::replace::replace(cursor, self.ident, lit));
        self.output.push(quote! { #(#iter)* });
    }

    pub fn search_repeat(&mut self) {
        while let Some((token, cur)) = self.cursor.token_tree() {
            self.cursor = cur;
            // dbg!(&self.output);
            // match dbg!(token) {
            match token {
                TT::Punct(p) if p.as_char() == '#' => {
                    // if !dbg!(self.search_group(cur)) {
                    if !self.search_group(cur) {
                        self.output.push(TokenStream2::from(TT::Punct(p)));
                    }
                }
                TT::Group(g) => {
                    self.output.push(SeqToken::group(g, self.ident, self.range.clone(), self.count))
                }
                t => self.output.push(t.into()),
            }
        }
    }

    fn search_group(&mut self, cur_group: Cursor<'c>) -> bool {
        fn check_star(c_star: Cursor) -> Option<Cursor> {
            match c_star.token_tree() {
                Some((token, c_next)) if matches!(&token, TT::Punct(p) if p.as_char() == '*') => Some(c_next),
                _ => None,
            }
        }
        if let Some((TT::Group(g), c_star)) = cur_group.token_tree() {
            if let Some(c_next) = check_star(c_star) {
                self.repeat_and_replace(TokenBuffer::new2(g.stream()).begin());
                self.cursor = c_next;
                *self.count += 1;
                // dbg!(&self.output);
                return true;
            }
        }
        false
    }

    fn output(mut self) -> TokenStream2 {
        self.search_repeat();
        TokenStream2::from_iter(self.output)
    }

    // 如果存在 `#()*`，则不重复整个块
    pub fn token_stream(mut self) -> TokenStream2 {
        self.search_repeat();
        if *self.count == 0 {
            // dbg!(&self.output, self.count);
            self.output.clear();
            self.repeat_and_replace(self.raw);
        }
        TokenStream2::from_iter(self.output)
    }

    fn group(g: Group, ident: &'i Ident, range: Range, count: &'t mut u8) -> TokenStream2 {
        let output = SeqToken::new(TokenBuffer::new2(g.stream()).begin(), ident, range, count).output();
        let mut group = Group::new(g.delimiter(), output);
        group.set_span(g.span());
        TokenStream2::from(TT::Group(group))
    }
}
