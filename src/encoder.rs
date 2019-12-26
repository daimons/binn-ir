// License: see LICENSE file at root directory of `master` branch

//! # Encoder

use {
    alloc::{
        boxed::Box,
        string::String,
    },
    std::io::Write,

    crate::{
        Blob, IoResult, List, Map, Object, Size,
        value::Value,
    },
};

/// # Encoder
///
/// Default implementors are copied from [`Write`][std::io/Write].
///
/// [std::io/Write]: https://doc.rust-lang.org/std/io/trait.Write.html
pub trait Encoder: Write + Sized {

    /// # Encodes a value
    ///
    /// Result: total bytes that have been written.
    fn encode(&mut self, value: &Value) -> IoResult<Size> {
        value.encode(self)
    }

    /// # Encodes a null
    ///
    /// Result: total bytes that have been written.
    fn encode_null(&mut self) -> IoResult<Size> {
        crate::encode_null(self)
    }

    /// # Encodes a boolean
    ///
    /// Result: total bytes that have been written.
    fn encode_bool(&mut self, b: bool) -> IoResult<Size> {
        crate::encode_bool(self, b)
    }

    /// # Encodes a `u8`
    ///
    /// Result: total bytes that have been written.
    fn encode_u8(&mut self, u: u8) -> IoResult<Size> {
        crate::encode_u8(self, u)
    }

    /// # Encodes an `i8`
    ///
    /// Result: total bytes that have been written.
    fn encode_i8(&mut self, i: i8) -> IoResult<Size> {
        crate::encode_i8(self, i)
    }

    /// # Encodes a `u16`
    ///
    /// Result: total bytes that have been written.
    fn encode_u16(&mut self, u: u16) -> IoResult<Size> {
        crate::encode_u16(self, u)
    }

    /// # Encodes an `i16`
    ///
    /// Result: total bytes that have been written.
    fn encode_i16(&mut self, i: i16) -> IoResult<Size> {
        crate::encode_i16(self, i)
    }

    /// # Encodes a `u32`
    ///
    /// Result: total bytes that have been written.
    fn encode_u32(&mut self, u: u32) -> IoResult<Size> {
        crate::encode_u32(self, u)
    }

    /// # Encodes an `i32`
    ///
    /// Result: total bytes that have been written.
    fn encode_i32(&mut self, i: i32) -> IoResult<Size> {
        crate::encode_i32(self, i)
    }

    /// # Encodes a `u64`
    ///
    /// Result: total bytes that have been written.
    fn encode_u64(&mut self, u: u64) -> IoResult<Size> {
        crate::encode_u64(self, u)
    }

    /// # Encodes an `i64`
    ///
    /// Result: total bytes that have been written.
    fn encode_i64(&mut self, i: i64) -> IoResult<Size> {
        crate::encode_i64(self, i)
    }

    /// # Encodes a [`Float`][Value::Float]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Float]: value/enum.Value.html#variant.Float
    fn encode_float(&mut self, f: f32) -> IoResult<Size> {
        crate::encode_float(self, f)
    }

    /// # Encodes a [`Double`][Value::Double]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Double]: value/enum.Value.html#variant.Double
    fn encode_double(&mut self, d: f64) -> IoResult<Size> {
        crate::encode_double(self, d)
    }

    /// # Encodes a text
    ///
    /// Result: total bytes that have been written.
    fn encode_text<T>(&mut self, s: T) -> IoResult<Size> where T: Into<String> {
        crate::encode_text(self, s)
    }

    /// # Encodes a [`DateTime`][Value::DateTime]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::DateTime]: value/enum.Value.html#variant.DateTime
    fn encode_date_time<T>(&mut self, s: T) -> IoResult<Size> where T: Into<String> {
        crate::encode_date_time(self, s)
    }

    /// # Encodes a [`Date`][Value::Date]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Date]: value/enum.Value.html#variant.Date
    fn encode_date<T>(&mut self, s: T) -> IoResult<Size> where T: Into<String> {
        crate::encode_date(self, s)
    }

    /// # Encodes a [`Time`][Value::Time]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Time]: value/enum.Value.html#variant.Time
    fn encode_time<T>(&mut self, s: T) -> IoResult<Size> where T: Into<String> {
        crate::encode_time(self, s)
    }

    /// # Encodes a [`DecimalStr`][Value::DecimalStr]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::DecimalStr]: value/enum.Value.html#variant.DecimalStr
    fn encode_decimal_str<T>(&mut self, s: T) -> IoResult<Size> where T: Into<String> {
        crate::encode_decimal_str(self, s)
    }

    /// # Encodes a [`Blob`][Value::Blob]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Blob]: value/enum.Value.html#variant.Blob
    fn encode_blob<T>(&mut self, bytes: T) -> IoResult<Size> where T: Into<Blob> {
        crate::encode_blob(self, bytes)
    }

    /// # Encodes a [`List`][Value::List]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::List]: value/enum.Value.html#variant.List
    fn encode_list<T>(&mut self, list: T) -> IoResult<Size> where T: Into<List> {
        crate::encode_list(self, list)
    }

    /// # Encodes a [`Map`][Value::Map]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Map]: value/enum.Value.html#variant.Map
    fn encode_map<T>(&mut self, map: T) -> IoResult<Size> where T: Into<Map> {
        crate::encode_map(self, map)
    }

    /// # Encodes an [`Object`][Value::Object]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [Value::Object]: value/enum.Value.html#variant.Object
    fn encode_object<T>(&mut self, object: T) -> IoResult<Size> where T: Into<Object> {
        crate::encode_object(self, object)
    }

}

impl Encoder for std::fs::File {}
impl<'a> Encoder for &'a std::fs::File {}
impl<W: Write> Encoder for std::io::BufWriter<W> {}
impl<'a> Encoder for std::io::Cursor<&'a mut [u8]> {}
impl<'a> Encoder for std::io::Cursor<&'a mut Blob> {}
impl Encoder for std::io::Cursor<Blob> {}
impl Encoder for std::io::Cursor<Box<[u8]>> {}
impl<W: Write + ?Sized> Encoder for Box<W> {}
impl<'a> Encoder for &'a mut [u8] {}
impl Encoder for Blob {}
impl Encoder for std::io::Sink {}
impl Encoder for std::io::Stdout {}
impl<'a> Encoder for std::io::StdoutLock<'a> {}
impl Encoder for std::io::Stderr {}
impl<'a> Encoder for std::io::StderrLock<'a> {}
impl Encoder for std::net::TcpStream {}
impl<'a> Encoder for &'a std::net::TcpStream {}
impl Encoder for std::process::ChildStdin {}
#[cfg(unix)]
impl Encoder for std::os::unix::net::UnixStream {}
#[cfg(unix)]
impl<'a> Encoder for &'a std::os::unix::net::UnixStream {}
