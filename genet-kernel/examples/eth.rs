extern crate genet_sdk;

#[macro_use]
extern crate genet_derive;

/** # Example 
aaa
*/
#[derive(Attr)]
#[genet(range = "0..5")]
struct PayloadLen {}

use genet_sdk::{cast, decoder::*, prelude::*};

struct EthWorker {}

impl Worker for EthWorker {
    fn decode(
        &mut self,
        _ctx: &mut Context,
        _stack: &LayerStack,
        parent: &mut Parent,
    ) -> Result<Status> {
        if parent.id() == token!("[link-1]") {
            let mut layer = Layer::new(&ETH_CLASS, parent.data());
            let len = LEN_ATTR_HEADER.try_get(&layer)?.try_into()?;
            if len <= 1500 {
                layer.add_attr(&LEN_ATTR_HEADER);
            } else {
                layer.add_attr(&TYPE_ATTR_HEADER);
            }
            if let Some((typ, attr)) = get_type(len) {
                layer.add_attr(attr!(attr, range: 12..14));
                let payload = parent.data().try_get(14..)?;
                layer.add_payload(Payload::new(payload, typ));
            }
            parent.add_child(layer);
            Ok(Status::Done)
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct EthDecoder {}

impl Decoder for EthDecoder {
    fn new_worker(&self, _ctx: &Context) -> Box<Worker> {
        Box::new(EthWorker {})
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            exec_type: ExecType::ParallelSync,
            ..Metadata::default()
        }
    }
}

def_layer_class!(ETH_CLASS, "eth",
            alias: "_.src" "eth.src",
            alias: "_.dst" "eth.dst",
            header: attr!(&SRC_ATTR, range: 0..6),
            header: attr!(&DST_ATTR, range: 6..12)
        );

def_attr_class!(SRC_ATTR, "eth.src",
            typ: "@eth:mac",
            cast: cast::ByteSlice()
        );

def_attr_class!(DST_ATTR, "eth.dst",
            typ: "@eth:mac",
            cast: cast::ByteSlice()
        );

def_attr_class!(LEN_ATTR, "eth.len", cast: cast::UInt16BE());

def_attr_class!(TYPE_ATTR, "eth.type",
            typ: "@enum",
            cast: cast::UInt16BE()
        );

def_attr!(LEN_ATTR_HEADER,  &LEN_ATTR, range: 12..14);

def_attr!(TYPE_ATTR_HEADER,  &TYPE_ATTR, range: 12..14);

fn get_type(val: u64) -> Option<(Token, &'static AttrClass)> {
    match val {
        0x0800 => Some((
            token!("@data:ipv4"),
            attr_class_lazy!("eth.type.ipv4", typ: "@novalue", value: true),
        )),
        0x0806 => Some((
            token!("@data:arp"),
            attr_class_lazy!("eth.type.arp", typ: "@novalue", value: true),
        )),
        0x0842 => Some((
            token!("@data:wol"),
            attr_class_lazy!("eth.type.wol", typ: "@novalue", value: true),
        )),
        0x86DD => Some((
            token!("@data:ipv6"),
            attr_class_lazy!("eth.type.ipv6", typ: "@novalue", value: true),
        )),
        0x888E => Some((
            token!("@data:eap"),
            attr_class_lazy!("eth.type.eap", typ: "@novalue", value: true),
        )),
        _ => None,
    }
}

genet_decoders!(EthDecoder {});
