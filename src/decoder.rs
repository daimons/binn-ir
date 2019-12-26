// License: see LICENSE file at root directory of `master` branch

//! # Decoder

use {
    alloc::string::String,
    std::io::Read,

    crate::{Blob, IoResult, List, Map, Object, Value},
};

/// # Decoder
///
/// ## Usage
///
/// ### Decoding any values
///
/// You can use [`decode()`][#decode()] and a `match` to filter values. This function will hand you the values after _finishing_ decoding
/// process.
///
/// ### Decoding desired values
///
/// You can use `decode_*()`. However, please note that: if an un-expected value is detected, the whole decoding operation might be _broken_.
/// It's because those functions just decode the header of a value, and stop if not matched. So at that point, data stream is already broken.
///
/// In contrast, with [`decode()`][#decode()], when you expect an [`Object`][Value::Object] but get a [`List`][Value::List], you can still
/// continue decoding next values.
///
/// [#decode()]: #method.decode
/// [Value::Object]: value/enum.Value.html#variant.Object
/// [Value::List]: value/enum.Value.html#variant.List
pub trait Decoder: Read + Sized {

    /// # Decodes a value
    fn decode(&mut self) -> IoResult<Option<Value>> {
        crate::decode(self)
    }

    /// # Decodes a null
    fn decode_null(&mut self) -> IoResult<Option<()>> {
        crate::decode_null(self)
    }

    /// # Decodes a boolean value
    fn decode_bool(&mut self) -> IoResult<Option<bool>> {
        crate::decode_bool(self)
    }

    /// # Decodes a `u8` value
    fn decode_u8(&mut self) -> IoResult<Option<u8>> {
        crate::decode_u8(self)
    }

    /// # Decodes an `i8` value
    fn decode_i8(&mut self) -> IoResult<Option<i8>> {
        crate::decode_i8(self)
    }

    /// # Decodes a `u16` value
    fn decode_u16(&mut self) -> IoResult<Option<u16>> {
        crate::decode_u16(self)
    }

    /// # Decodes an `i16` value
    fn decode_i16(&mut self) -> IoResult<Option<i16>> {
        crate::decode_i16(self)
    }

    /// # Decodes a `u32` value
    fn decode_u32(&mut self) -> IoResult<Option<u32>> {
        crate::decode_u32(self)
    }
    /// # Decodes an `i32` value
    fn decode_i32(&mut self) -> IoResult<Option<i32>> {
        crate::decode_i32(self)
    }

    /// # Decodes a `u64` value
    fn decode_u64(&mut self) -> IoResult<Option<u64>> {
        crate::decode_u64(self)
    }

    /// # Decodes an `i64` value
    fn decode_i64(&mut self) -> IoResult<Option<i64>> {
        crate::decode_i64(self)
    }

    /// # Decodes a [`Float`][Value::Float]
    ///
    /// [Value::Float]: value/enum.Value.html#variant.Float
    fn decode_float(&mut self) -> IoResult<Option<f32>> {
        crate::decode_float(self)
    }

    /// # Decodes a [`Double`]
    ///
    /// [Value::Double]: value/enum.Value.html#variant.Double
    fn decode_double(&mut self) -> IoResult<Option<f64>> {
        crate::decode_double(self)
    }

    /// # Decodes a text
    fn decode_text(&mut self) -> IoResult<Option<String>> {
        crate::decode_text(self)
    }

    /// # Decodes a [`DateTime`][Value::DateTime]
    ///
    /// [Value::DateTime]: value/enum.Value.html#variant.DateTime
    fn decode_date_time(&mut self) -> IoResult<Option<String>> {
        crate::decode_date_time(self)
    }

    /// # Decodes a [`Date`][Value::Date]
    ///
    /// [Value::Date]: value/enum.Value.html#variant.Date
    fn decode_date(&mut self) -> IoResult<Option<String>> {
        crate::decode_date(self)
    }

    /// # Decodes a [`Time`][Value::Time]
    ///
    /// [Value::Time]: value/enum.Value.html#variant.Time
    fn decode_time(&mut self) -> IoResult<Option<String>> {
        crate::decode_time(self)
    }

    /// # Decodes a [`DecimalStr`][Value::DecimalStr]
    ///
    /// [Value::DecimalStr]: value/enum.Value.html#variant.DecimalStr
    fn decode_decimal_str(&mut self) -> IoResult<Option<String>> {
        crate::decode_decimal_str(self)
    }

    /// # Decodes a [`Blob`][Value::Blob]
    ///
    /// [Value::Blob]: value/enum.Value.html#variant.Blob
    fn decode_blob(&mut self) -> IoResult<Option<Blob>> {
        crate::decode_blob(self)
    }

    /// # Decodes a [`List`][Value::List]
    ///
    /// [Value::List]: value/enum.Value.html#variant.List
    fn decode_list(&mut self) -> IoResult<Option<List>> {
        crate::decode_list(self)
    }

    /// # Decodes a [`Map`][Value::Map]
    ///
    /// [Value::Map]: value/enum.Value.html#variant.Map
    fn decode_map(&mut self) -> IoResult<Option<Map>> {
        crate::decode_map(self)
    }

    /// # Decodes an [`Object`][Value::Object]
    ///
    /// [Value::Object]: value/enum.Value.html#variant.Object
    fn decode_object(&mut self) -> IoResult<Option<Object>> {
        crate::decode_object(self)
    }

}

impl<T> Decoder for T where T: Read {}
