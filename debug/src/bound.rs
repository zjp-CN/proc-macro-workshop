use syn::{punctuated::Punctuated, token::Comma, Attribute, Ident, LitStr, Meta, Result, WherePredicate};

type PunctPreds = Punctuated<WherePredicate, Comma>;
type PredsIdent = (PunctPreds, std::collections::HashSet<Ident>);
pub type OptPredsIdent = Option<PredsIdent>;

pub fn struct_attr(attrs: &[Attribute]) -> OptPredsIdent {
    attrs.iter().find_map(|attr| attr.parse_meta().ok().and_then(search::debug))
}

pub fn field_attr(meta: Meta, opt_preds_ident: &mut OptPredsIdent) -> Option<Result<LitStr>> {
    fn transform(preds_ident: PredsIdent, opt_preds_ident: &mut OptPredsIdent) -> Option<Result<LitStr>> {
        if let Some((p, s)) = opt_preds_ident.as_mut() {
            p.extend(preds_ident.0);
            s.extend(preds_ident.1);
        } else {
            opt_preds_ident.replace(preds_ident);
        }
        None
    }
    search::debug(meta).and_then(|preds_ident| transform(preds_ident, opt_preds_ident))
}

mod search {
    use super::{OptPredsIdent, PunctPreds};
    use syn::{
        parse_quote, punctuated::Punctuated, Ident, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Path,
        PredicateType, Type, TypePath, WherePredicate,
    };

    pub fn debug(meta: Meta) -> OptPredsIdent {
        let debug: Path = parse_quote!(debug);
        if meta.path() == &debug {
            search_bound(meta)
        } else {
            None
        }
    }

    fn search_bound(meta: Meta) -> OptPredsIdent {
        if let Meta::List(MetaList { nested, .. }) = meta {
            nested.iter().find_map(predicate)
        } else {
            None
        }
    }

    fn predicate(m: &NestedMeta) -> OptPredsIdent {
        let bound: Path = parse_quote!(bound);
        match m {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) if path == &bound => {
                if let Lit::Str(s) = lit {
                    let wp: PunctPreds = s.parse_with(Punctuated::parse_terminated).ok()?;
                    let set = wp.iter().filter_map(search_generics_ident).collect();
                    Some((wp, set))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn search_generics_ident(w: &WherePredicate) -> Option<Ident> {
        if let WherePredicate::Type(PredicateType { bounded_ty: Type::Path(TypePath { path, .. }), .. }) = w {
            // 最好校验 bound 属性内的泛型关联类型是否明确写明 `Debug`，但是这比较麻烦
            // 所以，只要在 bound 属性写了泛型 T 的相关语句都不会给 T 增加 Debug 约束
            path.segments.first().map(|seg| seg.ident.clone())
        } else {
            None
        }
    }
}
