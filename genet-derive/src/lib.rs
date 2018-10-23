#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput};

#[proc_macro_derive(Attr, attributes(genet))]
pub fn derive_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    for attr in input.attrs {
        println!("{:?}", attr.tts.to_string());
    }
    let tokens = quote! {
        
    };
    tokens.into()
}
