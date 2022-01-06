use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Attribute, Ident, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta, Path, PredicateType, Type, TypePath, WherePredicate,
};

pub type PunctPreds = Punctuated<WherePredicate, Comma>;

pub type PredsIdent = Option<(PunctPreds, std::collections::HashSet<Ident>)>;

pub fn attr_bound(attrs: &[Attribute]) -> PredsIdent {
    fn search_bound(meta: Meta) -> PredsIdent {
        if let Meta::List(MetaList { nested, .. }) = meta {
            nested.iter().find_map(predicate)
        } else {
            None
        }
    }

    fn predicate(m: &NestedMeta) -> PredsIdent {
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

    fn search_debug(meta: Meta) -> PredsIdent {
        let debug: Path = parse_quote!(debug);
        if meta.path() == &debug {
            search_bound(meta)
        } else {
            None
        }
    }

    attrs.iter().find_map(|attr| attr.parse_meta().ok().and_then(search_debug))
}
