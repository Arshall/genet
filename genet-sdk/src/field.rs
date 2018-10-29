//! Attribute field types.

use cast;
use genet_abi::attr::{AttrContext, AttrList, AttrNode, AttrNodeType};
pub use genet_abi::field::Detach;

#[derive(Default)]
pub struct Uint8 {}

impl AttrNode for Uint8 {
    fn init(&mut self, ctx: &AttrContext) -> AttrList {
        AttrList::from_ctx(ctx, cast::UInt8())
    }

    fn node_type(&self) -> AttrNodeType {
        AttrNodeType::Static
    }

    fn bit_size(&self) -> usize {
        8
    }
}
