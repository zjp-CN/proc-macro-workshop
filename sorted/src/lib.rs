use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Item);
    TokenStream::from(_enum::process(&input))
}

// workaround for procedural macro invocations (like `#[sorted]`) on expressions
// see test/05-match-expr.rs
#[proc_macro_attribute]
pub fn check(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);
    TokenStream::from(_fn::process(&mut input))
}

mod _enum;
mod _fn;

fn cmp<F: FnOnce(usize) -> proc_macro2::Span>(raw: Vec<String>, f: F) -> syn::Result<()> {
    let mut sorted = raw.clone();
    sorted.sort();
    let (mut raw, sorted) = (raw.iter(), sorted.iter());
    sorted
        .zip(raw.clone())
        .try_for_each(|(sorted_s, raw_s)| {
            if sorted_s != raw_s {
                let pos = raw.position(|i| i == sorted_s).unwrap(); // 一定能找到
                Err((sorted_s, raw_s, pos))
            } else {
                Ok(())
            }
        })
        .map_err(|(sorted_s, raw_s, pos)| {
            syn::Error::new(
                (f)(pos),
                format!("{} should sort before {}", sorted_s, raw_s),
            )
        })
}
