// License: see LICENSE file at root directory of `master` branch

//! # Functions

use {
    alloc::string::String,
    std::io::{self, ErrorKind, Read, Write},

    crate::{
        // Error, Result,
        IoResult,
        value::{self, Blob, List, Map, Object, Value},
    },
};

/// # Encodes a [`Null`]
///
/// Result: total bytes that have been written.
///
/// [`Null`]: value/enum.Value.html#variant.Null
pub fn encode_null(buf: &mut dyn Write) -> IoResult<u32> {
    Value::Null.encode(buf)
}

/// # Encodes a `bool` via [`True`] or [`False`]
///
/// Result: total bytes that have been written.
///
/// [`True`]: value/enum.Value.html#variant.True
/// [`False`]: value/enum.Value.html#variant.False
pub fn encode_bool<T>(buf: &mut dyn Write, b: T) -> IoResult<u32> where T: Into<bool> {
    match b.into() {
        true => Value::True.encode(buf),
        false => Value::False.encode(buf),
    }
}

/// # Encodes a [`U8`]
///
/// Result: total bytes that have been written.
///
/// [`U8`]: value/enum.Value.html#variant.U8
pub fn encode_u8<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u8> {
    Value::U8(u.into()).encode(buf)
}

/// # Encodes an [`I8`]
///
/// Result: total bytes that have been written.
///
/// [`I8`]: value/enum.Value.html#variant.I8
pub fn encode_i8<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i8> {
    Value::I8(i.into()).encode(buf)
}

/// # Encodes a [`U16`]
///
/// Result: total bytes that have been written.
///
/// [`U16`]: value/enum.Value.html#variant.U16
pub fn encode_u16<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u16> {
    Value::U16(u.into()).encode(buf)
}

/// # Encodes an [`I16`]
///
/// Result: total bytes that have been written.
///
/// [`I16`]: value/enum.Value.html#variant.I16
pub fn encode_i16<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i16> {
    Value::I16(i.into()).encode(buf)
}

/// # Encodes a [`U32`]
///
/// Result: total bytes that have been written.
///
/// [`U32`]: value/enum.Value.html#variant.U32
pub fn encode_u32<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u32> {
    Value::U32(u.into()).encode(buf)
}

/// # Encodes an [`I32`]
///
/// Result: total bytes that have been written.
///
/// [`I32`]: value/enum.Value.html#variant.I32
pub fn encode_i32<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i32> {
    Value::I32(i.into()).encode(buf)
}

/// # Encodes a [`U64`]
///
/// Result: total bytes that have been written.
///
/// [`U64`]: value/enum.Value.html#variant.U64
pub fn encode_u64<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u64> {
    Value::U64(u.into()).encode(buf)
}

/// # Encodes an [`I64`]
///
/// Result: total bytes that have been written.
///
/// [`I64`]: value/enum.Value.html#variant.I64
pub fn encode_i64<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i64> {
    Value::I64(i.into()).encode(buf)
}

/// # Encodes a [`Float`]
///
/// Result: total bytes that have been written.
///
/// [`Float`]: value/enum.Value.html#variant.Float
pub fn encode_float<T>(buf: &mut dyn Write, f: T) -> IoResult<u32> where T: Into<f32> {
    Value::Float(f.into()).encode(buf)
}

/// # Encodes a [`Double`]
///
/// Result: total bytes that have been written.
///
/// [`Double`]: value/enum.Value.html#variant.Double
pub fn encode_double<T>(buf: &mut dyn Write, d: T) -> IoResult<u32> where T: Into<f64> {
    Value::Double(d.into()).encode(buf)
}

/// # Encodes a [`Text`]
///
/// Result: total bytes that have been written.
///
/// [`Text`]: value/enum.Value.html#variant.Text
pub fn encode_text<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::Text(s.into()).encode(buf)
}

/// # Encodes a [`DateTime`]
///
/// Result: total bytes that have been written.
///
/// [`DateTime`]: value/enum.Value.html#variant.DateTime
pub fn encode_date_time<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::DateTime(s.into()).encode(buf)
}

/// # Encodes a [`Date`]
///
/// Result: total bytes that have been written.
///
/// [`Date`]: value/enum.Value.html#variant.Date
pub fn encode_date<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::Date(s.into()).encode(buf)
}

/// # Encodes a [`Time`]
///
/// Result: total bytes that have been written.
///
/// [`Time`]: value/enum.Value.html#variant.Time
pub fn encode_time<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::Time(s.into()).encode(buf)
}

/// # Encodes a [`DecimalStr`]
///
/// Result: total bytes that have been written.
///
/// [`DecimalStr`]: value/enum.Value.html#variant.DecimalStr
pub fn encode_decimal_str<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::DecimalStr(s.into()).encode(buf)
}

/// # Encodes a [`Blob`]
///
/// Result: total bytes that have been written.
///
/// [`Blob`]: value/enum.Value.html#variant.Blob
pub fn encode_blob<T>(buf: &mut dyn Write, bytes: T) -> IoResult<u32> where T: Into<Blob> {
    Value::Blob(bytes.into()).encode(buf)
}

/// # Encodes a [`List`]
///
/// Result: total bytes that have been written.
///
/// [`List`]: value/enum.Value.html#variant.List
pub fn encode_list<T>(buf: &mut dyn Write, list: T) -> IoResult<u32> where T: Into<List> {
    Value::List(list.into()).encode(buf)
}

/// # Encodes a [`Map`]
///
/// Result: total bytes that have been written.
///
/// [`Map`]: value/enum.Value.html#variant.Map
pub fn encode_map<T>(buf: &mut dyn Write, map: T) -> IoResult<u32> where T: Into<Map> {
    Value::Map(map.into()).encode(buf)
}

/// # Encodes an [`Object`]
///
/// Result: total bytes that have been written.
///
/// [`Object`]: value/enum.Value.html#variant.Object
pub fn encode_object<T>(buf: &mut dyn Write, object: T) -> IoResult<u32> where T: Into<Object> {
    Value::Object(object.into()).encode(buf)
}

/// # Decodes a [`Null`]
///
/// [`Null`]: value/enum.Value.html#variant.Null
pub fn decode_null(source: &mut dyn Read) -> IoResult<Option<()>> {
    match value::decode_value(Some(&[value::NULL]), source)? {
        Some(Value::Null) => Ok(Some(())),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected null, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a boolean value
pub fn decode_bool(source: &mut dyn Read) -> IoResult<Option<bool>> {
    match value::decode_value(Some(&[value::TRUE, value::FALSE]), source)? {
        Some(Value::True) => Ok(Some(true)),
        Some(Value::False) => Ok(Some(false)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected bool, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u8` value
pub fn decode_u8(source: &mut dyn Read) -> IoResult<Option<u8>> {
    match value::decode_value(Some(&[value::U8]), source)? {
        Some(Value::U8(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u8, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i8` value
pub fn decode_i8(source: &mut dyn Read) -> IoResult<Option<i8>> {
    match value::decode_value(Some(&[value::I8]), source)? {
        Some(Value::I8(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i8, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u16` value
pub fn decode_u16(source: &mut dyn Read) -> IoResult<Option<u16>> {
    match value::decode_value(Some(&[value::U16]), source)? {
        Some(Value::U16(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u16, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i16` value
pub fn decode_i16(source: &mut dyn Read) -> IoResult<Option<i16>> {
    match value::decode_value(Some(&[value::I16]), source)? {
        Some(Value::I16(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i16, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u32` value
pub fn decode_u32(source: &mut dyn Read) -> IoResult<Option<u32>> {
    match value::decode_value(Some(&[value::U32]), source)? {
        Some(Value::U32(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u32, got: {:?}", &other))),
        None => Ok(None),
    }
}
/// # Decodes an `i32` value
pub fn decode_i32(source: &mut dyn Read) -> IoResult<Option<i32>> {
    match value::decode_value(Some(&[value::I32]), source)? {
        Some(Value::I32(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i32, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u64` value
pub fn decode_u64(source: &mut dyn Read) -> IoResult<Option<u64>> {
    match value::decode_value(Some(&[value::U64]), source)? {
        Some(Value::U64(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u64, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i64` value
pub fn decode_i64(source: &mut dyn Read) -> IoResult<Option<i64>> {
    match value::decode_value(Some(&[value::I64]), source)? {
        Some(Value::I64(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i64, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Float`] value
///
/// [`Float`]: value/enum.Value.html#variant.Float
pub fn decode_float(source: &mut dyn Read) -> IoResult<Option<f32>> {
    match value::decode_value(Some(&[value::FLOAT]), source)? {
        Some(Value::Float(f)) => Ok(Some(f)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected float, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Double`] value
///
/// [`Double`]: value/enum.Value.html#variant.Double
pub fn decode_double(source: &mut dyn Read) -> IoResult<Option<f64>> {
    match value::decode_value(Some(&[value::DOUBLE]), source)? {
        Some(Value::Double(d)) => Ok(Some(d)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected double, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Text`]
///
/// [`Text`]: value/enum.Value.html#variant.Text
pub fn decode_text(source: &mut dyn Read) -> IoResult<Option<String>> {
    match value::decode_value(Some(&[value::TEXT]), source)? {
        Some(Value::Text(t)) => Ok(Some(t)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected text, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`DateTime`]
///
/// [`DateTime`]: value/enum.Value.html#variant.DateTime
pub fn decode_date_time(source: &mut dyn Read) -> IoResult<Option<String>> {
    match value::decode_value(Some(&[value::DATE_TIME]), source)? {
        Some(Value::DateTime(dt)) => Ok(Some(dt)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected date_time, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Date`]
///
/// [`Date`]: value/enum.Value.html#variant.Date
pub fn decode_date(source: &mut dyn Read) -> IoResult<Option<String>> {
    match value::decode_value(Some(&[value::DATE]), source)? {
        Some(Value::Date(d)) => Ok(Some(d)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected date, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Time`]
///
/// [`Time`]: value/enum.Value.html#variant.Time
pub fn decode_time(source: &mut dyn Read) -> IoResult<Option<String>> {
    match value::decode_value(Some(&[value::TIME]), source)? {
        Some(Value::Time(t)) => Ok(Some(t)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected time, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`DecimalStr`]
///
/// [`DecimalStr`]: value/enum.Value.html#variant.DecimalStr
pub fn decode_decimal_str(source: &mut dyn Read) -> IoResult<Option<String>> {
    match value::decode_value(Some(&[value::DECIMAL_STR]), source)? {
        Some(Value::DecimalStr(ds)) => Ok(Some(ds)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected decimal_str, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Blob`]
///
/// [`Blob`]: value/enum.Value.html#variant.Blob
pub fn decode_blob(source: &mut dyn Read) -> IoResult<Option<Blob>> {
    match value::decode_value(Some(&[value::BLOB]), source)? {
        Some(Value::Blob(bytes)) => Ok(Some(bytes)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected blob, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`List`]
///
/// [`List`]: value/enum.Value.html#variant.List
pub fn decode_list(source: &mut dyn Read) -> IoResult<Option<List>> {
    match value::decode_value(Some(&[value::LIST]), source)? {
        Some(Value::List(list)) => Ok(Some(list)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected list, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Map`]
///
/// [`Map`]: value/enum.Value.html#variant.Map
pub fn decode_map(source: &mut dyn Read) -> IoResult<Option<Map>> {
    match value::decode_value(Some(&[value::MAP]), source)? {
        Some(Value::Map(map)) => Ok(Some(map)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected map, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an [`Object`]
///
/// [`Object`]: value/enum.Value.html#variant.Object
pub fn decode_object(source: &mut dyn Read) -> IoResult<Option<Object>> {
    match value::decode_value(Some(&[value::OBJECT]), source)? {
        Some(Value::Object(object)) => Ok(Some(object)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected object, got: {:?}", &other))),
        None => Ok(None),
    }
}
