use std::ops::ControlFlow;
use syn::{
    spanned::Spanned,
    visit_mut::{self, VisitMut},
    Pat,
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
        use quote::format_ident;

        // 移除 `#[sorted]` 并复制 match 表达式分支的模式部分
        let filter = |attr: &syn::Attribute| {
            attr.path.get_ident().map(|i| i == &format_ident!("sorted")).unwrap_or(false)
        };
        if let Some(pos) = node.attrs.iter().position(filter) {
            node.attrs.remove(pos);
            self.0.push(node.arms.iter().map(|arm| arm.pat.clone()).collect());
        }

        visit_mut::visit_expr_match_mut(self, node);
    }
}

impl MatchSorted {
    fn to_vec_string(&self) -> impl Iterator<Item = Vec<String>> + '_ {
        self.0
            .iter()
            .map(|match_item| match_item.iter().filter_map(path_to_string).collect())
    }

    fn cmp(&self) -> syn::Result<()> {
        if let ControlFlow::Break(err) = self.to_vec_string().zip(self.0.iter()).try_for_each(cmp_str) {
            Err(err)
        } else {
            Ok(())
        }
    }
}

// 只支持在部分模式中取路径
fn extract_path(pat: &Pat) -> Option<&syn::Path> {
    Some(match pat {
        Pat::Path(path) => &path.path,
        Pat::Struct(s) => &s.path,
        Pat::TupleStruct(s) => &s.path,
        _ => return None,
    })
}

// 把每个匹配分支中的路径（包括多路径形式的路径）拼接成一个字符串
fn path_to_string(pat: &Pat) -> Option<String> {
    extract_path(pat).map(|p| p.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::"))
}

fn cmp_str((raw, match_item): (Vec<String>, &Vec<Pat>)) -> ControlFlow<syn::Error> {
    if let Err(err) =
        crate::cmp::StringCmp::new(raw, |pos| extract_path(&match_item[pos]).unwrap().span()).check()
    {
        ControlFlow::Break(err)
    } else {
        ControlFlow::Continue(())
    }
}
