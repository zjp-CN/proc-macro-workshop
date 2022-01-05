use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Error, Ident, Result, Type};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(match custom_debug(input) {
        Ok(token) => token,
        Err(err) => err.to_compile_error(),
    })
}

fn custom_debug(mut input: DeriveInput) -> Result<TokenStream2> {
    use syn::{Data, DataStruct, Fields, FieldsNamed};
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = &input.data {
        let (ident, generics) = (&input.ident, &mut input.generics);
        let ident_str = ident.to_string();
        let field_idents = named.iter().map(|f| f.ident.as_ref().unwrap());
        let field_idents_str = field_idents.clone().map(|i| i.to_string());

        let field_rhs = field_idents.zip(named.iter().map(|f| f.attrs.as_slice()))
                                    .map(|(i, a)| attr_debug(a, i).map(|t| t.unwrap_or(quote! {&self.#i})))
                                    .collect::<Result<Vec<_>>>()?;

        let mut generics_associated = Vec::with_capacity(8);
        generics.type_params_mut()
                .map(|g| generics_add_debug(g, named.iter().map(|f| &f.ty), &mut generics_associated))
                .last();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let where_clause =
            where_clause.cloned()
                        .map(|wh| add_asscociated_bound(wh, &generics_associated))
                        .unwrap_or(syn::parse_quote! { where #(#generics_associated: ::std::fmt::Debug),* });
        let where_clause = Some(where_clause);

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

fn add_asscociated_bound(mut wh: syn::WhereClause, generics_associated: &[Type]) -> syn::WhereClause {
    fn convert(ty: &Type) -> syn::WherePredicate { syn::parse_quote!(#ty: ::std::fmt::Debug) }
    wh.predicates.extend(generics_associated.iter().map(convert));
    wh
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
        Some(Ok(fmt)) => Ok(Some(quote! {&::std::format_args!(#fmt, self.#ident)})),
        Some(Err(err)) => Err(err),
    }
}

fn generics_add_debug<'g>(ty: &mut syn::TypeParam, mut field_ty: impl Iterator<Item = &'g Type>,
                          associated: &mut Vec<Type>) {
    use syn::{parse_quote, TypeParam};
    let TypeParam { ref ident, bounds, .. } = ty;
    let phantom_data: Type = parse_quote!(PhantomData<#ident>);
    // do not add Debug trait constrain when the generics T is PhantomData<T>
    #[rustfmt::skip]
    let closure = |t: &Type| { generics_search(t, ident, associated) || t == &phantom_data };
    if !field_ty.any(closure) {
        bounds.push(parse_quote!(::std::fmt::Debug));
    }
}

fn generics_search(ty: &Type, ident: &Ident, associated: &mut Vec<Type>) -> bool {
    use syn::{AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, TypePath};
    fn check_associated(ty: &Type, ident: &Ident, associated: &mut Vec<Type>) -> bool {
        if let Type::Path(TypePath { path: Path { segments, leading_colon: None }, .. }) = ty {
            if segments.len() > 1 && segments.first().map(|seg| &seg.ident == ident).unwrap_or(false) {
                associated.push(ty.clone());
                return true;
            }
        }
        false
    }
    fn check_angle_bracket_associated(ty: &Type, ident: &Ident, associated: &mut Vec<Type>) -> bool {
        #[rustfmt::skip]
        fn check(seg: &PathSegment, ident: &Ident, associated: &mut Vec<Type>) -> bool {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &seg.arguments
            {
                args.iter().any(|arg| if let GenericArgument::Type(t) = arg
                                 { check_associated(t, ident, associated) } else { false } )
            } else {
                false
            }
        }
        if let Type::Path(TypePath { path: Path { segments, leading_colon: None }, .. }) = ty {
            if segments.iter().any(|seg| check(seg, ident, associated)) {
                associated.push(ty.clone());
                return true;
            }
        }
        false
    }
    check_associated(ty, ident, associated) || check_angle_bracket_associated(ty, ident, associated)
}
