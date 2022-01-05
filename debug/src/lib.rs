#![allow(unused)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident,
    Lit, Meta, MetaNameValue, Path, Result, Type,
};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(match custom_debug(input) {
        Ok(token) => token,
        Err(err) => err.to_compile_error(),
    })
}

fn custom_debug(input: DeriveInput) -> Result<TokenStream2> {
    let DeriveInput { attrs, vis, ident, generics, data } = &input;
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = data {
        let ident_str = ident.to_string();
        let field_idents = named.iter().map(|f| f.ident.as_ref().unwrap());
        let field_idents_str = field_idents.clone().map(|i| i.to_string());

        let field_rhs = named.iter()
                             .map(|f| f.attrs.as_slice())
                             .zip(field_idents)
                             .map(|(a, i)| attr_debug(a, i).map(|t| t.unwrap_or(quote! {&self.#i})))
                             .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            impl ::std::fmt::Debug for #ident {
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
    fn debug(attr: &syn::Attribute) -> Option<Result<syn::LitStr>> {
        match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue { path, lit: Lit::Str(s), .. })) if path.is_ident("debug") => {
                Some(Ok(s))
            }
            // Err(err) => return Some(Err(Error::new(attr.span(), "failed to parse attr meta"))),
            _ => Some(Err(Error::new(attr.span(), "failed to parse attr meta"))),
        }
    }
    match attrs.iter().find_map(debug) {
        None => Ok(None),
        Some(Ok(fmt)) => Ok(Some(quote! {&format_args!(#fmt, self.#ident)})),
        Some(Err(err)) => Err(err),
    }
}
