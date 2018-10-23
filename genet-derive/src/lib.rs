#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::{Attribute, DeriveInput, Lit, Meta, MetaNameValue};

#[proc_macro_derive(Attr, attributes(genet))]
pub fn derive_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("{:?}", parse_docs(&input.attrs));

    let tokens = quote!{};
    tokens.into()
}

fn parse_docs(attrs: &[Attribute]) -> (String, String) {
    let mut docs = String::new();
    for attr in attrs {
        if let Some(meta) = attr.interpret_meta() {
            if meta.name() == "doc" {
                if let Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(lit_str),
                    ..
                }) = meta
                {
                    docs += &lit_str.value();
                    docs += "\n";
                }
            }
        }
    }
    let mut lines = docs.split('\n').map(|line| line.trim());
    let title = lines.next().unwrap_or("");
    let description = lines.fold(String::new(), |acc, x| acc + x + "\n");
    (title.into(), description.trim().into())
}
