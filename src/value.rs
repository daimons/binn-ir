// License: see LICENSE file at root directory of `master` branch

//! # Values

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io::{Error, ErrorKind, Read, Write};
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

/// # Data size
pub type DataSize = u32;

/// # Calculates sum of data size with an integer
///
/// Returns: `Result<DataSize, Error>`
macro_rules! sum {
    ($a: expr, $b: expr) => {{
        // Do NOT nest another cmp_integers($a, ...); or the compiler will hang up!!!
        match cmp_integers!($b, Value::MAX_DATA_SIZE) {
            Ordering::Greater => Err(Error::new(
                ErrorKind::InvalidInput, format!("Data too large: {} (max allowed: {})", $b, Value::MAX_DATA_SIZE)
            )),
            // This guarantees that $a is DataSize
            _ => $a.checked_add($b as DataSize).ok_or(Error::new(ErrorKind::InvalidInput, format!("Can't add {} into {}", $a, $b))),
        }
    }};
}

impl<'a> Value<'a> {

    /// # Max data size, in bytes
    pub const MAX_DATA_SIZE: DataSize = ::std::i32::MAX as DataSize;

    /// # Calculates length of this value
    pub fn len(&self) -> Result<u32, Error> {
        match *self {
            Value::Null => Ok(1),
            Value::True => Ok(1),
            Value::False => Ok(1),
            Value::U8(_) => Ok(2),
            Value::I8(_) => Ok(2),
            Value::U16(_) => Ok(3),
            Value::I16(_) => Ok(3),
            Value::U32(_) => Ok(5),
            Value::I32(_) => Ok(5),
            Value::Float(_) => Ok(5),
            Value::U64(_) => Ok(9),
            Value::I64(_) => Ok(9),
            Value::Double(_) => Ok(9),
            // 1 byte for type, 1 byte for null terminator
            Value::Text(t) => sum!(sum!(Self::bytes_for_len(t.len())?, 2)?, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DateTime(dt) => sum!(sum!(Self::bytes_for_len(dt.len())?, 2)?, dt.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Date(d) => sum!(sum!(Self::bytes_for_len(d.len())?, 2)?, d.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Time(t) => sum!(sum!(Self::bytes_for_len(t.len())?, 2)?, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DecimalStr(ds) => sum!(sum!(Self::bytes_for_len(ds.len())?, 2)?, ds.len()),
            // 1 byte for type
            Value::Blob(bytes) => sum!(sum!(Self::bytes_for_len(bytes.len())?, 1)?, bytes.len()),
            Value::List(ref list) => Self::list_len(list),
            Value::Map(ref map) => Self::map_len(map),
            Value::Object(ref object) => Self::object_len(object),
        }
    }

    /// # Calculates bytes needed for a length
    fn bytes_for_len(len: usize) -> Result<DataSize, Error> {
        match cmp_integers!(len, ::std::i8::MAX) {
            Ordering::Greater => match cmp_integers!(len, Self::MAX_DATA_SIZE) {
                Ordering::Greater => Err(Error::new(ErrorKind::InvalidInput, format!("Value::bytes_for_len() -> too large: {} bytes", len))),
                _ => Ok(4),
            },
            _ => Ok(1),
        }
    }

    /// # Calculates list length
    fn list_len(list: &'a Vec<Value<'a>>) -> Result<u32, Error> {
        // Type + count
        let mut result = 1 + Self::bytes_for_len(list.len())?;
        // Items
        for v in list {
            result += v.len()?;
        }
        // The len value itself:
        // First, assume that it needs just 1 byte
        result += 1;
        if result > ::std::i8::MAX as u32 {
            // Now we need 3 more bytes
            result += 3;
        }
        match result <= Self::MAX_DATA_SIZE {
            true => Ok(result),
            false => Err(Error::new(ErrorKind::InvalidInput, format!("len() -> data too large: {} bytes", result))),
        }
    }

    /// # Calculates map length
    fn map_len(map: &'a BTreeMap<i32, Value<'a>>) -> Result<u32, Error> {
        // Type + count
        let mut result = 1 + Self::bytes_for_len(map.len())?;
        // Items
        for v in map.values() {
            // 4 bytes for key
            result += 4 + v.len()?;
        }
        // The len value itself:
        // First, assume that it needs just 1 byte
        result += 1;
        if result > ::std::i8::MAX as u32 {
            // Now we need 3 more bytes
            result += 3;
        }
        match result <= Self::MAX_DATA_SIZE {
            true => Ok(result),
            false => Err(Error::new(ErrorKind::InvalidInput, format!("len() -> data too large: {} bytes", result))),
        }
    }

    /// # Calculates object length
    fn object_len(object: &'a HashMap<&'a str, Value<'a>>) -> Result<u32, Error> {
        // Type + count
        let mut result = 1 + Self::bytes_for_len(object.len())?;
        // Items
        for (k, v) in object {
            // Key is limited to 255 bytes; and has NO null terminator
            if k.len() > ::std::u8::MAX as usize {
                return Err(Error::new(
                    ErrorKind::InvalidInput, format!("len() -> key size is limited to {} bytes; got: {}", ::std::u8::MAX, k.len())
                ));
            }
            result += 1 + k.len() as u32 + v.len()?;
        }
        // The len value itself:
        // First, assume that it needs just 1 byte
        result += 1;
        if result > ::std::i8::MAX as u32 {
            // Now we need 3 more bytes
            result += 3;
        }
        match result <= Self::MAX_DATA_SIZE {
            true => Ok(result),
            false => Err(Error::new(ErrorKind::InvalidInput, format!("len() -> data too large: {} bytes", result))),
        }
    }

    /// # Writes this value into a buffer
    ///
    /// Returns the number of bytes written.
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
            Value::List(ref list) => self.write_list(list, buf),
            Value::Map(ref map) => self.write_map(map, buf),
            Value::Object(ref object) => self.write_object(object, buf),
        }
    }

    /// # Writes size into the buffer
    fn write_size(size: u32, buf: &mut Write) -> Result<u32, Error> {
        if size <= ::std::i8::MAX as u32 {
            write_integer!(u8, size as u8, buf)
        } else {
            write_integer!(i32, size as i32, buf)
        }
    }

    /// # Writes a string into the buffer
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

    /// # Writes blob into the buffer
    fn write_blob(bytes: &[u8], buf: &mut Write) -> Result<u32, Error> {
        let len = bytes.len() as u32;
        if len > Self::MAX_DATA_SIZE {
            return Err(Error::new(ErrorKind::WriteZero, format!("write_blob() -> too large: {} byte(s)", len)));
        }

        let mut bytes_written = 0;

        // Type
        match buf.write(&[BLOB])? {
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

    /// # Writes a list into the buffer
    fn write_list(&self, list: &'a Vec<Value<'a>>, buf: &mut Write) -> Result<u32, Error> {
        let result = self.len()?;
        // Type
        write_integer!(u8, LIST, buf)?;
        // Size
        Self::write_size(result, buf)?;
        // Count
        Self::write_size(list.len() as u32, buf)?;
        // Items
        for v in list {
            v.write(buf)?;
        }
        Ok(result)
    }

    /// # Writes a map into the buffer
    fn write_map(&self, map: &'a BTreeMap<i32, Value<'a>>, buf: &mut Write) -> Result<u32, Error> {
        let result = self.len()?;
        // Type
        write_integer!(u8, MAP, buf)?;
        // Size
        Self::write_size(result, buf)?;
        // Count
        Self::write_size(map.len() as u32, buf)?;
        // Items
        for (k, v) in map {
            write_integer!(i32, k, buf)?;
            v.write(buf)?;
        }
        Ok(result)
    }

    /// # Writes an object into the buffer
    fn write_object(&self, object: &'a HashMap<&'a str, Value<'a>>, buf: &mut Write) -> Result<u32, Error> {
        let result = self.len()?;
        // Type
        write_integer!(u8, OBJECT, buf)?;
        // Size
        Self::write_size(result, buf)?;
        // Count
        Self::write_size(object.len() as u32, buf)?;
        // Items
        for (k, v) in object {
            // Call to self.len()? above already verified that key len is <= u8::MAX
            write_integer!(u8, k.len() as u8, buf)?;
            let written = buf.write(k.as_bytes())?;
            if written != k.len() {
                return Err(Error::new(
                    ErrorKind::WriteZero, format!("write_object() -> expected to write {} byte(s) of key; result: {}", k.len(), written)
                ));
            }
            v.write(buf)?;
        }
        Ok(result)
    }

    /// # TODO
    pub fn read(_buf: &mut Read) -> Result<Self, Error> {
        unimplemented!()
    }

}
