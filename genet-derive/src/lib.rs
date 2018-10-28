#![recursion_limit = "256"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate genet_abi;
extern crate inflector;
extern crate proc_macro;
extern crate proc_macro2;

use genet_abi::attr::AttrContext;
use inflector::cases::{camelcase::to_camel_case, titlecase::to_title_case};
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

    let mut fields_ident = Vec::new();
    let mut fields_ctx = Vec::new();
    let mut fields_aliases = Vec::new();

    if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            for field in f.named {
                if let Some(ident) = field.ident {
                    let meta = parse_attrs(&field.attrs);
                    let id = to_camel_case(&ident.to_string());
                    let typ = meta.typ;
                    let name = if meta.name.is_empty() {
                        to_title_case(&ident.to_string())
                    } else {
                        meta.name
                    };
                    let desc = meta.description;
                    fields_ctx.push(quote!{
                        AttrContext{
                            path: format!("{}.{}", ctx.path, #id),
                            typ: #typ.into(),
                            name: #name.into(),
                            description: #desc.into(),
                            ..Default::default()
                        }
                    });
                    fields_ident.push(ident);

                    for name in meta.aliases {
                        fields_aliases.push(quote!{
                            (format!("{}", #name), format!("{}.{}", ctx.path, #id))
                        });
                    }
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
    let fields_ident2 = fields_ident.clone();
    let tokens = quote!{
        impl ::genet_sdk::attr::AttrNode for #ident {
            fn init(&mut self, ctx: &::genet_sdk::attr::AttrContext)
                -> ::genet_sdk::attr::AttrList {
                use ::genet_sdk::attr::{Attr, AttrNode, AttrList, AttrContext, AttrClass, AttrNodeType};
                use ::genet_sdk::fixed::Fixed;

                let class = Fixed::new(
                    AttrClass::builder(ctx.path.clone())
                    .typ(ctx.typ.clone())
                    .build()
                );
                let mut attrs = Vec::new();
                let mut children = Vec::new();
                let mut aliases = vec![
                    #( #fields_aliases ),*
                ];

                #(
                    {
                        let ctx = #fields_ctx;
                        let attr : &mut AttrNode = &mut self.#fields_ident;
                        let mut child = attr.init(&ctx);
                        if attr.node_type() == AttrNodeType::Static {
                            attrs.push(Attr::builder(child.class.clone()).build());
                        }
                        children.push(child.class.clone());
                        children.append(&mut child.children);
                        attrs.append(&mut child.attrs);
                        aliases.append(&mut child.aliases);
                    }
                )*

                AttrList {
                    class,
                    children,
                    attrs,
                    aliases,
                }
            }

            fn node_type(&self) -> ::genet_sdk::attr::AttrNodeType {
                ::genet_sdk::attr::AttrNodeType::Static
            }

            fn byte_size(&self) -> usize {
                use ::genet_sdk::attr::{AttrNodeType, AttrNode};
                let mut size = 0;

                #(
                    {
                        let attr : &AttrNode = &self.#fields_ident2;
                        if attr.node_type() == AttrNodeType::Static {
                            size += attr.byte_size();
                        }
                    }
                )*

                size
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
    pub aliases: Vec<String>,
}

fn parse_attrs(attrs: &[Attribute]) -> AttrMetadata {
    let mut typ = String::new();
    let mut aliases = Vec::new();
    let mut docs = String::new();
    for attr in attrs {
        if let Some(meta) = attr.interpret_meta() {
            let name = meta.name().to_string();
            match (name.as_str(), meta) {
                (
                    "doc",
                    Meta::NameValue(MetaNameValue {
                        lit: Lit::Str(lit_str),
                        ..
                    }),
                ) => {
                    docs += &lit_str.value();
                    docs += "\n";
                }
                ("genet", Meta::List(list)) => {
                    for item in list.nested {
                        if let NestedMeta::Meta(meta) = item {
                            let name = meta.name().to_string();
                            if let Meta::NameValue(MetaNameValue {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = meta
                            {
                                match name.as_str() {
                                    "typ" => {
                                        typ = lit_str.value().to_string();
                                    }
                                    "alias" => {
                                        aliases.push(lit_str.value().to_string());
                                    }
                                    _ => panic!("unsupported attribute: {}", name),
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
        aliases,
    }
}
