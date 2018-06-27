extern crate genet_abi;
extern crate pcap;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use genet_sdk::{
    attr::{Attr, AttrBuilder, AttrClass},
    context::Context,
    io::{Reader, ReaderWorker},
    layer::{Layer, LayerBuilder, LayerClass},
    ptr::Ptr,
    result::Result,
    token,
    variant::Variant,
};
use pcap::Header;

use std::io::{BufRead, BufReader, Read};
use std::mem;
use std::process::{Child, ChildStdout, Command, Stdio};
use std::slice;

#[derive(Deserialize)]
struct Arg {
    cmd: String,
    args: Vec<String>,
    link: u32,
}

#[derive(Clone)]
struct PcapReader {}

impl Reader for PcapReader {
    fn new_worker(&self, _ctx: &Context, arg: &str) -> Box<ReaderWorker> {
        let arg: Arg = serde_json::from_str(arg).unwrap();
        let mut child = Command::new(&arg.cmd)
            .args(&arg.args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute pcap_cli");
        let reader = BufReader::new(child.stdout.take().unwrap());
        let link_class = LayerBuilder::new(format!("[link-{}]", arg.link))
            .header(Attr::with_value(
                &TYPE_CLASS,
                0..0,
                Variant::Int64(arg.link as i64),
            ))
            .build();
        Box::new(PcapReaderWorker {
            child,
            reader,
            link_class,
        })
    }

    fn id(&self) -> &str {
        "pcap"
    }
}

struct PcapReaderWorker {
    child: Child,
    reader: BufReader<ChildStdout>,
    link_class: Ptr<LayerClass>,
}

impl ReaderWorker for PcapReaderWorker {
    fn read(&mut self) -> Result<Vec<Layer>> {
        let mut header = String::new();
        self.reader.read_line(&mut header)?;
        let header = header.trim();
        if header.is_empty() {
            return Ok(vec![]);
        }
        let header: Header = serde_json::from_str(header)?;
        let mut data = vec![0u8; header.datalen as usize];
        self.reader.read_exact(&mut data)?;
        let payload = unsafe { slice::from_raw_parts(data.as_ptr(), data.len()) };
        mem::forget(data);
        let mut layer = Layer::new(&self.link_class, payload);
        layer.add_attr(Attr::with_value(
            &LENGTH_CLASS,
            0..0,
            Variant::UInt64(header.actlen as u64),
        ));
        layer.add_attr(Attr::with_value(
            &TIMESTAMP_CLASS,
            0..0,
            Variant::Float64(header.ts_sec as f64 + header.ts_usec as f64 / 1000_000f64),
        ));
        Ok(vec![layer])
    }
}

impl Drop for PcapReaderWorker {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

lazy_static! {
    static ref TYPE_CLASS: Ptr<AttrClass> = AttrBuilder::new("link.type").build();
    static ref LENGTH_CLASS: Ptr<AttrClass> = AttrBuilder::new("link.length").build();
    static ref TIMESTAMP_CLASS: Ptr<AttrClass> = AttrBuilder::new("link.timestamp")
        .typ("@datetime:unix")
        .build();
}

genet_readers!(PcapReader {});
