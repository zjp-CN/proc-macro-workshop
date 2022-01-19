use std::borrow::Cow;
use syn::{
    spanned::Spanned,
    visit_mut::{self, VisitMut},
    Error, Pat, Result,
};

pub fn process(input: &mut syn::ItemFn) -> proc_macro2::TokenStream {
    use quote::ToTokens;

    let mut pats = MatchSorted(Vec::with_capacity(8));
    pats.visit_item_fn_mut(input);

    let input = input.to_token_stream();
    if let Err(err) = pats.cmp() {
        let err = err.to_compile_error();
        quote::quote! { #input #err }
    } else {
        input
    }
}

struct MatchSorted(Vec<Vec<Pat>>);

impl VisitMut for MatchSorted {
    fn visit_expr_match_mut(&mut self, node: &mut syn::ExprMatch) {
        // 移除 `#[sorted]` 并复制 match 表达式分支的模式部分
        let filter = |attr: &syn::Attribute| {
            attr.path
                .get_ident()
                .map(|i| i == &quote::format_ident!("sorted"))
                .unwrap_or(false)
        };
        if let Some(pos) = node.attrs.iter().position(filter) {
            node.attrs.remove(pos);
            self.0.push(node.arms.iter().map(|arm| arm.pat.clone()).collect());
        }

        visit_mut::visit_expr_match_mut(self, node);
    }
}

impl MatchSorted {
    fn to_vec_string(&self) -> Result<Vec<Vec<String>>> {
        let mut v = Vec::with_capacity(8);
        for match_item in self.0.iter() {
            let mut u = Vec::with_capacity(8);
            for pat in match_item.iter() {
                u.push(path_to_string(pat)?);
            }
            v.push(u);
        }
        Ok(v)
    }

    fn cmp(&self) -> Result<()> {
        self.to_vec_string()?
            .into_iter()
            .zip(self.0.iter())
            .try_for_each(|(raw, match_item)| {
                crate::cmp::finish(raw, |pos| extract_path(&match_item[pos]).unwrap().span())
            })
    }
}

// 只支持在部分模式中取路径
fn extract_path(pat: &Pat) -> Result<Cow<'_, syn::Path>> {
    use syn::parse_quote_spanned;
    let path = match pat {
        Pat::Path(path) => Cow::Borrowed(&path.path),
        Pat::Struct(s) => Cow::Borrowed(&s.path),
        Pat::TupleStruct(s) => Cow::Borrowed(&s.path),
        Pat::Ident(syn::PatIdent { ident: i, .. }) => Cow::Owned(parse_quote_spanned! { i.span()=> #i }),
        Pat::Wild(w) => {
            // 无法使用 parse_quote_spanned! 把 `_` 转化成 Path，所以需要手动构造
            // Cow::Owned(parse_quote_spanned! { w.underscore_token.span()=> #underscore })
            let underscore: syn::Ident = w.underscore_token.into();
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(underscore.into());
            Cow::Owned(syn::Path { leading_colon: None, segments })
        }
        p => return Err(Error::new(p.span(), "unsupported by #[sorted]")),
    };
    Ok(path)
}

// 把每个匹配分支中的路径（包括多路径形式的路径）拼接成一个字符串
fn path_to_string(pat: &Pat) -> Result<String> {
    extract_path(pat).map(|p| p.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::"))
}
