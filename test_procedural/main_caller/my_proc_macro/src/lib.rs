use proc_macro::{self, TokenStream};
use quote::*;
use syn::*;

#[proc_macro_derive(Describe)]
pub fn dumb(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);

    quote!(
        #parsed
    )
    .into() //From TokenStream2 to TokenStrem
}
