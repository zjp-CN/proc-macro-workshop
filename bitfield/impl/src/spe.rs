pub fn derive_bitfield_specifier_for_enum(input: syn::ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let vars = input.variants.iter().map(|v| &v.ident);

    let len = input.variants.len() as u32;
    let bits = if let Some(b) = log2_exact(len) {
        b
    } else {
        return syn::Error::new(proc_macro2::Span::call_site(),
                               "BitfieldSpecifier expected a number of variants which is a power of 2").to_compile_error();
    };

    // __check_bits
    let var = vars.clone();
    let max = 1u32 << log2(len);

    let ty = quote::format_ident!("B{}", bits);
    let ty_u = quote::format_ident!("__{}", input.ident);
    let ty_equiv = quote::format_ident!("__{}Equiv", input.ident);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // derive 宏无需返回被定义的 item
    quote::quote! {
        // 这里的技巧：在 const 中实现 trait，因为 impl
        // 是全局静态的，无关代码是否执行，也无论是否跨作用域。
        // 受 https://users.rust-lang.org/t/conditional-trait-implementations/11850/4 启发。
        //
        // 此外， `const _` 利用了
        // [Unnamed constant 技巧](https://doc.rust-lang.org/nightly/reference/items/constant-items.html#unnamed-constant)
        const _ : () = {
            type #ty_equiv = ::bitfield::#ty;
            type #ty_u = <#ty_equiv as ::bitfield::Specifier>::T;
            // 测试 discriminant 必须小于 MAX
            const _ : () = {
                use #enum_name::*;
                const MAX: #ty_u = #max as #ty_u;
                #(
                    // 受 [static_assertions::const_assert](https://docs.rs/static_assertions) 启发
                    const _ : #ty_u = 0 - !((#var as #ty_u) < MAX) as #ty_u;
                )*
            };

            impl #impl_generics ::bitfield::Specifier for #enum_name #ty_generics #where_clause {
                type T = #enum_name;
                const BITS: usize = <#ty_equiv as ::bitfield::Specifier>::BITS;
                fn set<const ACC: usize>(arr: &mut [u8], num: <Self as Specifier>::T) {
                    <#ty_equiv as ::bitfield::Specifier>::set::<ACC>(arr, num as #ty_u)
                }
                fn get<const ACC: usize>(arr: &[u8]) -> <Self as Specifier>::T {
                    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bed314b000b163a027a7a5312c94e74b
                    fn __from_integer(num: #ty_u) -> #enum_name {
                        use #enum_name::*;
                        let arr = [#( (#vars as #ty_u, #vars) ),*];
                        // The variant must be found in this case.
                        arr.into_iter().find_map(|(u, e)| if u == num { Some(e) } else { None }).unwrap()
                    }
                    __from_integer(<#ty_equiv as ::bitfield::Specifier>::get::<ACC>(arr))
                }
            }
        };
    }
}

// 改进自 https://users.rust-lang.org/t/logarithm-of-integers/8506/5
// 这个函数可以根据 enum 的成员数自动计算最小 bits
const fn log2(n: u32) -> u32 { u32::BITS - n.leading_zeros() - 1 + (n.count_ones() != 1) as u32 }

const fn log2_exact(n: u32) -> Option<u32> {
    if n.count_ones() == 1 {
        Some(u32::BITS - n.leading_zeros() - 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests_log2 {
    use super::{log2, log2_exact};

    #[test]
    fn test_log2() {
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(3), 2);
        assert_eq!(log2(4), 2);
        assert_eq!(log2(8), 3);
        assert_eq!(log2(10), 4);
        assert_eq!(log2(63), 6);
        assert_eq!(log2(64), 6);
        assert_eq!(log2(1024), 10);
        assert_eq!(log2(1025), 11);
        assert_eq!(log2(u32::MAX), 32);
    }

    #[test]
    #[should_panic]
    fn test_log2_0() { log2(0); }

    #[test]
    fn test_log2_exact() {
        assert_eq!(log2_exact(1), Some(0));
        assert_eq!(log2_exact(2), Some(1));
        assert_eq!(log2_exact(3), None);
        assert_eq!(log2_exact(4), Some(2));
        assert_eq!(log2_exact(8), Some(3));
        assert_eq!(log2_exact(10), None);
        assert_eq!(log2_exact(63), None);
        assert_eq!(log2_exact(64), Some(6));
        assert_eq!(log2_exact(1024), Some(10));
        assert_eq!(log2_exact(1025), None);
        assert_eq!(log2_exact(u32::MAX), None);
    }

    #[test]
    fn test_log2_0_extact() {
        assert_eq!(log2_exact(0), None);
    }
}
