// License: see LICENSE file at root directory of `master` branch

//! # Decoding functions

use {
    alloc::string::String,
    std::io::{self, ErrorKind, Read},

    crate::{Blob, IoResult, List, Map, Object, Value},
};

/// # Decodes a value from source
///
/// If it returns `Ok(None)`, it means there's no more data to decode.
pub fn decode(source: &mut dyn Read) -> IoResult<Option<Value>> {
    crate::decode_value(None, source)
}

/// # Decodes a [`Null`]
///
/// [`Null`]: enum.Value.html#variant.Null
pub fn decode_null(source: &mut dyn Read) -> IoResult<Option<()>> {
    match crate::decode_value(Some(&[crate::value::NULL]), source)? {
        Some(Value::Null) => Ok(Some(())),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected null, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a boolean value
pub fn decode_bool(source: &mut dyn Read) -> IoResult<Option<bool>> {
    match crate::decode_value(Some(&[crate::value::TRUE, crate::value::FALSE]), source)? {
        Some(Value::True) => Ok(Some(true)),
        Some(Value::False) => Ok(Some(false)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected bool, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u8` value
pub fn decode_u8(source: &mut dyn Read) -> IoResult<Option<u8>> {
    match crate::decode_value(Some(&[crate::value::U8]), source)? {
        Some(Value::U8(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u8, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i8` value
pub fn decode_i8(source: &mut dyn Read) -> IoResult<Option<i8>> {
    match crate::decode_value(Some(&[crate::value::I8]), source)? {
        Some(Value::I8(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i8, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u16` value
pub fn decode_u16(source: &mut dyn Read) -> IoResult<Option<u16>> {
    match crate::decode_value(Some(&[crate::value::U16]), source)? {
        Some(Value::U16(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u16, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i16` value
pub fn decode_i16(source: &mut dyn Read) -> IoResult<Option<i16>> {
    match crate::decode_value(Some(&[crate::value::I16]), source)? {
        Some(Value::I16(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i16, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u32` value
pub fn decode_u32(source: &mut dyn Read) -> IoResult<Option<u32>> {
    match crate::decode_value(Some(&[crate::value::U32]), source)? {
        Some(Value::U32(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u32, got: {:?}", &other))),
        None => Ok(None),
    }
}
/// # Decodes an `i32` value
pub fn decode_i32(source: &mut dyn Read) -> IoResult<Option<i32>> {
    match crate::decode_value(Some(&[crate::value::I32]), source)? {
        Some(Value::I32(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i32, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u64` value
pub fn decode_u64(source: &mut dyn Read) -> IoResult<Option<u64>> {
    match crate::decode_value(Some(&[crate::value::U64]), source)? {
        Some(Value::U64(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u64, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i64` value
pub fn decode_i64(source: &mut dyn Read) -> IoResult<Option<i64>> {
    match crate::decode_value(Some(&[crate::value::I64]), source)? {
        Some(Value::I64(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i64, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Float`] value
///
/// [`Float`]: enum.Value.html#variant.Float
pub fn decode_float(source: &mut dyn Read) -> IoResult<Option<f32>> {
    match crate::decode_value(Some(&[crate::value::FLOAT]), source)? {
        Some(Value::Float(f)) => Ok(Some(f)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected float, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Double`] value
///
/// [`Double`]: enum.Value.html#variant.Double
pub fn decode_double(source: &mut dyn Read) -> IoResult<Option<f64>> {
    match crate::decode_value(Some(&[crate::value::DOUBLE]), source)? {
        Some(Value::Double(d)) => Ok(Some(d)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected double, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Text`]
///
/// [`Text`]: enum.Value.html#variant.Text
pub fn decode_text(source: &mut dyn Read) -> IoResult<Option<String>> {
    match crate::decode_value(Some(&[crate::value::TEXT]), source)? {
        Some(Value::Text(t)) => Ok(Some(t)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected text, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`DateTime`]
///
/// [`DateTime`]: enum.Value.html#variant.DateTime
pub fn decode_date_time(source: &mut dyn Read) -> IoResult<Option<String>> {
    match crate::decode_value(Some(&[crate::value::DATE_TIME]), source)? {
        Some(Value::DateTime(dt)) => Ok(Some(dt)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected date_time, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Date`]
///
/// [`Date`]: enum.Value.html#variant.Date
pub fn decode_date(source: &mut dyn Read) -> IoResult<Option<String>> {
    match crate::decode_value(Some(&[crate::value::DATE]), source)? {
        Some(Value::Date(d)) => Ok(Some(d)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected date, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Time`]
///
/// [`Time`]: enum.Value.html#variant.Time
pub fn decode_time(source: &mut dyn Read) -> IoResult<Option<String>> {
    match crate::decode_value(Some(&[crate::value::TIME]), source)? {
        Some(Value::Time(t)) => Ok(Some(t)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected time, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`DecimalStr`]
///
/// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
pub fn decode_decimal_str(source: &mut dyn Read) -> IoResult<Option<String>> {
    match crate::decode_value(Some(&[crate::value::DECIMAL_STR]), source)? {
        Some(Value::DecimalStr(ds)) => Ok(Some(ds)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected decimal_str, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Blob`]
///
/// [`Blob`]: enum.Value.html#variant.Blob
pub fn decode_blob(source: &mut dyn Read) -> IoResult<Option<Blob>> {
    match crate::decode_value(Some(&[crate::value::BLOB]), source)? {
        Some(Value::Blob(bytes)) => Ok(Some(bytes)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected blob, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`List`]
///
/// [`List`]: enum.Value.html#variant.List
pub fn decode_list(source: &mut dyn Read) -> IoResult<Option<List>> {
    match crate::decode_value(Some(&[crate::value::LIST]), source)? {
        Some(Value::List(list)) => Ok(Some(list)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected list, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Map`]
///
/// [`Map`]: enum.Value.html#variant.Map
pub fn decode_map(source: &mut dyn Read) -> IoResult<Option<Map>> {
    match crate::decode_value(Some(&[crate::value::MAP]), source)? {
        Some(Value::Map(map)) => Ok(Some(map)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected map, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an [`Object`]
///
/// [`Object`]: enum.Value.html#variant.Object
pub fn decode_object(source: &mut dyn Read) -> IoResult<Option<Object>> {
    match crate::decode_value(Some(&[crate::value::OBJECT]), source)? {
        Some(Value::Object(object)) => Ok(Some(object)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected object, got: {:?}", &other))),
        None => Ok(None),
    }
}
