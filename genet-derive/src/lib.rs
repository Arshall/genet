extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Attr, attributes(genet))]
pub fn derive_attr(input: TokenStream) -> TokenStream {
    input
}
