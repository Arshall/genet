#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use genet_sdk::prelude::*;
use std::collections::HashMap;

struct NtpWorker {}

impl Worker for NtpWorker {
    fn decode(
        &mut self,
        _ctx: &mut Context,
        stack: &LayerStack,
        parent: &mut Layer,
    ) -> Result<Status> {
        if parent.id() != token!("udp") {
            return Ok(Status::Skip);
        }
        if let Some(payload) = parent.payloads().iter().next() {
            let parent_src: u16 = stack
                .attr(token!("udp.src"))
                .unwrap()
                .try_get(parent)?
                .try_into()?;

            let parent_dst: u16 = stack
                .attr(token!("udp.dst"))
                .unwrap()
                .try_get(parent)?
                .try_into()?;

            if parent_src != 123 && parent_dst != 123 {
                return Ok(Status::Skip);
            }

            let mut layer = Layer::new(&NTP_CLASS, payload.data());
            let leap_type = LEAP_ATTR_HEADER.try_get(&layer)?.try_into()?;

            let leap = LEAP_MAP.get(&leap_type);
            if let Some(attr) = leap {
                layer.add_attr(attr!(attr, 0..1));
            }

            let mode_type = MODE_ATTR_HEADER.try_get(&layer)?.try_into()?;

            let mode = MODE_MAP.get(&mode_type);
            if let Some(attr) = mode {
                layer.add_attr(attr!(attr, 0..1));
            }

            let stratum: u8 = STRATUM_ATTR_HEADER.try_get(&layer)?.try_into()?;
            layer.add_attr(if stratum >= 2 {
                attr!(&ID_IP_ATTR, 12..16)
            } else {
                attr!(&ID_ATTR, 12..16)
            });

            Ok(Status::Done(vec![layer]))
        } else {
            Ok(Status::Skip)
        }
    }
}

#[derive(Clone)]
struct NtpDecoder {}

impl Decoder for NtpDecoder {
    fn new_worker(&self, typ: &str, _ctx: &Context) -> Option<Box<Worker>> {
        if typ == "parallel" {
            Some(Box::new(NtpWorker {}))
        } else {
            None
        }
    }
}

def_layer_class!(
    NTP_CLASS,
    "ntp",
    header: &LEAP_ATTR_HEADER,
    header: attr!(&VERSION_ATTR, 0..1),
    header: &MODE_ATTR_HEADER,
    header: &STRATUM_ATTR_HEADER,
    header: attr!(&POLL_ATTR, 2..3),
    header: attr!(&PRECISION_ATTR, 3..4),
    header: attr!(&RDELAY_ATTR, 4..8),
    header: attr!(&RDELAY_SEC_ATTR, 4..6),
    header: attr!(&RDELAY_FRA_ATTR, 6..8),
    header: attr!(&RDISP_ATTR, 8..12),
    header: attr!(&RDISP_SEC_ATTR, 8..10),
    header: attr!(&RDISP_FRA_ATTR, 10..12),
    header: attr!(&REFTS_ATTR, 16..24),
    header: attr!(&REFTS_SEC_ATTR, 16..20),
    header: attr!(&REFTS_FRA_ATTR, 20..24),
    header: attr!(&ORITS_ATTR, 24..32),
    header: attr!(&ORITS_SEC_ATTR, 24..28),
    header: attr!(&ORITS_FRA_ATTR, 28..32),
    header: attr!(&RECTS_ATTR, 32..40),
    header: attr!(&RECTS_SEC_ATTR, 32..36),
    header: attr!(&RECTS_FRA_ATTR, 36..40),
    header: attr!(&TRATS_ATTR, 40..48),
    header: attr!(&TRATS_SEC_ATTR, 40..44),
    header: attr!(&TRATS_FRA_ATTR, 44..48)
);

def_attr!(LEAP_ATTR_HEADER, &LEAP_ATTR, 0..1);
def_attr!(MODE_ATTR_HEADER, &MODE_ATTR, 0..1);
def_attr!(STRATUM_ATTR_HEADER, &STRATUM_ATTR, 1..2);

def_attr_class!(LEAP_ATTR, "ntp.leapIndicator",
    typ: "@enum",
    cast: cast::UInt8().map(|v| v >> 6)
);

def_attr_class!(VERSION_ATTR, "ntp.version",
    cast: cast::UInt8().map(|v| (v >> 3) & 0b111)
);

def_attr_class!(MODE_ATTR, "ntp.mode",
    typ: "@enum",
    cast: cast::UInt8().map(|v| v & 0b111)
);

def_attr_class!(STRATUM_ATTR, "ntp.stratum", cast: cast::UInt8());

def_attr_class!(POLL_ATTR, "ntp.pollInterval", cast: cast::UInt8());

def_attr_class!(PRECISION_ATTR, "ntp.precision", cast: cast::UInt8());

def_attr_class!(RDELAY_ATTR, "ntp.rootDelay",
    cast: cast::UInt32BE().map(|v| (v >> 16) as f64 + ((v & 0xffff) as f64 / 65536f64))
);

def_attr_class!(
    RDELAY_SEC_ATTR,
    "ntp.rootDelay.seconds",
    cast: cast::UInt16BE()
);

def_attr_class!(
    RDELAY_FRA_ATTR,
    "ntp.rootDelay.fraction",
    cast: cast::UInt16BE()
);

def_attr_class!(RDISP_ATTR, "ntp.rootDispersion",
    cast: cast::UInt32BE().map(|v| (v >> 16) as f64 + ((v & 0xffff) as f64 / 65536f64))
);

def_attr_class!(
    RDISP_SEC_ATTR,
    "ntp.rootDispersion.seconds",
    cast: cast::UInt16BE()
);

def_attr_class!(
    RDISP_FRA_ATTR,
    "ntp.rootDispersion.fraction",
    cast: cast::UInt16BE()
);

def_attr_class!(ID_ATTR, "ntp.identifier", cast: cast::ByteSlice());

def_attr_class!(ID_IP_ATTR, "ntp.identifier",
    typ: "@ipv4:addr",
    cast: cast::ByteSlice()
);

def_attr_class!(REFTS_ATTR, "ntp.referenceTs",
    typ: "@ntp:time",
    cast: cast::UInt64BE().map(|v| (v >> 32) as f64 + ((v & 0xffff_ffff) as f64 / 4294967296f64))
);

def_attr_class!(
    REFTS_SEC_ATTR,
    "ntp.referenceTs.seconds",
    cast: cast::UInt32BE()
);

def_attr_class!(
    REFTS_FRA_ATTR,
    "ntp.referenceTs.fraction",
    cast: cast::UInt32BE()
);

def_attr_class!(ORITS_ATTR, "ntp.originateTs",
    typ: "@ntp:time",
    cast: cast::UInt64BE().map(|v| (v >> 32) as f64 + ((v & 0xffff_ffff) as f64 / 4294967296f64))
);

def_attr_class!(
    ORITS_SEC_ATTR,
    "ntp.originateTs.seconds",
    cast: cast::UInt32BE()
);

def_attr_class!(
    ORITS_FRA_ATTR,
    "ntp.originateTs.fraction",
    cast: cast::UInt32BE()
);

def_attr_class!(RECTS_ATTR, "ntp.receiveTs",
    typ: "@ntp:time",
    cast: cast::UInt64BE().map(|v| (v >> 32) as f64 + ((v & 0xffff_ffff) as f64 / 4294967296f64))
);

def_attr_class!(
    RECTS_SEC_ATTR,
    "ntp.receiveTs.seconds",
    cast: cast::UInt32BE()
);

def_attr_class!(
    RECTS_FRA_ATTR,
    "ntp.receiveTs.fraction",
    cast: cast::UInt32BE()
);

def_attr_class!(TRATS_ATTR, "ntp.transmitTs",
    typ: "@ntp:time",
    cast: cast::UInt64BE().map(|v| (v >> 32) as f64 + ((v & 0xffff_ffff) as f64 / 4294967296f64))
);

def_attr_class!(
    TRATS_SEC_ATTR,
    "ntp.transmitTs.seconds",
    cast: cast::UInt32BE()
);

def_attr_class!(
    TRATS_FRA_ATTR,
    "ntp.transmitTs.fraction",
    cast: cast::UInt32BE()
);

lazy_static! {
    static ref LEAP_MAP: HashMap<u64, AttrClass> = hashmap!{
        0 => attr_class!("ntp.leapIndicator.noWarning", typ: "@novalue", cast: cast::Const(true)),
        1 => attr_class!("ntp.leapIndicator.sec61", typ: "@novalue", cast: cast::Const(true)),
        2 => attr_class!("ntp.leapIndicator.sec59", typ: "@novalue", cast: cast::Const(true)),
        3 => attr_class!("ntp.leapIndicator.unknown", typ: "@novalue", cast: cast::Const(true)),
    };
    static ref MODE_MAP: HashMap<u64, AttrClass> = hashmap!{
        0 => attr_class!("ntp.mode.reserved", typ: "@novalue", cast: cast::Const(true)),
        1 => attr_class!("ntp.mode.symmetricActive", typ: "@novalue", cast: cast::Const(true)),
        2 => attr_class!("ntp.mode.symmetricPassive", typ: "@novalue", cast: cast::Const(true)),
        3 => attr_class!("ntp.mode.client", typ: "@novalue", cast: cast::Const(true)),
        4 => attr_class!("ntp.mode.server", typ: "@novalue", cast: cast::Const(true)),
        5 => attr_class!("ntp.mode.broadcast", typ: "@novalue", cast: cast::Const(true)),
        6 => attr_class!("ntp.mode.controlMessage", typ: "@novalue", cast: cast::Const(true)),
        7 => attr_class!("ntp.mode.reservedForPrivate", typ: "@novalue", cast: cast::Const(true)),
    };
}

genet_decoders!(NtpDecoder {});
