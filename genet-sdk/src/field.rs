//! Attribute field types.

use cast;
pub use genet_abi::field::*;
use genet_abi::{
    attr::{AttrClass, AttrContext, AttrList, AttrNode, AttrNodeType},
    fixed::Fixed,
};

#[derive(Default)]
pub struct Uint8 {}

impl AttrNode for Uint8 {
    fn init(&mut self, ctx: &AttrContext) -> AttrList {
        AttrList {
            class: Fixed::new(
                AttrClass::builder(ctx.path.clone())
                    .cast(cast::UInt8())
                    .typ(ctx.typ.clone())
                    .name(ctx.name)
                    .description(ctx.description)
                    .build(),
            ),
            children: Vec::new(),
            attrs: Vec::new(),
            aliases: Vec::new(),
        }
    }

    fn node_type(&self) -> AttrNodeType {
        AttrNodeType::Static
    }

    fn bit_size(&self) -> usize {
        8
    }
}
