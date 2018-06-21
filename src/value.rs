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

/// # Converts a value to its underlying byte slice
macro_rules! as_bytes { ($type: ty, $v: expr) => {{
    unsafe { mem::transmute::<&$type, &[u8; mem::size_of::<$type>() ]>(&$v) }
}};}

/// # Writes an integer value
///
/// Returns: number of byte written, as u32.
macro_rules! write_integer { ($type: ty, $v: expr, $buf: expr) => {{
    let bytes = as_bytes!($type, $v.to_be());
    match $buf.write(bytes) {
        Ok(count) => match count == bytes.len() {
            true => Ok(count as u32),
            false => Err(Error::new(
                ErrorKind::WriteZero, format!("write_integer!() -> expected to write {} byte(s); result: {}", bytes.len(), count)
            )),
        },
        Err(err) => Err(err),
    }
}};}

impl<'a> Value<'a> {

    /// # Max data size, in bytes
    pub const MAX_DATA_SIZE: u32 = ::std::i32::MAX as u32;

    /// # TODO
    pub fn write(&self, buf: &mut Write) -> Result<u32, Error> {
        match *self {
            Value::Null => write_integer!(u8, NULL, buf),
            Value::True => write_integer!(u8, TRUE, buf),
            Value::False => write_integer!(u8, FALSE, buf),
            Value::U8(u) => {
                write_integer!(u8, U8, buf)?;
                write_integer!(u8, u, buf)
            },
            Value::I8(i) => {
                write_integer!(u8, I8, buf)?;
                write_integer!(i8, i, buf)
            },
            Value::U16(u) => {
                write_integer!(u8, U16, buf)?;
                write_integer!(u16, u, buf)
            },
            Value::I16(i) => {
                write_integer!(u8, I16, buf)?;
                write_integer!(i16, i, buf)
            },
            Value::U32(u) => {
                write_integer!(u8, U32, buf)?;
                write_integer!(u32, u, buf)
            },
            Value::I32(i) => {
                write_integer!(u8, I32, buf)?;
                write_integer!(i32, i, buf)
            },
            Value::Float(f) => {
                write_integer!(u8, FLOAT, buf)?;
                write_integer!(u32, f.to_bits(), buf)
            },
            Value::U64(u) => {
                write_integer!(u8, U64, buf)?;
                write_integer!(u64, u, buf)
            },
            Value::I64(i) => {
                write_integer!(u8, I64, buf)?;
                write_integer!(i64, i, buf)
            },
            Value::Double(f) => {
                write_integer!(u8, DOUBLE, buf)?;
                write_integer!(u64, f.to_bits(), buf)
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
    fn write_size(size: u32, buf: &mut Write) -> Result<u32, Error> {
        if size <= ::std::i8::MAX as u32 {
            write_integer!(u8, size as u8, buf)
        } else {
            write_integer!(i32, size as i32, buf)
        }
    }

    /// # TODO
    fn write_str(ty: u8, s: &str, buf: &mut Write) -> Result<u32, Error> {
        let bytes = s.as_bytes();
        let str_len = bytes.len() as u32;
        if str_len > Self::MAX_DATA_SIZE {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_str() -> string too large ({} bytes)", str_len)));
        }

        let total_size = 1 + if str_len <= ::std::i8::MAX as u32 { 1 } else { 4 } + str_len + 1;

        // Type
        let written = buf.write(&[ty])? as u32;
        if written != 1 {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_str() -> expected to write 1 byte; result: {}", written)));
        }

        // Size
        // Note that null terminator does NOT count
        Self::write_size(str_len, buf)?;

        // Data
        let written = buf.write(bytes)? as u32;
        if written != str_len {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_str() -> expected to write {} byte(s); result: {}", str_len, written)));
        }

        // Null terminator
        let written = buf.write(&[0])? as u32;
        if written != 1 {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_str() -> expected to write 1 byte; result: {}", written)));
        }

        Ok(total_size)
    }

    /// # TODO
    fn write_blob(bytes: &[u8], buf: &mut Write) -> Result<u32, Error> {
        let len = bytes.len() as u32;
        if len > Self::MAX_DATA_SIZE {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_blob() -> too large: {} byte(s)", len)));
        }

        let mut bytes_written = 0;

        // Type
        match buf.write(&[BLOB])? as u32 {
            1 => bytes_written += 1,
            other => return Err(Error::new(ErrorKind::WriteZero, format!("write_blob() -> expected to write 1 byte; result: {}", other))),
        };

        // Size
        bytes_written += Self::write_size(len, buf)?;

        // Data
        let written = buf.write(bytes)? as u32;
        if written != len {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_blob() -> expected to write {} byte(s); result: {}", len, written)));
        }
        bytes_written += written;

        Ok(bytes_written)
    }

}
