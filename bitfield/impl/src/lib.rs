use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Item);
    TokenStream::from(bit::expand(input))
}

mod bit;
mod gen;
mod spe;

#[proc_macro]
pub fn gen(_: TokenStream) -> TokenStream { TokenStream::from(gen::generate()) }

#[proc_macro_derive(BitfieldSpecifier)]
pub fn derive_bitfield_specifier(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemEnum);
    TokenStream::from(spe::derive_bitfield_specifier_for_enum(input))
}
