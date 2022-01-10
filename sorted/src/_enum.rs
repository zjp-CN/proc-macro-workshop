use std::ops::ControlFlow;

pub fn process(input: &syn::Item) -> proc_macro2::TokenStream {
    let err = if let syn::Item::Enum(input) = input {
        if let Err(err) = sort(input) {
            err.to_compile_error()
        } else {
            use quote::ToTokens;
            return input.to_token_stream();
        }
    } else {
        let span = proc_macro2::Span::call_site();
        syn::Error::new(span, "expected enum or match expression").to_compile_error()
    };

    quote::quote! { #input #err }
}

fn sort(input: &syn::ItemEnum) -> syn::Result<()> {
    let input = &input.variants;

    let vars: Vec<_> = input.iter().map(|variant| variant.ident.to_string()).collect();
    let mut vars_sorted = vars.clone();
    vars_sorted.sort();

    if let ControlFlow::Break((sorted, raw)) = vars_sorted.iter().zip(vars.iter()).try_for_each(cmp_str) {
        let pos = vars.iter().position(|i| i == sorted).unwrap(); // 一定能找到
        Err(syn::Error::new(input[pos].ident.span(), format!("{} should sort before {}", sorted, raw)))
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
