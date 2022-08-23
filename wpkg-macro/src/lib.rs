use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use thiserror::Error;
use wpkg_crypto::decode;

const INFO_COLOR: &str = "\u{001B}[33m";
const PURPLE: &str = "\u{001B}[35m";
const RESET: &str = "\u{001B}[0m";

#[proc_macro]
pub fn encode(input: TokenStream) -> TokenStream {
    encode_fn(input.clone()).unwrap_or_else(|e| {
        let msg = format!("{}", e);
        TokenStream::from(quote_spanned! {
            TokenStream2::from(input).span() =>
            compile_error!(#msg)
        })
    })
}

#[derive(Debug, Error)]
enum Error {
    #[error("expected string literal")]
    NonLiteral(#[from] syn::Error),
    #[error("expected string literal")]
    NonStringLiteral,
}

fn encode_fn(input: TokenStream) -> Result<TokenStream, Error> {
    let msg = match syn::parse::<syn::Lit>(input)? {
        syn::Lit::Str(ref literal) => literal.value(),
        _ => return Err(Error::NonStringLiteral),
    };

    let encoded = wpkg_crypto::_encode(wpkg_crypto::KEY, &msg);

    // check if it is possible to decode
    assert_eq!(msg, decode(&encoded));

    println!(
        "{INFO_COLOR}INFO{RESET} {PURPLE}INPUT{RESET}:  {}",
        msg
    );
    println!(
        "{INFO_COLOR}INFO{RESET} {PURPLE}OUTPUT{RESET}: {}",
        encoded
    );

    Ok(quote! {#encoded}.into())
}
