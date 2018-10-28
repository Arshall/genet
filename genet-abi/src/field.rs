use attr::{AttrClass, AttrContext, AttrList, AttrNode, AttrNodeType};
use fixed::Fixed;
use std::ops::Deref;

pub struct Detach<T: AttrNode> {
    attr: T,
    class: Option<Fixed<AttrClass>>,
}

impl<T: AttrNode> AttrNode for Detach<T> {
    fn init(&mut self, ctx: &AttrContext) -> AttrList {
        let child = self.attr.init(ctx);
        self.class = Some(child.class.clone());
        child
    }

    fn node_type(&self) -> AttrNodeType {
        AttrNodeType::Dynamic
    }

    fn bit_size(&self) -> usize {
        self.attr.bit_size()
    }
}

impl<T: AttrNode> Deref for Detach<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.attr
    }
}

impl<T: AttrNode + Default> Default for Detach<T> {
    fn default() -> Self {
        Self {
            attr: T::default(),
            class: None,
        }
    }
}

impl<T: AttrNode> AsRef<Fixed<AttrClass>> for Detach<T> {
    fn as_ref(&self) -> &Fixed<AttrClass> {
        self.class.as_ref().unwrap()
    }
}
