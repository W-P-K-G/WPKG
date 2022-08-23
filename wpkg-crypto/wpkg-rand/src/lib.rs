use proc_macro::TokenStream;
use quote::quote;
use rand::{thread_rng, Rng};

#[proc_macro]
pub fn rand(_input: TokenStream) -> TokenStream {
    let mut rng = thread_rng();

    let num: u8 = rng.gen_range(5..50);

    quote! {#num}.into()
}
