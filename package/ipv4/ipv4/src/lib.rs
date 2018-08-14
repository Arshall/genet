#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use genet_sdk::prelude::*;
use std::collections::HashMap;

struct IPv4Worker {}

impl Worker for IPv4Worker {
    fn decode(
        &mut self,
        _ctx: &mut Context,
        _stack: &LayerStack,
        parent: &mut Layer,
    ) -> Result<Status> {
        if let Some(payload) = parent
            .payloads()
            .iter()
            .find(|p| p.id() == token!("@data:ipv4"))
        {
            let mut layer = Layer::new(&IPV4_CLASS, payload.data());
            let proto = PROTO_ATTR_HEADER.try_get(&layer)?.try_into()?;
            if let Some((typ, attr)) = PROTO_MAP.get(&proto) {
                layer.add_attr(Attr::new(attr, 9..10));
                let payload = layer.data().try_get(20..)?;
                layer.add_payload(Payload::new(payload, typ));
            }
            Ok(Status::Done(vec![layer]))
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct IPv4Decoder {}

impl Decoder for IPv4Decoder {
    fn new_worker(&self, typ: &str, _ctx: &Context) -> Option<Box<Worker>> {
        if typ == "parallel" {
            Some(Box::new(IPv4Worker {}))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref PROTO_ATTR_HEADER: Attr = Attr::new(&PROTO_ATTR, 9..10);
    static ref IPV4_CLASS: LayerClass = LayerClass::builder("ipv4")
        .alias("_.src", "ipv4.src")
        .alias("_.dst", "ipv4.dst")
        .header(Attr::new(&VERSION_ATTR, 0..1))
        .header(Attr::new(&HLEN_ATTR, 0..1))
        .header(Attr::new(&TOS_ATTR, 1..2))
        .header(Attr::new(&LENGTH_ATTR, 2..4))
        .header(Attr::new(&ID_ATTR, 4..6))
        .header(Attr::new(&FLAGS_ATTR, 6..7))
        .header(Attr::new(&FLAGS_RV_ATTR, 6..7))
        .header(Attr::new(&FLAGS_DF_ATTR, 6..7))
        .header(Attr::new(&FLAGS_MF_ATTR, 6..7))
        .header(Attr::new(&OFFSET_ATTR, 6..8))
        .header(Attr::new(&TTL_ATTR, 8..9))
        .header(&PROTO_ATTR_HEADER)
        .header(Attr::new(&CHECKSUM_ATTR, 10..12))
        .header(Attr::new(&SRC_ATTR, 12..16))
        .header(Attr::new(&DST_ATTR, 16..20))
        .build();
    static ref VERSION_ATTR: AttrClass = AttrClass::builder("ipv4.version")
        .cast(cast::UInt8().map(|v| v >> 4))
        .build();
    static ref HLEN_ATTR: AttrClass = AttrClass::builder("ipv4.headerLength")
        .cast(cast::UInt8().map(|v| v & 0b00001111))
        .build();
    static ref TOS_ATTR: AttrClass = AttrClass::builder("ipv4.tos")
        .cast(cast::UInt8())
        .build();
    static ref LENGTH_ATTR: AttrClass = AttrClass::builder("ipv4.totalLength")
        .cast(cast::UInt16BE())
        .build();
    static ref ID_ATTR: AttrClass = AttrClass::builder("ipv4.id")
        .cast(cast::UInt16BE())
        .build();
    static ref FLAGS_ATTR: AttrClass = AttrClass::builder("ipv4.flags")
        .cast(cast::UInt8().map(|v| (v >> 5) & 0b00000111))
        .typ("@flags")
        .build();
    static ref FLAGS_RV_ATTR: AttrClass = AttrClass::builder("ipv4.flags.reserved")
        .cast(cast::UInt8().map(|v| v & 0b10000000 != 0))
        .build();
    static ref FLAGS_DF_ATTR: AttrClass = AttrClass::builder("ipv4.flags.dontFragment")
        .cast(cast::UInt8().map(|v| v & 0b01000000 != 0))
        .build();
    static ref FLAGS_MF_ATTR: AttrClass = AttrClass::builder("ipv4.flags.moreFragments")
        .cast(cast::UInt8().map(|v| v & 0b00100000 != 0))
        .build();
    static ref OFFSET_ATTR: AttrClass = AttrClass::builder("ipv4.fragmentOffset")
        .cast(cast::UInt16BE().map(|v| v & 0x1fff))
        .build();
    static ref TTL_ATTR: AttrClass = AttrClass::builder("ipv4.ttl")
        .cast(cast::UInt8())
        .build();
    static ref PROTO_ATTR: AttrClass = AttrClass::builder("ipv4.protocol")
        .cast(cast::UInt8())
        .typ("@enum")
        .build();
    static ref CHECKSUM_ATTR: AttrClass = AttrClass::builder("ipv4.checksum")
        .cast(cast::UInt16BE())
        .build();
    static ref SRC_ATTR: AttrClass = AttrClass::builder("ipv4.src")
        .typ("@ipv4:addr")
        .cast(cast::ByteSlice())
        .build();
    static ref DST_ATTR: AttrClass = AttrClass::builder("ipv4.dst")
        .typ("@ipv4:addr")
        .cast(cast::ByteSlice())
        .build();
    static ref PROTO_MAP: HashMap<u64, (Token, AttrClass)> = hashmap!{
        0x01 => (token!("@data:icmp"), AttrClass::builder("ipv4.protocol.icmp").typ("@novalue").cast(cast::Const(true)).build()),
        0x02 => (token!("@data:igmp"), AttrClass::builder("ipv4.protocol.igmp").typ("@novalue").cast(cast::Const(true)).build()),
        0x06 => (token!("@data:tcp"), AttrClass::builder("ipv4.protocol.tcp").typ("@novalue").cast(cast::Const(true)).build()),
        0x11 => (token!("@data:udp"), AttrClass::builder("ipv4.protocol.udp").typ("@novalue").cast(cast::Const(true)).build()),
    };
}
genet_decoders!(IPv4Decoder {});
