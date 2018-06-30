use frame::Frame;
use genet_abi::{layer::Layer, ptr::MutPtr, result::Result};
use std::fmt::Debug;

pub trait Output: Send + Debug {
    fn write(&mut self, frames: Option<&[&Frame]>) -> Result<()>;
}

pub trait Input: Send + Debug {
    fn read(&mut self) -> Result<Vec<MutPtr<Layer>>>;
}
