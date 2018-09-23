use frame::Frame;
use genet_abi::{attr::Attr, layer::Layer, token::Token};
use genet_napi::napi::{
    CallbackInfo, Env, HandleScope, PropertyAttributes, PropertyDescriptor, Result, Status, Value,
    ValueRef,
};
use std::{ptr, rc::Rc};

#[no_mangle]
pub unsafe extern "C" fn genet_frame_index(frame: *const Frame) -> u32 {
    (*frame).index()
}

#[no_mangle]
pub unsafe extern "C" fn genet_frame_layers(
    frame: *const Frame,
    len: *mut u32,
) -> *const *const Layer {
    let layers = (*frame).layers();
    *len = layers.len() as u32;
    layers.as_ptr() as *const *const Layer
}

#[no_mangle]
pub unsafe extern "C" fn genet_frame_tree_indices(frame: *const Frame, len: *mut u32) -> *const u8 {
    let indices = (*frame).tree_indices();
    *len = indices.len() as u32;
    indices.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn genet_frame_attr(frame: *const Frame, id: Token) -> *const Attr {
    let frame = &*frame;
    if let Some(attr) = frame.attr(id) {
        attr
    } else {
        ptr::null()
    }
}

pub fn wrapper(env: &Env) -> Rc<ValueRef> {
    fn ctor<'env>(env: &'env Env, info: &'env CallbackInfo) -> Result<&'env Value> {
        env.get_null()
    }

    fn frame_index<'env>(env: &'env Env, info: &'env CallbackInfo) -> Result<&'env Value> {
        let frame = env.unwrap::<Frame>(info.this())?;
        env.create_uint32(frame.index())
    }

    fn frame_tree_indices<'env>(env: &'env Env, info: &'env CallbackInfo) -> Result<&'env Value> {
        let frame = env.unwrap::<Frame>(info.this())?;
        let indices = frame.tree_indices();
        let array = env.create_array(indices.len())?;
        for i in 0..indices.len() {
            env.set_element(array, i as u32, env.create_uint32(indices[i] as u32)?)?;
        }
        Ok(array)
    }

    let class = env
        .define_class(
            "Frame",
            ctor,
            &vec![
                PropertyDescriptor::new_property(
                    env,
                    "index",
                    PropertyAttributes::Default,
                    frame_index,
                    false,
                ),
                PropertyDescriptor::new_property(
                    env,
                    "treeIndices",
                    PropertyAttributes::Default,
                    frame_tree_indices,
                    false,
                ),
            ],
        ).unwrap();
    env.create_ref(class)
}
