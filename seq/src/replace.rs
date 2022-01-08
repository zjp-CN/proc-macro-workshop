use proc_macro2::{Group, Ident, Literal, Punct, TokenStream as TokenStream2, TokenTree as TT};
use syn::buffer::Cursor;

// 将所有 ident 替换成字面值、`prefix~ident` 替换成 `prefix字面值`
pub fn replace(mut cursor: Cursor, ident: &Ident, lit: usize) -> TokenStream2 {
    let mut ts = Vec::with_capacity(32);
    while let Some((token, cur)) = cursor.token_tree() {
        cursor = cur;
        // dbg!(&ts);
        // match dbg!(token) {
        match token {
            TT::Ident(i) => {
                if &i == ident {
                    ts.push(Literal::usize_unsuffixed(lit).into());
                } else if let Some((matched, cur)) =
                    search_tidle_ident(i.clone().into(), cur, ident, lit, &mut ts)
                {
                    cursor = cur;
                    if matched {
                        ts.push(quote::format_ident!("{}{}", i, lit).into());
                    }
                } else {
                    ts.push(i.into());
                }
            }
            TT::Group(ref g) => match_group(g, &mut ts, ident, lit),
            t => ts.push(t),
        }
    }
    TokenStream2::from_iter(ts)
}

fn match_group(g: &Group, ts: &mut Vec<TT>, ident: &Ident, lit: usize) {
    let buf = syn::buffer::TokenBuffer::new2(g.stream());
    let mut group = Group::new(g.delimiter(), replace(buf.begin(), ident, lit));
    group.set_span(g.span());
    ts.push(group.into());
}

type Search<'c> = Option<(bool, Cursor<'c>)>;

// 查找某个标识符 i 之后是否跟 `~ident`，如果返回：
// - Some((true, cur)) 表示找到
// - Some((false, cur)) 表示未找到，且把捕获的标记添加到 ts
// - None 表示标记流结束，或者 ** i 与此宏功能无关**
fn search_tidle_ident<'c>(i: TT, cursor: Cursor<'c>, ident: &Ident, lit: usize, ts: &mut Vec<TT>)
                          -> Search<'c> {
    fn search_ident<'c>(i: TT, tidle: TT, cursor: Cursor<'c>, ident: &Ident, lit: usize, ts: &mut Vec<TT>)
                        -> Search<'c> {
        // cursor.token_tree().map(|(token, cur)| match dbg!(&token) {
        cursor.token_tree().map(|(token, cur)| match &token {
                               TT::Ident(id) if id == ident => (true, cur),
                               TT::Group(g) => {
                                   ts.extend([i, tidle]);
                                   match_group(g, ts, ident, lit);
                                   (false, cur)
                               }
                               _ => {
                                   ts.extend([i, tidle, token]);
                                   (false, cur)
                               }
                           })
    }
    // cursor.token_tree().and_then(|(token, cur)| match dbg!(&token) {
    cursor.token_tree().and_then(|(token, cur)| match &token {
                           TT::Ident(id) => None,
                           TT::Punct(p) if p.as_char() == '~' => search_ident(i, token, cur, ident, lit, ts),
                           TT::Group(g) => {
                               ts.push(i);
                               match_group(g, ts, ident, lit);
                               Some((false, cur))
                           }
                           _ => {
                               ts.extend([i, token]);
                               Some((false, cur))
                           }
                       })
}
