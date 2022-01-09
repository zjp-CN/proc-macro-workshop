use crate::range::RangeLit;
use proc_macro2::{Group, Ident, Literal, TokenStream as TokenStream2, TokenTree as TT};
use syn::buffer::{Cursor, TokenBuffer};

// 将所有 ident 替换成字面值、`prefix~ident` 替换成 `prefix字面值`
pub fn replace(mut cursor: Cursor, ident: &Ident, lit: RangeLit) -> TokenStream2 {
    let mut ts = Vec::with_capacity(32);
    while let Some((token, cur)) = cursor.token_tree() {
        cursor = cur;
        match token {
            TT::Ident(i) => {
                if &i == ident {
                    // TODO: 字面值暂时只假设为 usize（由 RangeLit 统一描述），它可以通过所有测试
                    //       真实场景中，需要放宽这个类型（见 https://docs.rs/seq-macro ）
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

fn match_group(g: &Group, ts: &mut Vec<TT>, ident: &Ident, lit: RangeLit) {
    let tokens = replace(TokenBuffer::new2(g.stream()).begin(), ident, lit);
    ts.push(crate::new_group(g, tokens).into());
}

type Search<'c> = Option<(bool, Cursor<'c>)>;

// 查找某个标识符 i 之后是否跟 `~ident`，如果返回：
// - Some((true, cur)) 表示找到
// - Some((false, cur)) 表示未找到，且把捕获的标记添加到 ts
// - None 在 search_ident 函数中表示遇到标记流结束（虽然它最终不返回 None）； 在
//   search_tidle_ident 函数中表示 **i 与此宏功能无关**，或者标记流结束
fn search_tidle_ident<'c>(i: TT, cursor: Cursor<'c>, ident: &Ident, lit: RangeLit, ts: &mut Vec<TT>)
                          -> Search<'c> {
    fn search_ident<'c>(i: TT, tidle: TT, cursor: Cursor<'c>, ident: &Ident, lit: RangeLit,
                        ts: &mut Vec<TT>)
                        -> Search<'c> {
        if let Some((token, cur)) = cursor.token_tree() {
            match &token {
                TT::Ident(id) if id == ident => Some((true, cur)),
                TT::Group(g) => {
                    ts.extend([i, tidle]);
                    match_group(g, ts, ident, lit);
                    Some((false, cur))
                }
                _ => {
                    ts.extend([i, tidle, token]);
                    Some((false, cur))
                }
            }
        } else {
            // 虽然几乎不会遇到 `i~` 结尾的标记，但是还是需要考虑
            ts.extend([i, tidle]);
            Some((false, cursor))
        }
    }
    cursor.token_tree().and_then(|(token, cur)| match &token {
                           TT::Ident(_) => None,
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
