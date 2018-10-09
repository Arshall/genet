use context::Context;
use error::Error;
use fixed::MutFixed;
use layer::{Layer, LayerStack, MutLayer};
use result::Result;
use std::{mem, ptr};

/// Execution type.
#[repr(u8)]
#[derive(Clone, PartialEq, Debug)]
pub enum ExecType {
    ParallelSync = 0,
    SerialSync = 1,
    SerialAsync = 2,
}

/// Dissection status.
#[derive(Debug)]
pub enum Status {
    Done(Vec<Layer>),
    Skip,
}

/// Decoder worker trait.
pub trait Worker {
    fn decode(&mut self, &mut Context, &LayerStack, &mut MutLayer) -> Result<Status>;
}

#[repr(C)]
pub struct WorkerBox {
    decode: extern "C" fn(
        *mut WorkerBox,
        *mut Context,
        *const *const Layer,
        u64,
        *mut MutLayer,
        *mut MutFixed<Layer>,
        *mut Error,
    ) -> u8,
    worker: *mut Box<Worker>,
}

const MAX_CHILDREN: usize = <u8>::max_value() as usize - 2;

impl WorkerBox {
    fn new(worker: Box<Worker>) -> WorkerBox {
        Self {
            decode: abi_decode,
            worker: Box::into_raw(Box::new(worker)),
        }
    }

    pub fn decode(
        &mut self,
        ctx: &mut Context,
        layers: &[MutFixed<Layer>],
        layer: &mut MutLayer,
        results: &mut Vec<MutFixed<Layer>>,
    ) -> Result<bool> {
        let mut children: [MutFixed<Layer>; MAX_CHILDREN];
        unsafe {
            children = mem::uninitialized();
        }
        let stack = layers.as_ptr() as *const *const Layer;
        let mut error = Error::new("");
        let result = (self.decode)(
            self,
            ctx,
            stack,
            layers.len() as u64,
            layer,
            children.as_mut_ptr(),
            &mut error,
        );
        if result > 1 {
            let len = result as usize - 2;
            unsafe {
                let grow = len.saturating_sub(results.capacity());
                results.reserve(grow);
                results.set_len(len);
                ptr::copy(children.as_ptr(), results.as_mut_ptr(), len);
            }
            Ok(true)
        } else if result > 0 {
            Ok(false)
        } else {
            Err(Box::new(error))
        }
    }
}

extern "C" fn abi_decode(
    worker: *mut WorkerBox,
    ctx: *mut Context,
    layers: *const *const Layer,
    len: u64,
    layer: *mut MutLayer,
    children: *mut MutFixed<Layer>,
    error: *mut Error,
) -> u8 {
    let worker = unsafe { &mut *((*worker).worker) };
    let ctx = unsafe { &mut (*ctx) };
    let mut layer = MutLayer::new(unsafe { &mut (*layer) });
    let stack = unsafe { LayerStack::new(layers, len as usize) };
    match worker.decode(ctx, &stack, &mut layer) {
        Ok(stat) => match stat {
            Status::Done(layers) => {
                let len = 2u8.saturating_add(layers.len() as u8);
                for (i, layer) in layers.into_iter().take(MAX_CHILDREN).enumerate() {
                    unsafe { ptr::write(children.offset(i as isize), MutFixed::new(layer)) };
                }
                len
            }
            Status::Skip => 1,
        },
        Err(err) => {
            unsafe {
                ptr::write(error, Error::new(err.description()));
            }
            0
        }
    }
}

/// Decoder trait.
pub trait Decoder: DecoderClone + Send {
    fn new_worker(&self, &Context) -> Box<Worker>;
    fn execution_type(&self) -> ExecType;
}

pub trait DecoderClone {
    fn clone_box(&self) -> Box<Decoder>;
    fn into_box(self) -> Box<Decoder>;
}

impl<T> DecoderClone for T
where
    T: 'static + Decoder + Clone,
{
    fn clone_box(&self) -> Box<Decoder> {
        Box::new(self.clone())
    }

    fn into_box(self) -> Box<Decoder> {
        Box::new(self)
    }
}

impl Clone for Box<Decoder> {
    fn clone(&self) -> Box<Decoder> {
        self.clone_box()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DecoderBox {
    new_worker: extern "C" fn(*mut DecoderBox, *const Context) -> WorkerBox,
    execution_type: extern "C" fn(*const DecoderBox) -> u8,
    decoder: *mut Box<Decoder>,
}

unsafe impl Send for DecoderBox {}

impl DecoderBox {
    pub fn new<T: 'static + Decoder>(diss: T) -> DecoderBox {
        let diss: Box<Decoder> = Box::new(diss);
        Self {
            new_worker: abi_new_worker,
            execution_type: abi_execution_type,
            decoder: Box::into_raw(Box::new(diss)),
        }
    }

    pub fn new_worker(&mut self, ctx: &Context) -> WorkerBox {
        (self.new_worker)(self, ctx)
    }

    pub fn execution_type(&self) -> ExecType {
        match (self.execution_type)(self) {
            1 => ExecType::SerialSync,
            2 => ExecType::SerialAsync,
            _ => ExecType::ParallelSync,
        }
    }
}

extern "C" fn abi_new_worker(diss: *mut DecoderBox, ctx: *const Context) -> WorkerBox {
    let diss = unsafe { &mut *((*diss).decoder) };
    let ctx = unsafe { &(*ctx) };
    WorkerBox::new(diss.new_worker(ctx))
}

extern "C" fn abi_execution_type(diss: *const DecoderBox) -> u8 {
    let diss = unsafe { &*((*diss).decoder) };
    diss.execution_type() as u8
}

#[cfg(test)]
mod tests {
    use context::Context;
    use decoder::{Decoder, DecoderBox, ExecType, Status, Worker};
    use fixed::Fixed;
    use fnv::FnvHashMap;
    use layer::{Layer, LayerClass, LayerStack, MutLayer};
    use result::Result;
    use slice::ByteSlice;
    use token::Token;

    #[test]
    fn decode() {
        struct TestWorker {}

        impl Worker for TestWorker {
            fn decode(
                &mut self,
                _ctx: &mut Context,
                _stack: &LayerStack,
                _layer: &mut MutLayer,
            ) -> Result<Status> {
                let class = Fixed::new(LayerClass::builder(Token::from(1234)).build());
                let layer = Layer::new(class, ByteSlice::new());
                Ok(Status::Done(vec![layer]))
            }
        }

        #[derive(Clone)]
        struct TestDecoder {}

        impl Decoder for TestDecoder {
            fn new_worker(&self, _ctx: &Context) -> Box<Worker> {
                Box::new(TestWorker {})
            }

            fn execution_type(&self) -> ExecType {
                ExecType::ParallelSync
            }
        }

        let mut ctx = Context::new(FnvHashMap::default());
        let mut diss = DecoderBox::new(TestDecoder {});
        let mut worker = diss.new_worker(&ctx);

        let class = Fixed::new(LayerClass::builder(Token::null()).build());
        let mut layer = Layer::new(class, ByteSlice::new());
        let mut layer = MutLayer::new(&mut layer);
        let mut results = Vec::new();

        assert_eq!(
            worker
                .decode(&mut ctx, &[], &mut layer, &mut results)
                .unwrap(),
            true
        );
        assert_eq!(results.len(), 1);
        let child = &results[0];
        assert_eq!(child.id(), Token::from(1234));
    }
}
