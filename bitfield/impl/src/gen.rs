//! 生成 B1 ~ B64（只在 lib 内部使用）

use quote::format_ident;

pub fn generate() -> proc_macro2::TokenStream {
    let range = 1..=64;
    let ident = range.clone().map(|n| format_ident!("B{}", n));
    let (u_ident, bits_u): (Vec<_>, Vec<_>) = range.clone().map(u_).unzip();
    quote::quote! {
        #(
            pub struct #ident;
            impl Specifier for #ident {
                const BITS: usize = #range;
                type T = ::core::primitive::#u_ident;

                fn set<const ACC: usize, const SIZE: usize>(arr: &mut [u8], num: <Self as Specifier>::T) {
                    <#bits_u <#range, ACC, SIZE> as SetGet>::SET(arr, num)
                }
                fn get<const ACC: usize, const SIZE: usize>(arr: &[u8]) -> <Self as Specifier>::T {
                    <#bits_u <#range, ACC, SIZE> as SetGet>::GET(arr)
                }
            }
        )*
    }
}

// 1*8 => u8; 2*8 => u16; 4*8 => u32; 8*8 => u64
// 可以通过位运算优化这里的分支判断
fn u_(bits: usize) -> (proc_macro2::Ident, proc_macro2::Ident) {
    let u = if bits > 64 {
        unreachable!()
    } else if bits > 32 {
        64u8
    } else if bits > 16 {
        32
    } else if bits > 8 {
        16
    } else {
        8
    };
    (format_ident!("u{}", u), format_ident!("BitsU{}", u))
}
