//! 生成 B1 ~ B64（只在 lib 内部使用）

pub fn generate() -> proc_macro2::TokenStream {
    //
    let range = 1..=64;
    let ident = range.clone().map(|n| quote::format_ident!("B{}", n));
    let (u_ident, u): (Vec<_>, Vec<_>) = range.clone().map(u_).unzip();
    quote::quote! {
        #(
            pub struct #ident;
            impl Specifier for #ident {
                const BITS: u8 = #range;
                type T = #u_ident;
                const TLEN: u8 = #u;
                const MAX: <Self as Specifier>::T = #u_ident::MAX >> (#u - #range);
            }
        )*
    }
}

// 1*8 => u8; 2*8 => u16; 4*8 => u32; 8*8 => u64
fn u_(bits: u8) -> (proc_macro2::Ident, u8) {
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
    (quote::format_ident!("u{}", u), u)
}
