// License: see LICENSE file at root directory of `master` branch

//! # Encoding functions

use {
    alloc::string::String,
    std::io::Write,

    crate::{Blob, IoResult, List, Map, Object, Size, Value},
};

/// # Encodes a value
///
/// Result: total bytes that have been written.
pub fn encode<W, T>(stream: &mut W, value: T) -> IoResult<Size> where W: Write, T: Into<Value> {
    value.into().encode(stream)
}

/// # Encodes a [`Null`]
///
/// Result: total bytes that have been written.
///
/// [`Null`]: enum.Value.html#variant.Null
pub fn encode_null<W>(stream: &mut W) -> IoResult<Size> where W: Write {
    Value::Null.encode(stream)
}

/// # Encodes a `bool` via [`True`] or [`False`]
///
/// Result: total bytes that have been written.
///
/// [`True`]: enum.Value.html#variant.True
/// [`False`]: enum.Value.html#variant.False
pub fn encode_bool<W>(stream: &mut W, b: bool) -> IoResult<Size> where W: Write {
    match b {
        true => Value::True.encode(stream),
        false => Value::False.encode(stream),
    }
}

/// # Encodes a [`U8`]
///
/// Result: total bytes that have been written.
///
/// [`U8`]: enum.Value.html#variant.U8
pub fn encode_u8<W>(stream: &mut W, u: u8) -> IoResult<Size> where W: Write {
    Value::U8(u).encode(stream)
}

/// # Encodes an [`I8`]
///
/// Result: total bytes that have been written.
///
/// [`I8`]: enum.Value.html#variant.I8
pub fn encode_i8<W>(stream: &mut W, i: i8) -> IoResult<Size> where W: Write {
    Value::I8(i).encode(stream)
}

/// # Encodes a [`U16`]
///
/// Result: total bytes that have been written.
///
/// [`U16`]: enum.Value.html#variant.U16
pub fn encode_u16<W>(stream: &mut W, u: u16) -> IoResult<Size> where W: Write {
    Value::U16(u).encode(stream)
}

/// # Encodes an [`I16`]
///
/// Result: total bytes that have been written.
///
/// [`I16`]: enum.Value.html#variant.I16
pub fn encode_i16<W>(stream: &mut W, i: i16) -> IoResult<Size> where W: Write {
    Value::I16(i).encode(stream)
}

/// # Encodes a [`U32`]
///
/// Result: total bytes that have been written.
///
/// [`U32`]: enum.Value.html#variant.U32
pub fn encode_u32<W>(stream: &mut W, u: u32) -> IoResult<Size> where W: Write {
    Value::U32(u).encode(stream)
}

/// # Encodes an [`I32`]
///
/// Result: total bytes that have been written.
///
/// [`I32`]: enum.Value.html#variant.I32
pub fn encode_i32<W>(stream: &mut W, i: i32) -> IoResult<Size> where W: Write {
    Value::I32(i).encode(stream)
}

/// # Encodes a [`U64`]
///
/// Result: total bytes that have been written.
///
/// [`U64`]: enum.Value.html#variant.U64
pub fn encode_u64<W>(stream: &mut W, u: u64) -> IoResult<Size> where W: Write {
    Value::U64(u).encode(stream)
}

/// # Encodes an [`I64`]
///
/// Result: total bytes that have been written.
///
/// [`I64`]: enum.Value.html#variant.I64
pub fn encode_i64<W>(stream: &mut W, i: i64) -> IoResult<Size> where W: Write {
    Value::I64(i).encode(stream)
}

/// # Encodes a [`Float`]
///
/// Result: total bytes that have been written.
///
/// [`Float`]: enum.Value.html#variant.Float
pub fn encode_float<W>(stream: &mut W, f: f32) -> IoResult<Size> where W: Write {
    Value::Float(f).encode(stream)
}

/// # Encodes a [`Double`]
///
/// Result: total bytes that have been written.
///
/// [`Double`]: enum.Value.html#variant.Double
pub fn encode_double<W>(stream: &mut W, d: f64) -> IoResult<Size> where W: Write {
    Value::Double(d).encode(stream)
}

/// # Encodes a [`Text`]
///
/// Result: total bytes that have been written.
///
/// [`Text`]: enum.Value.html#variant.Text
pub fn encode_text<W, T>(stream: &mut W, s: T) -> IoResult<Size> where W: Write, T: Into<String> {
    Value::Text(s.into()).encode(stream)
}

/// # Encodes a [`DateTime`]
///
/// Result: total bytes that have been written.
///
/// [`DateTime`]: enum.Value.html#variant.DateTime
pub fn encode_date_time<W, T>(stream: &mut W, s: T) -> IoResult<Size> where W: Write, T: Into<String> {
    Value::DateTime(s.into()).encode(stream)
}

/// # Encodes a [`Date`]
///
/// Result: total bytes that have been written.
///
/// [`Date`]: enum.Value.html#variant.Date
pub fn encode_date<W, T>(stream: &mut W, s: T) -> IoResult<Size> where W: Write, T: Into<String> {
    Value::Date(s.into()).encode(stream)
}

/// # Encodes a [`Time`]
///
/// Result: total bytes that have been written.
///
/// [`Time`]: enum.Value.html#variant.Time
pub fn encode_time<W, T>(stream: &mut W, s: T) -> IoResult<Size> where W: Write, T: Into<String> {
    Value::Time(s.into()).encode(stream)
}

/// # Encodes a [`DecimalStr`]
///
/// Result: total bytes that have been written.
///
/// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
pub fn encode_decimal_str<W, T>(stream: &mut W, s: T) -> IoResult<Size> where W: Write, T: Into<String> {
    Value::DecimalStr(s.into()).encode(stream)
}

/// # Encodes a [`Blob`]
///
/// Result: total bytes that have been written.
///
/// [`Blob`]: enum.Value.html#variant.Blob
pub fn encode_blob<W, T>(stream: &mut W, bytes: T) -> IoResult<Size> where W: Write, T: Into<Blob> {
    Value::Blob(bytes.into()).encode(stream)
}

/// # Encodes a [`List`]
///
/// Result: total bytes that have been written.
///
/// [`List`]: enum.Value.html#variant.List
pub fn encode_list<W, T>(stream: &mut W, list: T) -> IoResult<Size> where W: Write, T: Into<List> {
    Value::List(list.into()).encode(stream)
}

/// # Encodes a [`Map`]
///
/// Result: total bytes that have been written.
///
/// [`Map`]: enum.Value.html#variant.Map
pub fn encode_map<W, T>(stream: &mut W, map: T) -> IoResult<Size> where W: Write, T: Into<Map> {
    Value::Map(map.into()).encode(stream)
}

/// # Encodes an [`Object`]
///
/// Result: total bytes that have been written.
///
/// [`Object`]: enum.Value.html#variant.Object
pub fn encode_object<W, T>(stream: &mut W, object: T) -> IoResult<Size> where W: Write, T: Into<Object> {
    Value::Object(object.into()).encode(stream)
}
