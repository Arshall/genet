//! This crate provides APIs for creating plugin pacakges for
//! [genet](https://genet.app/).

extern crate byteorder;
extern crate genet_abi;
extern crate lazy_static;

pub mod attr;
pub mod cast;
pub mod context;
pub mod decoder;
pub mod error;
pub mod field;
pub mod file;
pub mod fixed;
pub mod helper;
pub mod layer;
pub mod prelude;
pub mod reader;
pub mod result;
pub mod slice;
pub mod token;
pub mod variant;
pub mod writer;

#[doc(hidden)]
pub use lazy_static::*;
