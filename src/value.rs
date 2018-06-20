// License: see LICENSE file at root directory of `master` branch

//! # Values

use std::collections::{BTreeMap, HashMap};
use std::io::{Error, ErrorKind, Write};
use std::mem;

/// # Null
pub const NULL: u8 = 0b_0000_0000;

/// # True
pub const TRUE: u8 = 0b_0000_0001;

/// # False
pub const FALSE: u8 = 0b_0000_0010;

/// # 8-bit unsigned integer
pub const U8: u8 = 0b_0010_0000;

/// # 8-bit signed integer
pub const I8: u8 = 0b_0010_0001;

/// # 16-bit unsigned integer
pub const U16: u8 = 0b_0100_0000;

/// # 16-bit signed integer
pub const I16: u8 = 0b_0100_0001;

/// # 32-bit unsigned integer
pub const U32: u8 = 0b_0110_0000;

/// # 32-bit signed integer
pub const I32: u8 = 0b_0110_0001;

/// # Float
pub const FLOAT: u8 = 0b_0110_0010;

/// # 64-bit unsigned integer
pub const U64: u8 = 0b_1000_0000;

/// # 64-bit signed integer
pub const I64: u8 = 0b_1000_0001;

/// # Double
pub const DOUBLE: u8 = 0b_1000_0010;

/// # Text
pub const TEXT: u8 = 0b_1010_0000;

/// # Date time
pub const DATE_TIME: u8 = 0b_1010_0001;

/// # Date
pub const DATE: u8 = 0b_1010_0010;

/// # Time
pub const TIME: u8 = 0b_1010_0011;

/// # Decimal string
pub const DECIMAL_STR: u8 = 0b_1010_0100;

/// # Blob
pub const BLOB: u8 = 0b_1100_0000;

/// # List
pub const LIST: u8 = 0b_1110_0000;

/// # Map
pub const MAP: u8 = 0b_1110_0001;

/// # Object
pub const OBJECT: u8 = 0b_1110_0010;

/// # Values
pub enum Value<'a> {

    /// # Null
    Null,

    /// # True
    True,

    /// # False
    False,

    /// # 8-bit unsigned integer
    U8(u8),

    /// # 8-bit signed integer
    I8(i8),

    /// # 16-bit unsigned integer
    U16(u16),

    /// # 16-bit signed integer
    I16(i16),

    /// # 32-bit unsigned integer
    U32(u32),

    /// # 32-bit signed integer
    I32(i32),

    /// # Float
    Float(f32),

    /// # 64-bit unsigned integer
    U64(u64),

    /// # 64-bit signed integer
    I64(i64),

    /// # Double
    Double(f64),

    /// # Text
    Text(&'a str),

    /// # Date time
    DateTime(&'a str),

    /// # Date
    Date(&'a str),

    /// # Time
    Time(&'a str),

    /// # Decimal string
    DecimalStr(&'a str),

    /// # Blob
    Blob(&'a [u8]),

    /// # List
    List(Vec<Value<'a>>),

    /// # Map
    Map(BTreeMap<i32, Value<'a>>),

    /// # Object
    Object(HashMap<&'a str, Value<'a>>),

}

macro_rules! as_bytes { ($type: ty, $v: expr) => {{
    unsafe { mem::transmute::<&$type, &[u8; mem::size_of::<$type>() ]>(&$v) }
}};}

macro_rules! write_integer { ($type: ty, $v: expr, $buf: expr) => {{
    let bytes = as_bytes!($type, $v.to_be());
    let result = bytes.len();
    if $buf.len() < result {
        Err(Error::new(ErrorKind::WriteZero, format!("write_integer!() -> output buffer needs at least {} byte(s)", result)))
    } else {
        for i in 0..result {
            $buf[i] = bytes[i]
        }
        Ok(result)
    }
}};}

impl<'a> Value<'a> {

    /// # Max data size, in bytes
    pub const MAX_DATA_SIZE: u32 = ::std::i32::MAX as u32;

    /// # TODO
    pub fn write(&self, buf: &mut [u8]) -> Result<usize, Error> {
        match *self {
            Value::Null => write_integer!(u8, NULL, buf),
            Value::True => write_integer!(u8, TRUE, buf),
            Value::False => write_integer!(u8, FALSE, buf),
            Value::U8(u) => {
                write_integer!(u8, U8, buf)?;
                write_integer!(u8, u, buf[1..])
            },
            Value::I8(i) => {
                write_integer!(u8, I8, buf)?;
                write_integer!(i8, i, buf[1..])
            },
            Value::U16(u) => {
                write_integer!(u8, U16, buf)?;
                write_integer!(u16, u, buf[1..])
            },
            Value::I16(i) => {
                write_integer!(u8, I16, buf)?;
                write_integer!(i16, i, buf[1..])
            },
            Value::U32(u) => {
                write_integer!(u8, U32, buf)?;
                write_integer!(u32, u, buf[1..])
            },
            Value::I32(i) => {
                write_integer!(u8, I32, buf)?;
                write_integer!(i32, i, buf[1..])
            },
            Value::Float(f) => {
                write_integer!(u8, FLOAT, buf)?;
                write_integer!(u32, f.to_bits(), buf[1..])
            },
            Value::U64(u) => {
                write_integer!(u8, U64, buf)?;
                write_integer!(u64, u, buf[1..])
            },
            Value::I64(i) => {
                write_integer!(u8, I64, buf)?;
                write_integer!(i64, i, buf[1..])
            },
            Value::Double(f) => {
                write_integer!(u8, DOUBLE, buf)?;
                write_integer!(u64, f.to_bits(), buf[1..])
            },
            Value::Text(t) => Self::write_str(TEXT, t, buf),
            Value::DateTime(dt) => Self::write_str(DATE_TIME, dt, buf),
            Value::Date(d) => Self::write_str(DATE, d, buf),
            Value::Time(t) => Self::write_str(TIME, t, buf),
            Value::DecimalStr(ds) => Self::write_str(DECIMAL_STR, ds, buf),
            Value::Blob(bytes) => Self::write_blob(bytes, buf),
            _ => unimplemented!(),
        }
    }

    /// # TODO
    fn write_str(ty: u8, s: &str, buf: &mut [u8]) -> Result<usize, Error> {
        let bytes = s.as_bytes();
        let str_len = bytes.len() as u32;
        if str_len > Self::MAX_DATA_SIZE {
            return Err(Error::new(ErrorKind::WriteZero, "write_str() -> string too large"));
        }

        let total_size = 1 + if str_len <= ::std::i8::MAX as u32 { 1 } else { 4 } + str_len + 1;
        if buf.len() < total_size as usize {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_str() -> output buffer needs at least {} bytes", total_size)));
        }

        // Type
        let mut i = 0;
        buf[i] = ty;
        i += 1;

        // Size
        // Note that null terminator does NOT count
        if str_len <= ::std::i8::MAX as u32 {
            buf[i] = str_len as u8;
            i += 1;
        } else {
            write_integer!(i32, str_len as i32, buf[i..])?;
            i += 4;
        }

        // Data
        if let Some(mut buf) = buf.get_mut(i..) {
            let written = buf.write(bytes)? as u32;
            if written != str_len {
                return Err(Error::new(
                    ErrorKind::WriteZero, format!("write_str() -> expected to write {} byte(s); result: {}", str_len, written)
                ));
            }
            i += str_len as usize;
        }

        // Null terminator
        if let Some(item) = buf.get_mut(i) {
            *item = 0;
        }

        Ok(total_size as usize)
    }

    /// # TODO
    fn write_blob(bytes: &[u8], buf: &mut [u8]) -> Result<usize, Error> {
        let len = bytes.len() as u32;
        if len > Self::MAX_DATA_SIZE {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_blob() -> too large: {} byte(s)", len)));
        }

        let total_size = 1 + if len <= ::std::i8::MAX as u32 { 1 } else { 4 } + len;
        if buf.len() < total_size as usize {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_blob() -> output buffer needs at least {} bytes", total_size)));
        }

        // Type
        let mut i = 0;
        buf[i] = BLOB;
        i += 1;

        // Size
        if len <= ::std::i8::MAX as u32 {
            buf[i] = len as u8;
            i += 1;
        } else {
            write_integer!(i32, len as i32, buf[i..])?;
            i += 4;
        }

        // Data
        if let Some(mut buf) = buf.get_mut(i..) {
            let written = buf.write(bytes)? as u32;
            if written != len {
                return Err(Error::new(
                    ErrorKind::WriteZero, format!("write_blob() -> expected to write {} byte(s); result: {}", len, written)
                ));
            }
        }

        Ok(total_size as usize)
    }

}
