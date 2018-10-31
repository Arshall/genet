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

pub struct Node<T: AttrField, U: AttrField> {
    attr: T,
    children: U,
}

impl<T: AttrField, U: AttrField> Node<T, U> {
    pub fn new(attr: T, children: U) -> Self {
        Self { attr, children }
    }

    pub fn attrs(&self) -> &U {
        &self.children
    }
}

impl<T: AttrField, U: AttrField> AttrField for Node<T, U> {
    fn init(&mut self, ctx: &AttrContext) -> AttrList {
        let mut attr = self.attr.init(ctx);
        let mut child = self.children.init(ctx);
        attr.children.append(&mut child.children);
        attr.attrs.append(&mut child.attrs);
        attr.aliases.append(&mut child.aliases);
        attr
    }

    fn node_type(&self) -> AttrFieldType {
        AttrFieldType::Attached
    }

    fn bit_size(&self) -> usize {
        self.attr.bit_size()
    }
}

impl<T: AttrField + Default, U: AttrField + Default> Default for Node<T, U> {
    fn default() -> Self {
        Self {
            attr: T::default(),
            children: U::default(),
        }
    }
}

impl<T: AttrField, U: AttrField> Deref for Node<T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.attr
    }
}
