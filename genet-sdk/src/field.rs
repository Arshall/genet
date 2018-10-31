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

macro_rules! impl_pad {
    ($id:ident, $size:expr) => {
        #[derive(Default)]
        pub struct $id {}

        impl AttrNode for $id {
            fn init(&mut self, ctx: &AttrContext) -> AttrList {
                AttrList::from_ctx(ctx, cast::UInt8())
            }

            fn node_type(&self) -> AttrNodeType {
                AttrNodeType::Padding
            }

            fn bit_size(&self) -> usize {
                $size
            }
        }
    };
}

impl_pad!(BytePad1, 8 * 1);
impl_pad!(BytePad2, 8 * 2);
impl_pad!(BytePad3, 8 * 3);
impl_pad!(BytePad4, 8 * 4);
impl_pad!(BytePad5, 8 * 5);
impl_pad!(BytePad6, 8 * 6);
impl_pad!(BytePad7, 8 * 7);
impl_pad!(BytePad8, 8 * 8);

pub struct BytePad {
    len: usize,
}

impl BytePad {
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl AttrNode for BytePad {
    fn init(&mut self, ctx: &AttrContext) -> AttrList {
        AttrList::from_ctx(ctx, cast::UInt8())
    }

    fn node_type(&self) -> AttrNodeType {
        AttrNodeType::Padding
    }

    fn bit_size(&self) -> usize {
        self.len * 8
    }
}
