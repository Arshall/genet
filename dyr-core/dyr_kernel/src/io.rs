use dyr_ffi::{layer::Layer, ptr::MutPtr};
use frame::Frame;
use std::fmt::Debug;
use std::io::Result;

pub trait Output: Send + Debug {
    fn write(&self, frames: Option<&[&Frame]>) -> Result<()>;
}

pub trait InputCallback: Send {
    fn write(&self, result: Result<Vec<MutPtr<Layer>>>) -> bool;
}

pub trait Input: Send + Debug {
    fn start(&self, callback: Box<InputCallback>);
}
