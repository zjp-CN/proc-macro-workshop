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
    let vars: Vec<_> = input
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect();
    crate::cmp(vars, |pos| input[pos].ident.span())
}
