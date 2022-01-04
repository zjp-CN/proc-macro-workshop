#![allow(unused)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let token = custom_debug(input);
    TokenStream::from(token)
}

fn custom_debug(input: DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput { attrs, vis, ident, generics, data } = input;
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) = &data {
        let ident_str = ident.to_string();
        let field_idents = named.iter().map(|f| f.ident.as_ref().unwrap());
        let field_idents_str = field_idents.clone().map(|i| i.to_string());
        quote! {
            impl ::std::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                    f.debug_struct(&#ident_str)
                        #(
                            .field(&#field_idents_str, &self.#field_idents)
                        )*
                        .finish()
                }
            }
        }
    } else {
        unimplemented!()
    }
}
