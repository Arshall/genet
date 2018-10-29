#![recursion_limit = "512"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate genet_abi;
extern crate inflector;
extern crate proc_macro;

use inflector::cases::{camelcase::to_camel_case, titlecase::to_title_case};
use proc_macro::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields};

mod meta;
use meta::AttrMetadata;

#[proc_macro_derive(Attr, attributes(genet))]
pub fn derive_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(s) => parse_struct(&input, &s),
        _ => panic!("Attr derive supports struct types only"),
    }
}

fn parse_struct(input: &DeriveInput, s: &DataStruct) -> TokenStream {
    let mut fields_ident = Vec::new();
    let mut fields_ctx = Vec::new();
    let mut fields_aliases = Vec::new();

    if let Fields::Named(f) = &s.fields {
        for field in &f.named {
            if let Some(ident) = &field.ident {
                let meta = AttrMetadata::parse(&field.attrs);
                let id = ident.to_string();
                let id = if id == "_self" {
                    String::new()
                } else {
                    format!(".{}", to_camel_case(&ident.to_string()))
                };
                let typ = meta.typ;
                let name = if meta.name.is_empty() {
                    to_title_case(&ident.to_string())
                } else {
                    meta.name
                };
                let desc = meta.description;
                fields_ctx.push(quote!{
                    AttrContext{
                        path: format!("{}{}", ctx.path, #id),
                        typ: #typ.into(),
                        name: #name.into(),
                        description: #desc.into(),
                        ..Default::default()
                    }
                });
                fields_ident.push(ident);

                for name in meta.aliases {
                    fields_aliases.push(quote!{
                        (format!("{}", #name), format!("{}{}", ctx.path, #id))
                    });
                }
            }
        }
    }

    let self_attrs = AttrMetadata::parse(&input.attrs);
    let self_name = self_attrs.name;
    let self_desc = self_attrs.description;

    let ident = &input.ident;
    let fields_ident2 = fields_ident.clone();
    let tokens = quote!{
        impl genet_sdk::attr::AttrNode for #ident {
            fn init(&mut self, ctx: &::genet_sdk::attr::AttrContext)
                -> genet_sdk::attr::AttrList {
                use genet_sdk::attr::{Attr, AttrNode, AttrList, AttrContext, AttrClass, AttrNodeType};
                use genet_sdk::fixed::Fixed;

                let mut class = None;
                let mut bit_offset = ctx.bit_offset;
                let mut attrs = Vec::new();
                let mut children = Vec::new();
                let mut aliases = vec![
                    #( #fields_aliases ),*
                ];

                #(
                    {
                        let subctx = #fields_ctx;
                        let merged;
                        let attr : &mut AttrNode = &mut self.#fields_ident;
                        let mut child = attr.init(if subctx.path == ctx.path {
                            merged = subctx.clone().merge(ctx.clone());
                            &merged
                        } else {
                            &subctx
                        });
                        match attr.node_type() {
                            AttrNodeType::Static => {
                                if subctx.path == ctx.path {
                                    class = Some(child.class.clone());
                                } else {
                                    let size = attr.bit_size();
                                    attrs.push(
                                        Attr::builder(child.class.clone())
                                            .bit_range(0, bit_offset..(bit_offset + size))
                                            .build()
                                    );
                                    bit_offset += size;
                                }
                            },
                            AttrNodeType::Padding => {
                                bit_offset += attr.bit_size();
                            },
                            _ => {}
                        }

                        if (attr.node_type() != AttrNodeType::Padding) {
                            if subctx.path != ctx.path {
                                children.push(child.class.clone());
                                children.append(&mut child.children);
                                attrs.append(&mut child.attrs);
                                aliases.append(&mut child.aliases);
                            }
                        }
                    }
                )*

                AttrList {
                    class: class.unwrap_or_else(|| Fixed::new(
                        AttrClass::builder(ctx.path.clone())
                        .typ(ctx.typ.clone())
                        .name(if ctx.name.is_empty() {
                            #self_name
                        } else {
                            ctx.name
                        })
                        .description(if ctx.description.is_empty() {
                            #self_desc
                        } else {
                            ctx.description
                        })
                        .build()
                    )),
                    children,
                    attrs,
                    aliases,
                }
            }

            fn node_type(&self) -> genet_sdk::attr::AttrNodeType {
                genet_sdk::attr::AttrNodeType::Static
            }

            fn bit_size(&self) -> usize {
                use genet_sdk::attr::{AttrNodeType, AttrNode};
                let mut size = 0;

                #(
                    {
                        // TODO: ignore _self
                        let attr : &AttrNode = &self.#fields_ident2;
                        if attr.node_type() != AttrNodeType::Dynamic {
                            size += attr.bit_size();
                        }
                    }
                )*

                size
            }
        }
    };

    tokens.into()
}
