use attr::Attr;
use fixed::{Fixed, MutFixed};
use slice::ByteSlice;
use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr, slice,
    sync::atomic::{AtomicPtr, Ordering},
};
use token::Token;

/// A layer stack object.
pub struct LayerStack<'a> {
    buffer: &'a [*const Layer],
}

impl<'a> LayerStack<'a> {
    pub(crate) unsafe fn new(ptr: *const *const Layer, len: usize) -> LayerStack<'a> {
        Self {
            buffer: slice::from_raw_parts(ptr, len),
        }
    }

    /// Returns the top of the LayerStack.
    pub fn top(&self) -> Option<&Layer> {
        self.layers().last()
    }

    /// Returns the bottom of the LayerStack.
    pub fn bottom(&self) -> Option<&Layer> {
        self.layers().next()
    }

    /// Find the attribute in the LayerStack.
    pub fn attr(&self, id: Token) -> Option<&Attr> {
        for layer in self.layers().rev() {
            if let Some(attr) = layer.attr(id) {
                return Some(attr);
            }
        }
        None
    }

    /// Find the layer in the LayerStack.
    pub fn layer(&self, id: Token) -> Option<&Layer> {
        self.layers().find(|layer| layer.id() == id)
    }

    fn layers(&self) -> impl DoubleEndedIterator<Item = &'a Layer> {
        self.buffer.iter().map(|layer| unsafe { &**layer })
    }
}

/// A mutable proxy for a layer object.
#[repr(C)]
pub struct LayerProxy<'a> {
    layer: *mut Layer,
    next: *mut LayerData,
    attrs: Vec<Fixed<Attr>>,
    payloads: Vec<Payload>,
    children: Vec<*mut Layer>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> LayerProxy<'a> {
    pub fn from_mut_ref(layer: &'a mut Layer) -> LayerProxy {
        LayerProxy {
            layer,
            next: &mut layer.root as *mut LayerData,
            attrs: Vec::new(),
            payloads: Vec::new(),
            children: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn from_ref(layer: &'a Layer) -> LayerProxy {
        LayerProxy {
            layer: unsafe { &mut *((layer as *const Layer) as *mut Layer) },
            next: ptr::null_mut(),
            attrs: Vec::new(),
            payloads: Vec::new(),
            children: Vec::new(),
            phantom: PhantomData,
        }
    }

    fn get_next(&mut self) -> &mut LayerData {
        if self.next.is_null() {
            self.next = Box::into_raw(Box::new(LayerData {
                next: AtomicPtr::default(),
                attrs: Vec::new(),
                payloads: Vec::new(),
                next_child: None,
            }));
        }
        unsafe { &mut *self.next }
    }

    pub fn apply(&mut self) {
        let mut data = &mut self.deref_mut().root as *mut LayerData;
        if !self.next.is_null() && self.next != data {
            while !data.is_null() {
                data = unsafe { &(*data).next }.compare_and_swap(
                    ptr::null_mut(),
                    self.next,
                    Ordering::Relaxed,
                );
            }
        }
    }

    /// Returns the ID of self.
    pub fn id(&self) -> Token {
        self.deref().id()
    }

    /// Returns the type of self.
    pub fn data(&self) -> ByteSlice {
        self.deref().data()
    }

    /// Returns the slice of headers.
    pub fn headers(&self) -> &[Fixed<Attr>] {
        self.deref().headers()
    }

    /// Returns the slice of attributes.
    pub fn attrs(&self) -> &[Fixed<Attr>] {
        self.deref().attrs()
    }

    /// Find the attribute in the Layer.
    pub fn attr<T: Into<Token>>(&self, id: T) -> Option<&Attr> {
        self.deref().attr(id)
    }

    /// Adds an attribute to the Layer.
    pub fn add_attr<T: Into<Fixed<Attr>>>(&mut self, attr: T) {
        self.attrs.push(attr.into());
    }

    /// Returns the slice of payloads.
    pub fn payloads(&self) -> &[Payload] {
        self.deref().payloads()
    }

    /// Adds a payload to the Layer.
    pub fn add_payload(&mut self, payload: Payload) {
        self.payloads.push(payload);
    }

    pub fn add_child<T: Into<MutFixed<Layer>>>(&mut self, layer: T) {
        self.children.push(layer.into().as_mut_ptr());
    }

    pub fn children(&self) -> &[*mut Layer] {
        &self.children
    }
}

impl<'a> Deref for LayerProxy<'a> {
    type Target = Layer;

    fn deref(&self) -> &Layer {
        unsafe { &*self.layer }
    }
}

impl<'a> DerefMut for LayerProxy<'a> {
    fn deref_mut(&mut self) -> &mut Layer {
        unsafe { &mut *self.layer }
    }
}

#[repr(C)]
struct LayerData {
    next: AtomicPtr<LayerData>,
    attrs: Vec<Fixed<Attr>>,
    payloads: Vec<Payload>,
    next_child: Option<MutFixed<Layer>>,
}

pub struct LayerBuilder {
    class: Fixed<LayerClass>,
    data: ByteSlice,
    attrs: Vec<Fixed<Attr>>,
    payloads: Vec<Payload>,
    children: Vec<MutFixed<Layer>>,
}

impl Into<Layer> for LayerBuilder {
    fn into(self) -> Layer {
        Layer {
            class: self.class,
            data: self.data,
            root: LayerData {
                next: AtomicPtr::default(),
                attrs: self.attrs,
                payloads: self.payloads,
                next_child: None,
            },
        }
    }
}

impl LayerBuilder {
    /// Returns the type of self.
    pub fn data(&self) -> ByteSlice {
        self.data
    }

    /// Adds an attribute to the Layer.
    pub fn add_attr<T: Into<Fixed<Attr>>>(&mut self, attr: T) {
        self.attrs.push(attr.into());
    }

    /// Adds a payload to the Layer.
    pub fn add_payload(&mut self, payload: Payload) {
        self.payloads.push(payload);
    }
}

impl Into<MutFixed<Layer>> for LayerBuilder {
    fn into(self) -> MutFixed<Layer> {
        MutFixed::new(self.into())
    }
}

/// A layer object.
#[repr(C)]
pub struct Layer {
    class: Fixed<LayerClass>,
    data: ByteSlice,
    root: LayerData,
}

unsafe impl Send for Layer {}

impl Layer {
    /// Creates a new Layer.
    pub fn new<C: Into<Fixed<LayerClass>>, B: Into<ByteSlice>>(class: C, data: B) -> LayerBuilder {
        LayerBuilder {
            class: class.into(),
            data: data.into(),
            attrs: Vec::new(),
            payloads: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Returns the ID of self.
    pub fn id(&self) -> Token {
        self.class.id()
    }

    /// Returns the type of self.
    pub fn data(&self) -> ByteSlice {
        self.data
    }

    /// Returns the slice of headers.
    pub fn headers(&self) -> &[Fixed<Attr>] {
        self.class.headers()
    }

    /// Returns the slice of attributes.
    pub fn attrs(&self) -> &[Fixed<Attr>] {
        self.class.attrs(&self.root)
    }

    /// Find the attribute in the Layer.
    pub fn attr<T: Into<Token>>(&self, id: T) -> Option<&Attr> {
        let id = id.into();
        let id = self
            .class
            .aliases()
            .find(|alias| alias.id == id)
            .map(|alias| alias.target)
            .unwrap_or(id);
        self.attrs()
            .iter()
            .chain(self.class.headers().iter())
            .find(|attr| attr.id() == id)
            .map(|attr| attr.as_ref())
    }

    /// Adds an attribute to the Layer.
    pub fn add_attr<T: Into<Fixed<Attr>>>(&mut self, attr: T) {
        let func = self.class.add_attr;
        (func)(&mut self.root, attr.into());
    }

    /// Returns the slice of payloads.
    pub fn payloads(&self) -> &[Payload] {
        self.class.payloads(&self.root)
    }

    /// Adds a payload to the Layer.
    pub fn add_payload(&mut self, payload: Payload) {
        let func = self.class.add_payload;
        (func)(&mut self.root, payload);
    }
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer {:?}", self.id())
    }
}

impl Into<MutFixed<Layer>> for Layer {
    fn into(self) -> MutFixed<Layer> {
        MutFixed::new(self)
    }
}

/// A payload object.
#[repr(C)]
pub struct Payload {
    data: *const u8,
    len: u64,
    id: Token,
    typ: Token,
}

impl Payload {
    /// Creates a new payload.
    pub fn new<B: Into<ByteSlice>, T: Into<Token>>(data: B, id: T) -> Payload {
        Self::with_typ(data, id, "")
    }

    /// Creates a new payload with the given type.
    pub fn with_typ<B: Into<ByteSlice>, T: Into<Token>, U: Into<Token>>(
        data: B,
        id: T,
        typ: U,
    ) -> Payload {
        let data: ByteSlice = data.into();
        Self {
            data: data.as_ptr(),
            len: data.len() as u64,
            id: id.into(),
            typ: typ.into(),
        }
    }

    /// Returns the ID of self.
    pub fn id(&self) -> Token {
        self.id
    }

    /// Returns the type of self.
    pub fn typ(&self) -> Token {
        self.typ
    }

    /// Returns the data of self.
    pub fn data(&self) -> ByteSlice {
        unsafe { ByteSlice::from_raw_parts(self.data, self.len as usize) }
    }
}

/// A builder object for LayerClass.
pub struct LayerClassBuilder {
    id: Token,
    aliases: Vec<Alias>,
    headers: Vec<Fixed<Attr>>,
}

impl LayerClassBuilder {
    /// Adds an attribute alias for LayerClass.
    pub fn alias<T: Into<Token>, U: Into<Token>>(mut self, id: T, target: U) -> LayerClassBuilder {
        self.aliases.push(Alias {
            id: id.into(),
            target: target.into(),
        });
        self
    }

    /// Adds a header attribute for LayerClass.
    pub fn header<T: Into<Fixed<Attr>>>(mut self, attr: T) -> LayerClassBuilder {
        self.headers.push(attr.into());
        self
    }

    /// Builds a new LayerClass.
    pub fn build(self) -> LayerClass {
        LayerClass {
            attrs_len: abi_attrs_len,
            attrs_data: abi_attrs_data,
            aliases_len: abi_aliases_len,
            aliases_data: abi_aliases_data,
            headers_len: abi_headers_len,
            headers_data: abi_headers_data,
            add_attr: abi_add_attr,
            payloads_len: abi_payloads_len,
            payloads_data: abi_payloads_data,
            add_payload: abi_add_payload,
            id: self.id,
            aliases: self.aliases,
            headers: self.headers,
        }
    }
}

#[repr(C)]
struct Alias {
    id: Token,
    target: Token,
}

/// A layer class object.
#[repr(C)]
pub struct LayerClass {
    aliases_len: extern "C" fn(*const LayerClass) -> u64,
    aliases_data: extern "C" fn(*const LayerClass) -> *const Alias,
    headers_len: extern "C" fn(*const LayerClass) -> u64,
    headers_data: extern "C" fn(*const LayerClass) -> *const Fixed<Attr>,
    attrs_len: extern "C" fn(*const LayerData) -> u64,
    attrs_data: extern "C" fn(*const LayerData) -> *const Fixed<Attr>,
    add_attr: extern "C" fn(*mut LayerData, Fixed<Attr>),
    payloads_len: extern "C" fn(*const LayerData) -> u64,
    payloads_data: extern "C" fn(*const LayerData) -> *const Payload,
    add_payload: extern "C" fn(*mut LayerData, Payload),
    id: Token,
    aliases: Vec<Alias>,
    headers: Vec<Fixed<Attr>>,
}

impl LayerClass {
    /// Creates a new builder object for LayerClass.
    pub fn builder<T: Into<Token>>(id: T) -> LayerClassBuilder {
        LayerClassBuilder {
            id: id.into(),
            aliases: Vec::new(),
            headers: Vec::new(),
        }
    }

    fn id(&self) -> Token {
        self.id
    }

    fn aliases(&self) -> impl Iterator<Item = &Alias> {
        let data = (self.aliases_data)(self);
        let len = (self.aliases_len)(self) as usize;
        let iter = unsafe { slice::from_raw_parts(data, len).iter() };
        iter.map(|v| &*v)
    }

    fn headers(&self) -> &[Fixed<Attr>] {
        let data = (self.headers_data)(self);
        let len = (self.headers_len)(self) as usize;
        unsafe { slice::from_raw_parts(data, len) }
    }

    fn attrs(&self, layer: &LayerData) -> &[Fixed<Attr>] {
        let data = (self.attrs_data)(layer);
        let len = (self.attrs_len)(layer) as usize;
        unsafe { slice::from_raw_parts(data, len) }
    }

    fn payloads(&self, layer: &LayerData) -> &[Payload] {
        let data = (self.payloads_data)(layer);
        let len = (self.payloads_len)(layer) as usize;
        unsafe { slice::from_raw_parts(data, len) }
    }
}

impl Into<Fixed<LayerClass>> for &'static LayerClass {
    fn into(self) -> Fixed<LayerClass> {
        Fixed::from_static(self)
    }
}

extern "C" fn abi_aliases_len(class: *const LayerClass) -> u64 {
    unsafe { (*class).aliases.len() as u64 }
}

extern "C" fn abi_aliases_data(class: *const LayerClass) -> *const Alias {
    unsafe { (*class).aliases.as_ptr() }
}

extern "C" fn abi_headers_len(class: *const LayerClass) -> u64 {
    unsafe { (*class).headers.len() as u64 }
}

extern "C" fn abi_headers_data(class: *const LayerClass) -> *const Fixed<Attr> {
    unsafe { (*class).headers.as_ptr() }
}

extern "C" fn abi_attrs_len(layer: *const LayerData) -> u64 {
    unsafe { (*layer).attrs.len() as u64 }
}

extern "C" fn abi_attrs_data(layer: *const LayerData) -> *const Fixed<Attr> {
    unsafe { (*layer).attrs.as_ptr() }
}

extern "C" fn abi_add_attr(layer: *mut LayerData, attr: Fixed<Attr>) {
    let attrs = unsafe { &mut (*layer).attrs };
    attrs.push(attr);
}

extern "C" fn abi_payloads_len(layer: *const LayerData) -> u64 {
    unsafe { (*layer).payloads.len() as u64 }
}

extern "C" fn abi_payloads_data(layer: *const LayerData) -> *const Payload {
    unsafe { (*layer).payloads.as_ptr() }
}

extern "C" fn abi_add_payload(layer: *mut LayerData, payload: Payload) {
    let payloads = unsafe { &mut (*layer).payloads };
    payloads.push(payload);
}

#[cfg(test)]
mod tests {
    use attr::{Attr, AttrClass};
    use cast::Cast;
    use fixed::Fixed;
    use layer::{Layer, LayerClass, Payload};
    use slice::ByteSlice;
    use std::io::Result;
    use token::Token;
    use variant::Variant;

    #[test]
    fn id() {
        let id = Token::from(123);
        let class = Fixed::new(LayerClass::builder(id).build());
        let layer = Layer::new(class, ByteSlice::new());
        assert_eq!(layer.id(), id);
    }

    #[test]
    fn data() {
        let data = b"hello";
        let class = Fixed::new(LayerClass::builder(Token::null()).build());
        let layer = Layer::new(class, ByteSlice::from(&data[..]));
        assert_eq!(layer.data(), ByteSlice::from(&data[..]));
    }

    #[test]
    fn payloads() {
        let class = Fixed::new(LayerClass::builder(Token::null()).build());
        let mut layer = Layer::new(class, ByteSlice::new());
        assert!(layer.payloads().iter().next().is_none());

        let count = 100;
        let data = b"hello";

        for i in 0..count {
            layer.add_payload(Payload::new(ByteSlice::from(&data[..]), Token::from(i)));
        }

        let mut iter = layer.payloads().iter();
        for i in 0..count {
            let payload = iter.next().unwrap();
            assert_eq!(payload.data(), ByteSlice::from(&data[..]));
            assert_eq!(payload.id(), Token::from(i));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn attrs() {
        let class = Fixed::new(LayerClass::builder(Token::null()).build());
        let mut layer = Layer::new(class, ByteSlice::new());
        assert!(layer.attrs().is_empty());

        #[derive(Clone)]
        struct TestCast {}

        impl Cast for TestCast {
            fn cast(&self, _: &ByteSlice) -> Result<Variant> {
                Ok(Variant::Nil)
            }
        }
        let class = Fixed::new(
            AttrClass::builder("nil")
                .typ("@nil")
                .cast(TestCast {})
                .build(),
        );

        let count = 100;
        for i in 0..count {
            let attr = Attr::builder(class.clone()).range(0..i).build();
            layer.add_attr(attr);
        }
        let mut iter = layer.attrs().iter();
        for i in 0..count {
            let attr = iter.next().unwrap();
            assert_eq!(attr.id(), Token::from("nil"));
            assert_eq!(attr.typ(), Token::from("@nil"));
            assert_eq!(attr.range(), 0..i);
        }
        assert!(iter.next().is_none());
    }
}
