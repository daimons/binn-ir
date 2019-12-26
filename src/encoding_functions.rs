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
pub fn encode<T>(buf: &mut dyn Write, value: T) -> IoResult<Size> where T: Into<Value> {
    value.into().encode(buf)
}

/// # Encodes a [`Null`]
///
/// Result: total bytes that have been written.
///
/// [`Null`]: enum.Value.html#variant.Null
pub fn encode_null(buf: &mut dyn Write) -> IoResult<Size> {
    Value::Null.encode(buf)
}

/// # Encodes a `bool` via [`True`] or [`False`]
///
/// Result: total bytes that have been written.
///
/// [`True`]: enum.Value.html#variant.True
/// [`False`]: enum.Value.html#variant.False
pub fn encode_bool(buf: &mut dyn Write, b: bool) -> IoResult<Size> {
    match b {
        true => Value::True.encode(buf),
        false => Value::False.encode(buf),
    }
}

/// # Encodes a [`U8`]
///
/// Result: total bytes that have been written.
///
/// [`U8`]: enum.Value.html#variant.U8
pub fn encode_u8(buf: &mut dyn Write, u: u8) -> IoResult<Size> {
    Value::U8(u).encode(buf)
}

/// # Encodes an [`I8`]
///
/// Result: total bytes that have been written.
///
/// [`I8`]: enum.Value.html#variant.I8
pub fn encode_i8(buf: &mut dyn Write, i: i8) -> IoResult<Size> {
    Value::I8(i).encode(buf)
}

/// # Encodes a [`U16`]
///
/// Result: total bytes that have been written.
///
/// [`U16`]: enum.Value.html#variant.U16
pub fn encode_u16(buf: &mut dyn Write, u: u16) -> IoResult<Size> {
    Value::U16(u).encode(buf)
}

/// # Encodes an [`I16`]
///
/// Result: total bytes that have been written.
///
/// [`I16`]: enum.Value.html#variant.I16
pub fn encode_i16(buf: &mut dyn Write, i: i16) -> IoResult<Size> {
    Value::I16(i).encode(buf)
}

/// # Encodes a [`U32`]
///
/// Result: total bytes that have been written.
///
/// [`U32`]: enum.Value.html#variant.U32
pub fn encode_u32(buf: &mut dyn Write, u: u32) -> IoResult<Size> {
    Value::U32(u).encode(buf)
}

/// # Encodes an [`I32`]
///
/// Result: total bytes that have been written.
///
/// [`I32`]: enum.Value.html#variant.I32
pub fn encode_i32(buf: &mut dyn Write, i: i32) -> IoResult<Size> {
    Value::I32(i).encode(buf)
}

/// # Encodes a [`U64`]
///
/// Result: total bytes that have been written.
///
/// [`U64`]: enum.Value.html#variant.U64
pub fn encode_u64(buf: &mut dyn Write, u: u64) -> IoResult<Size> {
    Value::U64(u).encode(buf)
}

/// # Encodes an [`I64`]
///
/// Result: total bytes that have been written.
///
/// [`I64`]: enum.Value.html#variant.I64
pub fn encode_i64(buf: &mut dyn Write, i: i64) -> IoResult<Size> {
    Value::I64(i).encode(buf)
}

/// # Encodes a [`Float`]
///
/// Result: total bytes that have been written.
///
/// [`Float`]: enum.Value.html#variant.Float
pub fn encode_float(buf: &mut dyn Write, f: f32) -> IoResult<Size> {
    Value::Float(f).encode(buf)
}

/// # Encodes a [`Double`]
///
/// Result: total bytes that have been written.
///
/// [`Double`]: enum.Value.html#variant.Double
pub fn encode_double(buf: &mut dyn Write, d: f64) -> IoResult<Size> {
    Value::Double(d).encode(buf)
}

/// # Encodes a [`Text`]
///
/// Result: total bytes that have been written.
///
/// [`Text`]: enum.Value.html#variant.Text
pub fn encode_text<T>(buf: &mut dyn Write, s: T) -> IoResult<Size> where T: Into<String> {
    Value::Text(s.into()).encode(buf)
}

/// # Encodes a [`DateTime`]
///
/// Result: total bytes that have been written.
///
/// [`DateTime`]: enum.Value.html#variant.DateTime
pub fn encode_date_time<T>(buf: &mut dyn Write, s: T) -> IoResult<Size> where T: Into<String> {
    Value::DateTime(s.into()).encode(buf)
}

/// # Encodes a [`Date`]
///
/// Result: total bytes that have been written.
///
/// [`Date`]: enum.Value.html#variant.Date
pub fn encode_date<T>(buf: &mut dyn Write, s: T) -> IoResult<Size> where T: Into<String> {
    Value::Date(s.into()).encode(buf)
}

/// # Encodes a [`Time`]
///
/// Result: total bytes that have been written.
///
/// [`Time`]: enum.Value.html#variant.Time
pub fn encode_time<T>(buf: &mut dyn Write, s: T) -> IoResult<Size> where T: Into<String> {
    Value::Time(s.into()).encode(buf)
}

/// # Encodes a [`DecimalStr`]
///
/// Result: total bytes that have been written.
///
/// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
pub fn encode_decimal_str<T>(buf: &mut dyn Write, s: T) -> IoResult<Size> where T: Into<String> {
    Value::DecimalStr(s.into()).encode(buf)
}

/// # Encodes a [`Blob`]
///
/// Result: total bytes that have been written.
///
/// [`Blob`]: enum.Value.html#variant.Blob
pub fn encode_blob<T>(buf: &mut dyn Write, bytes: T) -> IoResult<Size> where T: Into<Blob> {
    Value::Blob(bytes.into()).encode(buf)
}

/// # Encodes a [`List`]
///
/// Result: total bytes that have been written.
///
/// [`List`]: enum.Value.html#variant.List
pub fn encode_list<T>(buf: &mut dyn Write, list: T) -> IoResult<Size> where T: Into<List> {
    Value::List(list.into()).encode(buf)
}

/// # Encodes a [`Map`]
///
/// Result: total bytes that have been written.
///
/// [`Map`]: enum.Value.html#variant.Map
pub fn encode_map<T>(buf: &mut dyn Write, map: T) -> IoResult<Size> where T: Into<Map> {
    Value::Map(map.into()).encode(buf)
}

/// # Encodes an [`Object`]
///
/// Result: total bytes that have been written.
///
/// [`Object`]: enum.Value.html#variant.Object
pub fn encode_object<T>(buf: &mut dyn Write, object: T) -> IoResult<Size> where T: Into<Object> {
    Value::Object(object.into()).encode(buf)
}
