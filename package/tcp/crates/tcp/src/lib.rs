#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

use genet_sdk::prelude::*;

struct TcpWorker {}

impl Worker for TcpWorker {
    fn analyze(
        &mut self,
        _ctx: &mut Context,
        _stack: &LayerStack,
        parent: &mut Layer,
    ) -> Result<Status> {
        if let Some(payload) = parent
            .payloads()
            .iter()
            .find(|p| p.id() == token!("@data:tcp"))
        {
            let mut layer = Layer::new(&TCP_CLASS, payload.data());

            let data_offset: usize = OFFSET_ATTR_HEADER.try_get(&layer)?.try_into()?;
            let data_offset = data_offset * 4;
            let mut offset = 20;

            while offset < data_offset {
                let typ = layer.data().try_get(offset)?;
                let len = layer.data().try_get(offset + 1)? as usize;
                match typ {
                    0 => {
                        offset += 1;
                        continue;
                    }
                    1 => {
                        layer.add_attr(Attr::new(&OPTIONS_NOP_ATTR, offset..offset + 1));
                        offset += 1;
                        continue;
                    }
                    2 => {
                        layer.add_attr(Attr::new(&OPTIONS_MSS_ATTR, offset..offset + len));
                    }
                    3 => {
                        layer.add_attr(Attr::new(&OPTIONS_SCALE_ATTR, offset..offset + len));
                    }
                    4 => {
                        layer.add_attr(Attr::new(&OPTIONS_SACKP_ATTR, offset..offset + len));
                    }
                    5 => {
                        layer.add_attr(Attr::new(&OPTIONS_SACK_ATTR, offset..offset + len));
                    }
                    8 => {
                        layer.add_attr(Attr::new(&OPTIONS_TS_ATTR, offset..offset + len));
                        layer.add_attr(Attr::new(&OPTIONS_TS_MY_ATTR, offset + 2..offset + 6));
                        layer.add_attr(Attr::new(&OPTIONS_TS_ECHO_ATTR, offset + 6..offset + 10));
                    }
                    _ => {}
                }
                offset += len;
            }

            let payload = layer.data().try_get(data_offset..)?;
            layer.add_payload(payload, token!("@data:tcp"), "");
            Ok(Status::Done(vec![layer]))
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct TcpDissector {}

impl Dissector for TcpDissector {
    fn new_worker(&self, typ: &str, _ctx: &Context) -> Option<Box<Worker>> {
        if typ == "parallel" {
            Some(Box::new(TcpWorker {}))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref OFFSET_ATTR_HEADER: Attr = Attr::new(&OFFSET_ATTR, 12..13);
    static ref TCP_CLASS: LayerClass = LayerBuilder::new("tcp")
        .header(Attr::new(&SRC_ATTR, 0..2))
        .header(Attr::new(&DST_ATTR, 2..4))
        .header(Attr::new(&SEQ_ATTR, 4..8))
        .header(Attr::new(&ACK_ATTR, 8..12))
        .header(&OFFSET_ATTR_HEADER)
        .header(Attr::new(&FLAGS_ATTR, 12..14))
        .header(Attr::new(&FLAGS_NS_ATTR, 12..13))
        .header(Attr::new(&FLAGS_CWR_ATTR, 13..14))
        .header(Attr::new(&FLAGS_ECE_ATTR, 13..14))
        .header(Attr::new(&FLAGS_URG_ATTR, 13..14))
        .header(Attr::new(&FLAGS_ACK_ATTR, 13..14))
        .header(Attr::new(&FLAGS_PSH_ATTR, 13..14))
        .header(Attr::new(&FLAGS_RST_ATTR, 13..14))
        .header(Attr::new(&FLAGS_SYN_ATTR, 13..14))
        .header(Attr::new(&FLAGS_FIN_ATTR, 13..14))
        .header(Attr::new(&WINDOW_ATTR, 14..16))
        .header(Attr::new(&CHECKSUM_ATTR, 16..18))
        .header(Attr::new(&URGENT_ATTR, 18..20))
        .header(Attr::new(&OPTIONS_ATTR, 20..21))
        .build();
    static ref SRC_ATTR: AttrClass = AttrBuilder::new("tcp.src")
        .typ("@tcp:port")
        .decoder(decoder::UInt16BE())
        .build();
    static ref DST_ATTR: AttrClass = AttrBuilder::new("tcp.dst")
        .typ("@tcp:port")
        .decoder(decoder::UInt16BE())
        .build();
    static ref SEQ_ATTR: AttrClass = AttrBuilder::new("tcp.seq")
        .decoder(decoder::UInt32BE())
        .build();
    static ref ACK_ATTR: AttrClass = AttrBuilder::new("tcp.ack")
        .decoder(decoder::UInt32BE())
        .build();
    static ref OFFSET_ATTR: AttrClass = AttrBuilder::new("tcp.dataOffset")
        .decoder(decoder::UInt8().map(|v| v >> 4))
        .build();
    static ref FLAGS_ATTR: AttrClass = AttrBuilder::new("tcp.flags")
        .typ("@flags")
        .decoder(decoder::UInt16BE().map(|v| v & 0xfff))
        .build();
    static ref FLAGS_NS_ATTR: AttrClass = AttrBuilder::new("tcp.flags.ns")
        .decoder(decoder::UInt8().map(|v| (v & 0b0000_0001) != 0))
        .build();
    static ref FLAGS_CWR_ATTR: AttrClass = AttrBuilder::new("tcp.flags.cwr")
        .decoder(decoder::UInt8().map(|v| (v & 0b1000_0000) != 0))
        .build();
    static ref FLAGS_ECE_ATTR: AttrClass = AttrBuilder::new("tcp.flags.ece")
        .decoder(decoder::UInt8().map(|v| (v & 0b0100_0000) != 0))
        .build();
    static ref FLAGS_URG_ATTR: AttrClass = AttrBuilder::new("tcp.flags.urg")
        .decoder(decoder::UInt8().map(|v| (v & 0b0010_0000) != 0))
        .build();
    static ref FLAGS_ACK_ATTR: AttrClass = AttrBuilder::new("tcp.flags.ack")
        .decoder(decoder::UInt8().map(|v| (v & 0b0001_0000) != 0))
        .build();
    static ref FLAGS_PSH_ATTR: AttrClass = AttrBuilder::new("tcp.flags.psh")
        .decoder(decoder::UInt8().map(|v| (v & 0b0000_1000) != 0))
        .build();
    static ref FLAGS_RST_ATTR: AttrClass = AttrBuilder::new("tcp.flags.rst")
        .decoder(decoder::UInt8().map(|v| (v & 0b0000_0100) != 0))
        .build();
    static ref FLAGS_SYN_ATTR: AttrClass = AttrBuilder::new("tcp.flags.syn")
        .decoder(decoder::UInt8().map(|v| (v & 0b0000_0010) != 0))
        .build();
    static ref FLAGS_FIN_ATTR: AttrClass = AttrBuilder::new("tcp.flags.fin")
        .decoder(decoder::UInt8().map(|v| (v & 0b0000_0001) != 0))
        .build();
    static ref WINDOW_ATTR: AttrClass = AttrBuilder::new("tcp.window")
        .decoder(decoder::UInt16BE())
        .build();
    static ref CHECKSUM_ATTR: AttrClass = AttrBuilder::new("tcp.checksum")
        .decoder(decoder::UInt16BE())
        .build();
    static ref URGENT_ATTR: AttrClass = AttrBuilder::new("tcp.urgent")
        .decoder(decoder::UInt16BE())
        .build();
    static ref OPTIONS_ATTR: AttrClass = AttrBuilder::new("tcp.options")
        .typ("@nested")
        .decoder(decoder::Const(true))
        .build();
    static ref OPTIONS_NOP_ATTR: AttrClass = AttrBuilder::new("tcp.options.nop")
        .typ("@novalue")
        .decoder(decoder::Const(true))
        .build();
    static ref OPTIONS_MSS_ATTR: AttrClass = AttrBuilder::new("tcp.options.mss")
        .decoder(decoder::Ranged(decoder::UInt16BE(), 2..))
        .build();
    static ref OPTIONS_SCALE_ATTR: AttrClass = AttrBuilder::new("tcp.options.scale")
        .decoder(decoder::Ranged(decoder::UInt8(), 2..))
        .build();
    static ref OPTIONS_SACKP_ATTR: AttrClass = AttrBuilder::new(
        "tcp.options.selectiveAckPermitted"
    ).typ("@novalue")
        .decoder(decoder::Const(true))
        .build();
    static ref OPTIONS_SACK_ATTR: AttrClass = AttrBuilder::new("tcp.options.selectiveAck")
        .decoder(decoder::Ranged(decoder::ByteSlice(), 2..))
        .build();
    static ref OPTIONS_TS_ATTR: AttrClass = AttrBuilder::new("tcp.options.ts")
        .typ("@nested")
        .decoder(decoder::Const(true))
        .build();
    static ref OPTIONS_TS_MY_ATTR: AttrClass = AttrBuilder::new("tcp.options.ts.my")
        .decoder(decoder::UInt32BE())
        .build();
    static ref OPTIONS_TS_ECHO_ATTR: AttrClass = AttrBuilder::new("tcp.options.ts.echo")
        .decoder(decoder::UInt32BE())
        .build();
}

genet_dissectors!(TcpDissector {});
