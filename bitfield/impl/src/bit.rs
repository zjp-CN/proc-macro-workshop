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
            let id = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let getter = id.clone().map(|i| quote::format_ident!("get_{}", i));
            let setter = id.clone().map(|i| quote::format_ident!("set_{}", i));

            let ty = fields.named.iter().map(|f| &f.ty);
            let ty2 = ty.clone();
            let width = {
                let ty = ty.clone();
                quote! { [#( <#ty as ::bitfield::Specifier>::BITS ),*] }
            };
            let len = fields.named.len();

            let _id = id.clone();
            let _width_name = id.clone().map(|i| quote::format_ident!("width_{}", i));
            let _width_val = ty.clone().map(|t| quote! { <#t as ::bitfield::Specifier>::BITS  });

            let sig_ty = ty.clone().map(|t| quote! { <#t as ::bitfield::Specifier>::T });
            let size = quote! { #( <#ty as ::bitfield::Specifier>::BITS as usize )+* };
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            let range = 0..len;
            let acc_name = id.clone().map(|i| quote::format_ident!("acc_{}", i));
            let acc_name2 = id.clone().map(|i| quote::format_ident!("acc_{}", i));
            let acc_val = range.map(|n| {
                                   if n == 0 {
                                       quote! { 0 }
                                   } else {
                                       let idx = 0..n;
                                       quote! { #( Self::WIDTH[#idx] )+* }
                                   }
                               });

            // let getter_ty = sig_ty.clone().map(|t| quote::format_ident!("get_{}", t));
            // let ty_idents: Vec<syn::Ident> = sig_ty.clone().map(|t| syn::parse_quote!(#t)).collect();
            // dbg!(ty_idents);

            quote! {
                #(#attrs)*
                #[repr(C)]
                #vis struct #ident #impl_generics #where_clause {
                    // 把原字段内容完全替换成 `data: [u8; #size]`
                    data: [u8; #size >> 3],
                }

                impl #impl_generics #ident #ty_generics #where_clause{
                    #(
                        #vis fn #getter (&self) -> #sig_ty {
                            // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7e8b096e345dc86054814b095c9e3884
                            <#ty2 as ::bitfield::Specifier>::get::<{Self::#acc_name2}>(&self.data)
                        }

                        #vis fn #setter (&mut self, #id : #sig_ty) {
                            <#ty2 as ::bitfield::Specifier>::set::<{Self::#acc_name2}>(&mut self.data, #id)
                        }
                    )*

                    #vis fn new() -> Self {
                        Self { data: ::std::default::Default::default() }
                    }

                    const WIDTH: [usize; #len] = #width;

                    #(
                        // u8 考虑变成 usize 或者 u32
                        #[allow(non_upper_case_globals)]
                        const #acc_name : usize = #acc_val;
                        // #[allow(non_upper_case_globals)]
                        // const #width_name : usize = #width_val as usize;
                        // #[allow(non_upper_case_globals)]
                        // type #_id =
                        //     ::bitfield::BitsPos::<Self::#width_name , Self::#acc_name>;
                    )*
                }
            }
        }
        _ => unimplemented!(),
    }
}
