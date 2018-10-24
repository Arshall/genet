use attr::AttrClass;
use fixed::Fixed;
use result::Result;
use slice::{ByteSlice, TryGet};
use std::mem::size_of;

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

trait PxAttrType {
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

    fn byte_size(&self) -> Option<usize> {
        None
    }

    fn children(&self) -> &[Fixed<AttrClass>] {
        &[]
    }
}

impl PxAttrType for u8 {
    fn as_u64() -> fn(&ByteSlice) -> Result<u64> {
        |data| Ok(data.try_get(0)? as u64)
    }

    fn byte_size(&self) -> Option<usize> {
        Some(size_of::<u8>())
    }
}

impl PxAttrType for ByteSlice {
    fn as_slice() -> fn(&ByteSlice) -> Result<ByteSlice> {
        |data| Ok(data.clone())
    }
}
