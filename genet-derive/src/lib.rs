#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate inflector;
extern crate proc_macro;
extern crate proc_macro2;

use inflector::cases::camelcase::to_camel_case;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::{Attribute, Data, DeriveInput, Fields, Ident, Lit, Meta, MetaNameValue, Type, TypePath};

#[proc_macro_derive(Attr, attributes(genet))]
pub fn derive_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
/*
    let name = "444";
    let expanded = quote! {
        Str {
            ii: #name.into()
        }
    };
    println!("{:?}", expanded);

    if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            for field in f.named {
                println!("{:?} {:?}", field.ident, parse_docs(&field.attrs));
                if let Type::Path(p) = field.ty {
                    println!(
                        "{:?}",
                        p == syn::parse_str::<TypePath>("PayloadLen").unwrap()
                    );
                }
            }
        }
    }
*/
    let ident = input.ident;
    let tokens = quote!{
        impl ::genet_sdk::attr::AttrNode for #ident {
            fn init(&mut self, ctx: &::genet_sdk::attr::AttrContext) -> ::genet_sdk::attr::AttrClass {
                use ::genet_sdk::attr::AttrClass;
                AttrClass::builder(&ctx.path).build()
            }
        }
    };
    tokens.into()
}

fn parse_ident(ident: &Ident) -> String {
    to_camel_case(&ident.to_string())
        .trim_right_matches("Attr")
        .into()
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