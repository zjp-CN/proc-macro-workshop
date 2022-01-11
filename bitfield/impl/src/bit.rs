use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn expand(input: syn::Item) -> TokenStream2 {
    match input {
        syn::Item::Struct(syn::ItemStruct { attrs,
                                            vis,
                                            ident,
                                            generics,
                                            fields: syn::Fields::Named(fields),
                                            .. }) => {
            let size = fields.named.iter().map(|f| {
                                              let ty = &f.ty;
                                              quote! { <#ty as ::bitfield::Specifier>::BITS as usize }
                                          });
            let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();
            // 把原字段内容完全替换成 `data: [u8; #size]`
            quote! {
                #(#attrs)*
                #[repr(C)]
                #vis struct #ident #impl_generics #where_clause {
                    data: [u8; (0 #( + #size)*) / 8 ],
                }
            }
        }
        _ => unimplemented!(),
    }
}
