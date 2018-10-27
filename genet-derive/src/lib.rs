#![recursion_limit = "128"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate genet_abi;
extern crate inflector;
extern crate proc_macro;
extern crate proc_macro2;

use genet_abi::attr::AttrContext;
use inflector::cases::camelcase::to_camel_case;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, Lit, Meta, MetaNameValue, NestedMeta, Type,
    TypePath,
};

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
    
    */
    let mut fields_id = Vec::new();
    let mut fields_name = Vec::new();
    let mut fields_desc = Vec::new();

    if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            for field in f.named {
                if let Some(ident) = field.ident {
                    fields_id.push(to_camel_case(&ident.to_string()));
                    let mata = parse_attrs(&field.attrs);
                    fields_name.push(mata.name);
                    fields_desc.push(mata.description);
                }

                /*
                if let Type::Path(p) = field.ty {
                    println!(
                        "{:?}",
                        p == syn::parse_str::<TypePath>("PayloadLen").unwrap()
                    );
                }
                */
            }
        }
    }

    let ident = input.ident;
    let tokens = quote!{
        impl ::genet_sdk::attr::AttrNode for #ident {
            fn init(&mut self, ctx: &::genet_sdk::attr::AttrContext) -> ::genet_sdk::attr::AttrClass {
                use ::genet_sdk::attr::{AttrClass, AttrContext};

                #(
                    {
                        let ctx = AttrContext{
                            path: format!("{}.{}", ctx.path, #fields_id),
                            name: #fields_name.into(),
                            description: #fields_desc.into(),
                            ..Default::default()
                        };
                    }
                )*

                AttrClass::builder(&ctx.path).build()
            }
        }
    };

    println!("{}", tokens.to_string());

    tokens.into()
}

struct AttrMetadata {
    pub typ: String,
    pub name: String,
    pub description: String,
}

fn parse_attrs(attrs: &[Attribute]) -> AttrMetadata {
    let mut typ = String::new();
    let mut docs = String::new();
    for attr in attrs {
        if let Some(meta) = attr.interpret_meta() {
            match meta.name().to_string().as_str() {
                "doc" => {
                    if let Meta::NameValue(MetaNameValue {
                        lit: Lit::Str(lit_str),
                        ..
                    }) = meta
                    {
                        docs += &lit_str.value();
                        docs += "\n";
                    }
                }
                "genet" => {
                    if let Meta::List(list) = meta {
                        for item in list.nested {
                            if let NestedMeta::Meta(meta) = item {
                                let name = meta.name().to_string();
                                if let Meta::NameValue(MetaNameValue {
                                    lit: Lit::Str(lit_str),
                                    ..
                                }) = meta
                                {
                                    println!("@@ {} {}", name, lit_str.value().to_string());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let mut lines = docs.split('\n').map(|line| line.trim());
    let name = lines.next().unwrap_or("");
    let description = lines.fold(String::new(), |acc, x| acc + x + "\n");

    AttrMetadata {
        typ: typ.trim().into(),
        name: name.into(),
        description: description.trim().into(),
    }
}
