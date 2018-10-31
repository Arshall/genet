use attr::{AttrClass, AttrContext, AttrField, AttrFieldType, AttrList};
use fixed::Fixed;
use std::ops::Deref;

pub struct Detach<T: AttrField> {
    attr: T,
    class: Option<Fixed<AttrClass>>,
}

impl<T: AttrField> AttrField for Detach<T> {
    fn init(&mut self, ctx: &AttrContext) -> AttrList {
        let child = self.attr.init(ctx);
        self.class = Some(child.class.clone());
        child
    }

    fn node_type(&self) -> AttrFieldType {
        AttrFieldType::Detached
    }

    fn bit_size(&self) -> usize {
        self.attr.bit_size()
    }
}

impl<T: AttrField> Deref for Detach<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.attr
    }
}

impl<T: AttrField + Default> Default for Detach<T> {
    fn default() -> Self {
        Self {
            attr: T::default(),
            class: None,
        }
    }
}

impl<T: AttrField> AsRef<Fixed<AttrClass>> for Detach<T> {
    fn as_ref(&self) -> &Fixed<AttrClass> {
        self.class.as_ref().unwrap()
    }
}
