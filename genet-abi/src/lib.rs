extern crate bincode;
extern crate fnv;
extern crate libc;
extern crate parking_lot;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

pub mod attr;
pub mod cast;
pub mod context;
pub mod decoder;
pub mod env;
pub mod error;
pub mod field;
pub mod file;
pub mod fixed;
pub mod layer;
pub mod metadata;
pub mod reader;
pub mod result;
pub mod slice;
pub mod token;
pub mod variant;
pub mod writer;

mod string;
mod vec;
