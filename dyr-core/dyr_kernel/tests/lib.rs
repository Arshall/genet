extern crate dyr_ffi;
extern crate dyr_kernel;
extern crate libloading;
mod data;

#[macro_use]
extern crate dyr_sdk;

use dyr_kernel::profile::Profile;
use dyr_kernel::session::{Callback, Event, Session};
use dyr_sdk::layer::{Layer, LayerClass};
use dyr_sdk::ptr::MutPtr;
use std::iter;

struct SessionCallback {}

impl Callback for SessionCallback {
    fn on_event(&self, event: Event) {
        if let Event::Frames(len) = event {
            println!("{:?}", len);
            if len == 400000 {
                ::std::process::exit(0);
            }
        }
    }
}

#[test]
fn session() {
    let mut profile = Profile::new();

    let libdir = ::std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join("../examples");

    profile
        .load_library(libdir.join("libeth.dylib").to_str().unwrap())
        .expect("failed to load dylib");

    let mut session = Session::new(profile, SessionCallback {});

    let class = LayerClass::new(token!("[eth]"));
    for _ in 0..4000 {
        let frames = iter::repeat(data::tcp_ipv4_pcap())
            .take(100)
            .map(|data| MutPtr::new(Layer::new(&class, data)))
            .collect::<Vec<_>>();
        session.push_frames(frames);
    }

    loop {}
}
