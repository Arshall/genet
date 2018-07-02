#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use genet_sdk::{
    attr::{Attr, AttrBuilder, AttrClass},
    context::Context,
    decoder::{self, Map},
    dissector::{Dissector, Status, Worker},
    layer::{Layer, LayerBuilder, LayerClass},
    ptr::Ptr,
    result::Result,
    slice::SliceIndex,
};
use std::collections::HashMap;

struct IPv6Worker {}

impl Worker for IPv6Worker {
    fn analyze(&mut self, parent: &mut Layer) -> Result<Status> {
        if parent.id() == token!("eth") && parent.attr(token!("eth.type.ipv6")).is_some() {
            let mut layer = Layer::new(&IPV6_CLASS, parent.data().get(14..)?);
            Ok(Status::Done(vec![layer]))
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct IPv6Dissector {}

impl Dissector for IPv6Dissector {
    fn new_worker(&self, typ: &str, _ctx: &Context) -> Option<Box<Worker>> {
        if typ == "parallel" {
            Some(Box::new(IPv6Worker {}))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref IPV6_CLASS: Ptr<LayerClass> = LayerBuilder::new("ipv6")
        .alias("_.src", "ipv6.src")
        .alias("_.dst", "ipv6.dst")
        .header(Attr::new(&VERSION_ATTR, 0..1))
        .build();
    static ref VERSION_ATTR: Ptr<AttrClass> = AttrBuilder::new("ipv6.version")
        .decoder(decoder::UInt8().map(|v| v >> 4))
        .build();
    static ref SRC_ATTR: Ptr<AttrClass> = AttrBuilder::new("ipv6.src")
        .typ("@ipv6:addr")
        .decoder(decoder::Slice())
        .build();
    static ref DST_ATTR: Ptr<AttrClass> = AttrBuilder::new("ipv6.dst")
        .typ("@ipv6:addr")
        .decoder(decoder::Slice())
        .build();
    static ref PROTO_MAP: HashMap<u64, Ptr<AttrClass>> = hashmap!{
        0x02 => AttrBuilder::new("ipv6.protocol.igmp").build(),
        0x06 => AttrBuilder::new("ipv6.protocol.tcp").build(),
        0x11 => AttrBuilder::new("ipv6.protocol.udp").build(),
        0x3a => AttrBuilder::new("ipv6.protocol.icmp").build(),
    };
}
genet_dissectors!(IPv6Dissector {});
