use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_macro_input, parse_quote, spanned::Spanned, DeriveInput, Error, Ident, Result, Type};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(match custom_debug(input) {
        Ok(token) => token,
        Err(err) => err.to_compile_error(),
    })
}

mod bound;

fn custom_debug(mut input: DeriveInput) -> Result<TokenStream2> {
    use syn::{Data, DataStruct, Fields, FieldsNamed};
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = &input.data {
        let (ident, generics) = (&input.ident, &mut input.generics);
        
        // 构造 fmt 方法内部的标记
        let ident_str = ident.to_string();
        let field_idents = named.iter().map(|f| f.ident.as_ref().unwrap());
        let field_idents_str = field_idents.clone().map(|i| i.to_string());
        let field_rhs = field_idents.zip(named.iter().map(|f| f.attrs.as_slice()))
                                    .map(|(i, a)| attr_debug(a, i).map(|t| t.unwrap_or(quote! {&self.#i})))
                                    .collect::<Result<Vec<_>>>()?;

        // 在某些泛型关联类型的情况下，放宽 T: Debug 约束
        let mut associated = HashSet::with_capacity(8);
        let (mut bound_where_clause, bound_generics) = bound::attr_bound(&input.attrs).unwrap_or_default();
        generics.type_params_mut()
                .map(|g| generics_add_debug(g, named.iter().map(|f| &f.ty), &mut associated, &bound_generics))
                .last();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let mut where_clause = where_clause.cloned().unwrap_or_else(|| parse_quote! { where });
        let convert = |ty: &Type| -> syn::WherePredicate { parse_quote!(#ty: ::std::fmt::Debug) };
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
        Err(Error::new(input.span(), "Named Struct Only :)"))
    }
}

fn attr_debug(attrs: &[syn::Attribute], ident: &Ident) -> Result<Option<TokenStream2>> {
    use syn::{Lit, LitStr, Meta, MetaNameValue};
    fn debug(attr: &syn::Attribute) -> Option<Result<LitStr>> {
        match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue { path, lit: Lit::Str(s), .. })) if path.is_ident("debug") => {
                Some(Ok(s))
            }
            _ => Some(Err(Error::new(attr.span(), "failed to parse attr meta"))),
        }
    }
    match attrs.iter().find_map(debug) {
        None => Ok(None),
        Some(Ok(fmt)) => Ok(Some(quote! { &::std::format_args!(#fmt, self.#ident) })),
        Some(Err(err)) => Err(err),
    }
}

fn generics_add_debug<'f>(ty: &mut syn::TypeParam, field_ty: impl Iterator<Item = &'f Type>,
                          associated: &mut HashSet<&'f Type>, bound: &HashSet<Ident>) {
    let syn::TypeParam { ref ident, bounds, .. } = ty;
    let phantom_data: Type = parse_quote!(PhantomData<#ident>);
    // do not add Debug trait constrain
    // when the generics T contains associated types or T is PhantomData<T> or
    // `T::Associated: Debug` is in bound
    if !field_ty.fold(bound.contains(ident), |acc, t| {
                    generics_search(t, ident, associated) || t == &phantom_data || acc
                })
    {
        bounds.push(parse_quote!(::std::fmt::Debug));
    }
}

// 处理字段类型的关联类型
fn generics_search<'f>(ty: &'f Type, ident: &Ident, associated: &mut HashSet<&'f Type>) -> bool {
    use syn::{AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, TypePath};

    // 把 T::Associated 添加到 where 语句增加项
    fn check_associated<'f>(ty: &'f Type, ident: &Ident, associated: &mut HashSet<&'f Type>) -> bool {
        if let Type::Path(TypePath { path: Path { segments, leading_colon: None }, .. }) = ty {
            if segments.len() > 1 && segments.first().map(|seg| &seg.ident == ident).unwrap_or(false) {
                associated.insert(ty);
                return true;
            }
        }
        false
    }

    // 一层尖括号泛型中的关联类型 path::<T::Associated>
    fn check_angle_bracket_associated<'f>(ty: &'f Type, ident: &Ident, associated: &mut HashSet<&'f Type>)
                                          -> bool {
        // 检查尖括号内的泛型是否为关联类型
        fn check<'f>(arg: &'f PathArguments, ident: &Ident, associated: &mut HashSet<&'f Type>) -> bool {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = arg {
                args.iter().fold(false, |acc, arg| {
                               if let GenericArgument::Type(t) = arg {
                                   check_associated(t, ident, associated) || acc
                               } else {
                                   acc
                               }
                           })
            } else {
                false
            }
        }
        if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty {
            // 只考虑最后路径上的泛型，即 a::b::c::<T, I::Item, ...> 形式
            return segments.last()
                           .map(|seg| check(&seg.arguments, ident, associated))
                           .unwrap_or(false);
        }
        false
    }

    check_associated(ty, ident, associated) || check_angle_bracket_associated(ty, ident, associated)
}
