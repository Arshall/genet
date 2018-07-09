#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use genet_sdk::prelude::*;
use std::collections::HashMap;

struct TcpWorker {}

impl Worker for TcpWorker {
    fn analyze(&mut self, parent: &mut Layer) -> Result<Status> {
        if parent.id() == token!("tcp") {
            parent.add_attr(Attr::new(&STREAM_ATTR, 0..0));
            Ok(Status::Done(vec![]))
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct TcpDissector {}

impl Dissector for TcpDissector {
    fn new_worker(&self, typ: &str, _ctx: &Context) -> Option<Box<Worker>> {
        if typ == "serial" {
            Some(Box::new(TcpWorker {}))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref STREAM_ATTR: Ptr<AttrClass> = AttrBuilder::new("tcp.stream")
        .typ("@novalue")
        .decoder(decoder::Ranged(decoder::UInt8(), 0..0).map(|v| v))
        .build();
}

genet_dissectors!(TcpDissector {});
