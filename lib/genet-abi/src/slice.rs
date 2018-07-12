use std::{
    io::{Error, ErrorKind, Result},
    marker::PhantomData,
    mem,
    ops::{Deref, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    slice,
};

pub trait SliceIndex<T> {
    type Output;
    fn get(&self, index: T) -> Result<Self::Output>;
}

macro_rules! impl_slice_index {
    ( $( $x:ty ), * ) => {
        $(
            impl SliceIndex<$x> for Slice {
                type Output = Slice;
                fn get(&self, index: $x) -> Result<Slice> {
                    <[u8]>::get(self, index)
                        .map(|s| unsafe { Slice::from_raw_parts(s.as_ptr(), s.len()) })
                        .ok_or_else(|| Error::new(ErrorKind::Other, "out of bounds"))
                }
            }
        )*
    };
}

impl_slice_index!(
    Range<usize>,
    RangeFrom<usize>,
    RangeFull,
    RangeInclusive<usize>,
    RangeTo<usize>,
    RangeToInclusive<usize>
);

impl SliceIndex<usize> for Slice {
    type Output = u8;

    fn get(&self, index: usize) -> Result<u8> {
        <[u8]>::get(self, index)
            .map(|v| *v)
            .ok_or_else(|| Error::new(ErrorKind::Other, "out of bounds"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slice(&'static [u8]);

impl Slice {
    pub fn new() -> Slice {
        Slice(&[])
    }

    pub unsafe fn from_raw_parts(data: *const u8, len: usize) -> Slice {
        Slice(slice::from_raw_parts(data, len))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

impl From<&'static [u8]> for Slice {
    fn from(data: &'static [u8]) -> Self {
        Slice(data)
    }
}

impl From<Box<[u8]>> for Slice {
    fn from(data: Box<[u8]>) -> Self {
        let s = unsafe { Slice::from_raw_parts(data.as_ptr(), data.len()) };
        mem::forget(data);
        s
    }
}

impl From<Vec<u8>> for Slice {
    fn from(data: Vec<u8>) -> Self {
        Slice::from(data.into_boxed_slice())
    }
}

impl Deref for Slice {
    type Target = [u8];

    fn deref(&self) -> &'static [u8] {
        self.0
    }
}

impl AsRef<[u8]> for Slice {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[repr(C)]
struct SafeSlice<'a, T: 'a> {
    ptr: *const T,
    len: u64,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> SafeSlice<'a, T> {
    pub fn new(data: &'a [T]) -> SafeSlice<'a, T> {
        Self {
            ptr: data.as_ptr(),
            len: data.len() as u64,
            phantom: PhantomData,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(&*self.ptr, self.len as usize) }
    }
}

impl<'a, T: 'a> Deref for SafeSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}
