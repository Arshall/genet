use libc::c_char;
use std::ffi::CString;

pub mod attr;
pub mod embed;
pub mod frame;
pub mod layer;
pub mod session;
pub mod token;

#[no_mangle]
pub unsafe extern "C" fn genet_str_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        CString::from_raw(ptr);
    }
}
