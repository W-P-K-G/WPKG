use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn key(_input: TokenStream) -> TokenStream {
    let version_str = env!("CRYPTO_KEY");

    let version: usize = version_str.parse().expect("invalid key");

    quote! {#version}.into()
}
