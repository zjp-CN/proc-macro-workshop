//! 生成 B1 ~ B64（只在 lib 内部使用）

pub fn generate() -> proc_macro2::TokenStream {
    let range = 1..=64;
    let ident = range.clone().map(|n| quote::format_ident!("B{}", n));
    let (u_ident, _u): (Vec<_>, Vec<_>) = range.clone().map(u_).unzip();
    let setter = u_ident.iter().map(|i| quote::format_ident!("set_{}", i));
    let getter = u_ident.iter().map(|i| quote::format_ident!("get_{}", i));
    quote::quote! {
        #(
            pub struct #ident;
            impl Specifier for #ident {
                const BITS: usize = #range;
                type T = ::core::primitive::#u_ident;

                fn set<const ACC: usize>(arr: &mut [u8], num: <Self as Specifier>::T) {
                    <BitsPos<#range, ACC>>::#setter(arr, num)
                }
                fn get<const ACC: usize>(arr: &[u8]) -> <Self as Specifier>::T {
                    <BitsPos<#range, ACC>>::#getter(arr)
                }
            }
        )*
    }
}

// 1*8 => u8; 2*8 => u16; 4*8 => u32; 8*8 => u64
fn u_(bits: usize) -> (proc_macro2::Ident, usize) {
    let u = if bits > 64 {
        unreachable!()
    } else if bits > 32 {
        64
    } else if bits > 16 {
        32
    } else if bits > 8 {
        16
    } else {
        8
    };
    (quote::format_ident!("u{}", u), u)
}
