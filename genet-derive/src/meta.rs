use syn::{Attribute, Lit, Meta, MetaNameValue, NestedMeta};

pub struct AttrMetadata {
    pub typ: String,
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
}

impl AttrMetadata {
    pub fn parse(attrs: &[Attribute]) -> AttrMetadata {
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
}
