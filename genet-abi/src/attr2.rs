use attr::AttrClass;
use fixed::Fixed;
use result::Result;
use slice::{ByteSlice, TryGet};
use std::{mem::size_of, ops::Deref};

fn as_bool_placeholder(_data: &ByteSlice) -> Result<bool> {
    Ok(false)
}

fn as_u64_placeholder(_data: &ByteSlice) -> Result<u64> {
    Ok(0)
}

fn as_i64_placeholder(_data: &ByteSlice) -> Result<i64> {
    Ok(0)
}

fn as_f64_placeholder(_data: &ByteSlice) -> Result<f64> {
    Ok(0.0)
}

fn as_str_placeholder(_data: &ByteSlice) -> Result<Box<str>> {
    Ok(String::new().into())
}

fn as_slice_placeholder(data: &ByteSlice) -> Result<ByteSlice> {
    Ok(data.clone())
}

fn as_buffer_placeholder(_data: &ByteSlice) -> Result<Box<[u8]>> {
    Ok(Vec::new().into())
}

trait AttrType {
    fn as_bool() -> fn(&ByteSlice) -> Result<bool> {
        as_bool_placeholder
    }

    fn as_u64() -> fn(&ByteSlice) -> Result<u64> {
        as_u64_placeholder
    }

    fn as_i64() -> fn(&ByteSlice) -> Result<i64> {
        as_i64_placeholder
    }

    fn as_f64() -> fn(&ByteSlice) -> Result<f64> {
        as_f64_placeholder
    }

    fn as_str() -> fn(&ByteSlice) -> Result<Box<str>> {
        as_str_placeholder
    }

    fn as_slice() -> fn(&ByteSlice) -> Result<ByteSlice> {
        as_slice_placeholder
    }

    fn as_buffer() -> fn(&ByteSlice) -> Result<Box<[u8]>> {
        as_buffer_placeholder
    }    
}

trait AttrNode: AttrType {
    fn children(&self) -> Vec<AttrClass> {
        vec![]
    }
}

trait Static: AttrNode {
    fn byte_size(&self) -> usize;
}

struct AttrField<T: AttrNode> {
    attr: T,
    class: Fixed<AttrClass>,
}

impl<T: AttrNode> AttrField<T> {
    fn new(attr: T) -> Self {
        Self {
            attr,
            class: Fixed::new(AttrClass::builder("bool").typ("@bool").build()),
        }
    }

    fn class(&self) -> &Fixed<AttrClass> {
        &self.class
    }
}

impl<T: AttrNode> Deref for AttrField<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.attr
    }
}

impl<T: AttrNode> AsRef<Fixed<AttrClass>> for AttrField<T> {
    fn as_ref(&self) -> &Fixed<AttrClass> {
        &self.class
    }
}

struct StaticAttrField<T: Static> {
    byte_size: usize,
    field: AttrField<T>,
}

impl<T: Static> StaticAttrField<T> {
    fn new(attr: T) -> Self {
        Self {
            byte_size: attr.byte_size(),
            field: AttrField::new(attr),
        }
    }

    fn class(&self) -> &Fixed<AttrClass> {
        self.field.class()
    }

    fn byte_size(&self) -> usize {
        self.byte_size
    }
}

struct EthAttr {
    plen: StaticAttrField<Uint8>,
    nrn: AttrField<Uint8>,
}

struct Uint8 {}

impl AttrType for Uint8 {
    fn as_u64() -> fn(&ByteSlice) -> Result<u64> {
        |data| Ok(data.try_get(0)? as u64)
    }
}

impl AttrNode for Uint8 {}

impl Static for Uint8 {
    fn byte_size(&self) -> usize {
        1
    }
}