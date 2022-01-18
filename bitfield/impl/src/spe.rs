pub fn derive_bitfield_specifier_for_enum(input: syn::ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let vars = input.variants.iter().map(|v| &v.ident);
    let vars2 = vars.clone();
    let bits = log2(input.variants.len() as u32); // TODO: 考虑 len=0 和不是 8 倍数的情况
    let ty = quote::format_ident!("B{}", bits);
    let ty_u = quote::format_ident!("__{}", input.ident);
    let ty_equiv = quote::format_ident!("__{}Equiv", input.ident);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote::quote! {
        // derive 宏无需返回被定义的 item
        // #input

        // 这里定义辅助的 trait 可能无济于事
        type #ty_equiv = ::bitfield::#ty;
        type #ty_u = <#ty_equiv as ::bitfield::Specifier>::T;
        impl #impl_generics #enum_name #ty_generics #where_clause {
                fn from_integer(num: #ty_u) -> Self {
                    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bed314b000b163a027a7a5312c94e74b
                    use #enum_name::*;
                    [#(#vars as #ty_u),*].into_iter()
                        .zip([#(#vars2),*].into_iter())
                        .find_map(|(u, e)| if u == num { Some(e) } else { None })
                        // The variant must be found in this case.
                        .unwrap()
                }
            }

        impl #impl_generics ::bitfield::Specifier for #enum_name #ty_generics #where_clause {
            type T = #enum_name;
            const BITS: usize = <#ty_equiv as ::bitfield::Specifier>::BITS;
            fn set<const ACC: usize>(arr: &mut [u8], num: <Self as Specifier>::T) {
                <#ty_equiv as ::bitfield::Specifier>::set::<ACC>(arr, num as #ty_u)
            }
            fn get<const ACC: usize>(arr: &[u8]) -> <Self as Specifier>::T {
                #enum_name::from_integer(<#ty_equiv as ::bitfield::Specifier>::get::<ACC>(arr))
            }
        }
    }
}

const fn log2(n: u32) -> u32 { u32::BITS - n.leading_zeros() - 1 }
