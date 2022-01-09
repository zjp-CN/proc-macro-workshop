use std::ops::ControlFlow;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{spanned::Spanned, Error};

#[proc_macro_attribute]
pub fn sorted(arg: TokenStream, input: TokenStream) -> TokenStream {
    use quote::ToTokens;
    use syn::{parse_macro_input, Item};

    let input = parse_macro_input!(input as Item);

    if let Item::Enum(input) = &input {
        if let Err(err) = sort(input) {
            TokenStream::from(err.to_compile_error())
        } else {
            TokenStream::from(input.to_token_stream())
        }
    } else {
        let span = TokenStream2::from(arg).span();
        TokenStream::from(Error::new(span, "expected enum or match expression").to_compile_error())
    }
}

fn sort(input: &syn::ItemEnum) -> Result<(), Error> {
    let input = &input.variants;

    let vars: Vec<_> = input.iter().map(|variant| variant.ident.to_string()).collect();
    let mut vars_sorted = vars.clone();
    vars_sorted.sort();

    if let ControlFlow::Break((sorted, raw)) = vars_sorted.iter().zip(vars.iter()).try_for_each(cmp_str) {
        let pos = vars.iter().position(|i| i == sorted).unwrap(); // 一定能找到
        Err(Error::new(input[pos].span(), format!("{} should sort before {}", sorted, raw)))
    } else {
        Ok(())
    }
}

type StrPair<'s> = (&'s String, &'s String);

fn cmp_str((sorted, raw): StrPair) -> ControlFlow<StrPair> {
    if sorted != raw {
        ControlFlow::Break((sorted, raw))
    } else {
        ControlFlow::Continue(())
    }
}
