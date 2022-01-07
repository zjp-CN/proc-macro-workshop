use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_macro_input, parse_quote, spanned::Spanned, DeriveInput, Result};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match custom_debug(input) {
        Ok(token) => token,
        Err(err) => err.to_compile_error(),
    }.into()
}

mod bound;
mod generics;

fn custom_debug(mut input: DeriveInput) -> Result<TokenStream2> {
    use syn::{Data, DataStruct, Fields, FieldsNamed};
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = &input.data {
        let (ident, generics) = (&input.ident, &mut input.generics);
        let mut opt = bound::struct_attr(&input.attrs);

        // 构造 fmt 方法内部的标记
        let ident_str = ident.to_string();
        let field_idents = named.iter().map(|f| f.ident.as_ref().unwrap());
        let field_idents_str = field_idents.clone().map(|i| i.to_string());
        let field_rhs =
            field_idents.zip(named.iter().map(|f| f.attrs.as_slice()))
                        .map(|(i, a)| attr_debug(a, i, &mut opt).map(|t| t.unwrap_or(quote! {&self.#i})))
                        .collect::<Result<Vec<_>>>()?;

        // 在某些泛型关联类型的情况下，放宽 T: Debug 约束
        let mut associated = HashSet::with_capacity(8);
        let (mut bound_where_clause, bound_generics) = opt.unwrap_or_default();
        let closure = |g: &mut syn::TypeParam| {
            generics::add_debug(g, named.iter().map(|f| &f.ty), &mut associated, &bound_generics)
        };
        generics.type_params_mut().for_each(closure);
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let mut where_clause = where_clause.cloned().unwrap_or_else(|| parse_quote! { where });
        let convert = |ty: &syn::Type| -> syn::WherePredicate { parse_quote!(#ty: ::std::fmt::Debug) };
        bound_where_clause.extend(associated.into_iter().map(convert));
        where_clause.predicates.extend(bound_where_clause);

        Ok(quote! {
            impl #impl_generics ::std::fmt::Debug for #ident #ty_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                    f.debug_struct(&#ident_str)
                        #(
                            .field(&#field_idents_str, #field_rhs)
                        )*
                        .finish()
                }
            }
        })
    } else {
        Err(syn::Error::new(input.span(), "Named Struct Only :)"))
    }
}

fn attr_debug(attrs: &[syn::Attribute], ident: &syn::Ident, opt_preds_ident: &mut bound::OptPredsIdent)
              -> Result<Option<TokenStream2>> {
    use syn::{Lit, LitStr, Meta, MetaNameValue};
    fn debug(attr: &syn::Attribute, opt_preds_ident: &mut bound::OptPredsIdent) -> Option<Result<LitStr>> {
        match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue { path, lit: Lit::Str(s), .. })) if path.is_ident("debug") => {
                Some(Ok(s))
            }
            Ok(meta) => bound::field_attr(meta, opt_preds_ident),
            _ => Some(Err(syn::Error::new(attr.span(), "failed to parse attr meta"))),
        }
    }
    match attrs.iter().find_map(|attr| debug(attr, opt_preds_ident)) {
        None => Ok(None),
        Some(Ok(fmt)) => Ok(Some(quote! { &::std::format_args!(#fmt, self.#ident) })),
        Some(Err(err)) => Err(err),
    }
}
