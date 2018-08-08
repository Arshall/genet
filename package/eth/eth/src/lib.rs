#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use genet_sdk::prelude::*;
use std::collections::HashMap;

struct EthWorker {}

impl Worker for EthWorker {
    fn analyze(
        &mut self,
        _ctx: &mut Context,
        _stack: &LayerStack,
        parent: &mut Layer,
    ) -> Result<Status> {
        if parent.id() == token!("[link-1]") {
            let mut layer = Layer::new(&ETH_CLASS, parent.data());
            let len = LEN_ATTR_HEADER.try_get(&layer)?.try_into()?;
            if len <= 1500 {
                layer.add_attr(&LEN_ATTR_HEADER);
            } else {
                layer.add_attr(&TYPE_ATTR_HEADER);
            }
            if let Some((typ, attr)) = TYPE_MAP.get(&len) {
                layer.add_attr(Attr::new(attr, 12..14));
                let payload = parent.data().try_get(14..)?;
                layer.add_payload(Payload::new(payload, typ));
            }
            Ok(Status::Done(vec![layer]))
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct EthDissector {}

impl Dissector for EthDissector {
    fn new_worker(&self, typ: &str, _ctx: &Context) -> Option<Box<Worker>> {
        if typ == "parallel" {
            Some(Box::new(EthWorker {}))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref ETH_CLASS: LayerClass = LayerClass::builder("eth")
        .alias("_.src", "eth.src")
        .alias("_.dst", "eth.dst")
        .header(Attr::new(&SRC_ATTR, 0..6))
        .header(Attr::new(&DST_ATTR, 6..12))
        .build();
    static ref SRC_ATTR: AttrClass = AttrClass::builder("eth.src")
        .typ("@eth:mac")
        .decoder(decoder::ByteSlice())
        .build();
    static ref DST_ATTR: AttrClass = AttrClass::builder("eth.dst")
        .typ("@eth:mac")
        .decoder(decoder::ByteSlice())
        .build();
    static ref LEN_ATTR: AttrClass = AttrClass::builder("eth.len")
        .decoder(decoder::UInt16BE())
        .build();
    static ref TYPE_ATTR: AttrClass = AttrClass::builder("eth.type")
        .typ("@enum")
        .decoder(decoder::UInt16BE())
        .build();
    static ref LEN_ATTR_HEADER: Attr = Attr::new(&LEN_ATTR, 12..14);
    static ref TYPE_ATTR_HEADER: Attr = Attr::new(&TYPE_ATTR, 12..14);
    static ref TYPE_MAP: HashMap<u64, (Token, AttrClass)> = hashmap!{
        0x0800 => (token!("@data:ipv4"), AttrClass::builder("eth.type.ipv4").typ("@novalue").decoder(decoder::Const(true)).build()),
        0x0806 => (token!("@data:arp"), AttrClass::builder("eth.type.arp").typ("@novalue").decoder(decoder::Const(true)).build()),
        0x0842 => (token!("@data:wol"), AttrClass::builder("eth.type.wol").typ("@novalue").decoder(decoder::Const(true)).build()),
        0x86DD => (token!("@data:ipv6"), AttrClass::builder("eth.type.ipv6").typ("@novalue").decoder(decoder::Const(true)).build()),
        0x888E => (token!("@data:eap"), AttrClass::builder("eth.type.eap").typ("@novalue").decoder(decoder::Const(true)).build()),
    };
}

genet_dissectors!(EthDissector {});
