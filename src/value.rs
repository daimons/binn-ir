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
///
/// ## Notes
///
/// - This macro is dangerous: <https://doc.rust-lang.org/std/mem/fn.transmute.html>
/// - Again, this macro is dangerous: <https://doc.rust-lang.org/nomicon/transmutes.html>
/// - TODO: when `::to_bytes()` is stable, switch to it.
macro_rules! as_bytes { ($ty: ty, $v: expr) => {{
    unsafe { mem::transmute::<$ty, [u8; mem::size_of::<$ty>()]>($v) }
}};}

#[test]
fn test_macro_as_bytes() {
    assert!(as_bytes!(u8, 1) == [0x01]);
    assert!(as_bytes!(i8, -0x01) == [0xFF]);
    assert!(as_bytes!(i8, -99) == [0x9D]);
    assert!(as_bytes!(u8, 100) == [0x64]);
    assert!(as_bytes!(i8, -0x64) == [0x9C]);

    assert_eq!(as_bytes!(u16, 0x03E8_u16.to_be()), [0x03, 0xE8]);
    assert_eq!(as_bytes!(u16, 0x270F_u16.to_be()), [0x27, 0x0F]);

    assert_eq!(as_bytes!(i16, (-9999 as i16).to_be()), [0xD8, 0xF1]);
    assert_eq!(as_bytes!(i16, (-2000 as i16).to_be()), [0xF8, 0x30]);
    assert_eq!(as_bytes!(i16, (-1234 as i16).to_be()), [0xFB, 0x2E]);

    assert_eq!(as_bytes!(u32, 0x075B_CD15_u32.to_be()), [0x07, 0x5B, 0xCD, 0x15]);
    assert_eq!(as_bytes!(u32, 0x3ADE_68B1_u32.to_be()), [0x3A, 0xDE, 0x68, 0xB1]);
    assert_eq!(as_bytes!(u32, 0xAABB_FFEE_u32.to_be()), [0xAA, 0xBB, 0xFF, 0xEE]);

    assert_eq!(as_bytes!(u64, 0x8000_0000_0000_0000_u64.to_be()), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(as_bytes!(u64, 0xFFFF_FFFF_0000_0000_u64.to_be()), [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]);

    assert_eq!(as_bytes!(i64, (-3372036854775808 as i64).to_be()), [0xFF, 0xF4, 0x05, 0x26, 0x7D, 0x1A, 0x00, 0x00]);

    assert_eq!(
        as_bytes!(u128, 0xAAFF_0000_CCDD_8899_1234_8678_BCCB_0000_u128.to_be()),
        [0xAA, 0xFF, 0x00, 0x00, 0xCC, 0xDD, 0x88, 0x99, 0x12, 0x34, 0x86, 0x78, 0xBC, 0xCB, 0x00, 0x00]
    );
}

/// # Data size
pub type DataSize = u32;

/// # Writes an integer value
///
/// Returns: number of bytes written, as `DataSize`.
macro_rules! write_integer { ($ty: ty, $v: expr, $buf: expr) => {{
    let bytes = as_bytes!($ty, $v.to_be());
    match $buf.write(&bytes) {
        Ok(count) => match count == bytes.len() {
            true => Ok(count as DataSize),
            false => Err(Error::new(
                ErrorKind::Other, format!("value::write_integer!() -> expected to write {} byte(s); result: {}", bytes.len(), count)
            )),
        },
        Err(err) => Err(err),
    }
}};}

/// # Calculates sum of first value (`DataSize`) with integer(s)
///
/// Result: `Result<DataSize, Error>`.
///
/// If result > [`Value::MAX_DATA_SIZE`], an error is returned.
///
/// [`Value::MAX_DATA_SIZE`]: enum.Value.html#associatedconstant.MAX_DATA_SIZE
macro_rules! sum {
    ($a: expr, $($b: expr),+) => {{
        // Do NOT nest multiple calls to cmp_integers(...); or the compiler will hang up!!!
        let mut result: Result<DataSize, Error> = Ok($a);
        $(
            match result {
                Ok(current) => result = {
                    let b = $b;
                    match cmp_integers!(b, Value::MAX_DATA_SIZE) {
                        Ordering::Greater => Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("sum!() -> too large for: {} + {} (max allowed: {})", &current, &b, Value::MAX_DATA_SIZE)
                        )),
                        _ => match current.checked_add(b as DataSize) {
                            Some(new) => match cmp_integers!(new, Value::MAX_DATA_SIZE) {
                                Ordering::Greater => Err(Error::new(
                                    ErrorKind::InvalidInput,
                                    format!("sum!() -> too large for: {} + {} (max allowed: {})", &current, &b, Value::MAX_DATA_SIZE)
                                )),
                                _ => Ok(new),
                            },
                            None => Err(Error::new(ErrorKind::InvalidInput, format!("sum!() -> can't add {} into {}", &b, &current))),
                        },
                    }
                },
                Err(_) => (),
            };
        )+

        result
    }};
}

impl<'a> Value<'a> {

    /// # Max data size, in bytes
    pub const MAX_DATA_SIZE: DataSize = ::std::i32::MAX as DataSize;

    /// # Calculates length of this value
    pub fn len(&self) -> Result<DataSize, Error> {
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
            Value::Text(t) => sum!(Self::bytes_for_len(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DateTime(dt) => sum!(Self::bytes_for_len(dt.len())?, 2, dt.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Date(d) => sum!(Self::bytes_for_len(d.len())?, 2, d.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Time(t) => sum!(Self::bytes_for_len(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DecimalStr(ds) => sum!(Self::bytes_for_len(ds.len())?, 2, ds.len()),
            // 1 byte for type
            Value::Blob(bytes) => sum!(Self::bytes_for_len(bytes.len())?, 1, bytes.len()),
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
    fn list_len(list: &'a Vec<Value<'a>>) -> Result<DataSize, Error> {
        // Type + count
        let mut result: DataSize = sum!(Self::bytes_for_len(list.len())?, 1)?;
        // Items
        for v in list {
            result = sum!(result, v.len()?)?;
        }
        // The len value itself:
        // First, assume that it needs just 1 byte
        result = sum!(result, 1)?;
        match cmp_integers!(result, ::std::i8::MAX) {
            // Now we need 3 more bytes
            Ordering::Greater => result = sum!(result, 3)?,
            _ => (),
        };
        match result <= Self::MAX_DATA_SIZE {
            true => Ok(result),
            false => Err(Error::new(ErrorKind::InvalidInput, format!("Value::list_len() -> data too large: {} bytes", result))),
        }
    }

    /// # Calculates map length
    fn map_len(map: &'a BTreeMap<i32, Value<'a>>) -> Result<DataSize, Error> {
        // Type + count
        let mut result = sum!(Self::bytes_for_len(map.len())?, 1)?;
        // Items
        for v in map.values() {
            result = sum!(result, mem::size_of::<i32>(), v.len()?)?;
        }
        // The len value itself:
        // First, assume that it needs just 1 byte
        result = sum!(result, 1)?;
        match cmp_integers!(result, ::std::i8::MAX) {
            // Now we need 3 more bytes
            Ordering::Greater => result = sum!(result, 3)?,
            _ => (),
        };
        match result <= Self::MAX_DATA_SIZE {
            true => Ok(result),
            false => Err(Error::new(ErrorKind::InvalidInput, format!("Value::map_len() -> data too large: {} bytes", result))),
        }
    }

    /// # Calculates object length
    fn object_len(object: &'a HashMap<&'a str, Value<'a>>) -> Result<DataSize, Error> {
        // Type + count
        let mut result = sum!(Self::bytes_for_len(object.len())?, 1)?;
        // Items
        for (key, value) in object {
            // Key is limited to 255 bytes; and has NO null terminator
            match cmp_integers!(key.len(), ::std::u8::MAX) {
                Ordering::Greater => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Value::object_len() -> key size is limited to {} bytes; got: {}", ::std::u8::MAX, key.len())
                )),
                _ => (),
            };
            result = sum!(result, key.len(), value.len()?, 1)?;
        }
        // The len value itself:
        // First, assume that it needs just 1 byte
        result = sum!(result, 1)?;
        match cmp_integers!(result, ::std::i8::MAX) {
            // Now we need 3 more bytes
            Ordering::Greater => result = sum!(result, 3)?,
            _ => (),
        };
        match result <= Self::MAX_DATA_SIZE {
            true => Ok(result),
            false => Err(Error::new(ErrorKind::InvalidInput, format!("len() -> data too large: {} bytes", result))),
        }
    }

    /// # Writes this value into a buffer
    ///
    /// Returns the number of bytes written.
    pub fn write(&self, buf: &mut Write) -> Result<DataSize, Error> {
        let expected_result = self.len()?;

        let result = match *self {
            Value::Null => write_integer!(u8, NULL, buf)?,
            Value::True => write_integer!(u8, TRUE, buf)?,
            Value::False => write_integer!(u8, FALSE, buf)?,
            Value::U8(u) => sum!(write_integer!(u8, U8, buf)?, write_integer!(u8, u, buf)?)?,
            Value::I8(i) => sum!(write_integer!(u8, I8, buf)?, write_integer!(i8, i, buf)?)?,
            Value::U16(u) => sum!(write_integer!(u8, U16, buf)?, write_integer!(u16, u, buf)?)?,
            Value::I16(i) => sum!(write_integer!(u8, I16, buf)?, write_integer!(i16, i, buf)?)?,
            Value::U32(u) => sum!(write_integer!(u8, U32, buf)?, write_integer!(u32, u, buf)?)?,
            Value::I32(i) => sum!(write_integer!(u8, I32, buf)?, write_integer!(i32, i, buf)?)?,
            Value::Float(f) => sum!(write_integer!(u8, FLOAT, buf)?, write_integer!(u32, f.to_bits(), buf)?)?,
            Value::U64(u) => sum!(write_integer!(u8, U64, buf)?, write_integer!(u64, u, buf)?)?,
            Value::I64(i) => sum!(write_integer!(u8, I64, buf)?, write_integer!(i64, i, buf)?)?,
            Value::Double(f) => sum!(write_integer!(u8, DOUBLE, buf)?, write_integer!(u64, f.to_bits(), buf)?)?,
            Value::Text(t) => Self::write_str(TEXT, t, buf)?,
            Value::DateTime(dt) => Self::write_str(DATE_TIME, dt, buf)?,
            Value::Date(d) => Self::write_str(DATE, d, buf)?,
            Value::Time(t) => Self::write_str(TIME, t, buf)?,
            Value::DecimalStr(ds) => Self::write_str(DECIMAL_STR, ds, buf)?,
            Value::Blob(bytes) => Self::write_blob(bytes, buf)?,
            Value::List(ref list) => self.write_list(expected_result, list, buf)?,
            Value::Map(ref map) => self.write_map(expected_result, map, buf)?,
            Value::Object(ref object) => self.write_object(expected_result, object, buf)?,
        };

        match result == expected_result {
            true => Ok(result),
            false => Err(Error::new(
                ErrorKind::Other, format!("Value::write() -> expected to write {} bytes, result: {}", expected_result, result)
            )),
        }
    }

    /// # Writes size into the buffer
    fn write_size(size: DataSize, buf: &mut Write) -> Result<DataSize, Error> {
        match cmp_integers!(size, ::std::i8::MAX) {
            Ordering::Greater => write_integer!(i32, size as i32, buf),
            _ => write_integer!(u8, size as u8, buf),
        }
    }

    /// # Writes a string into the buffer
    fn write_str(ty: u8, s: &str, buf: &mut Write) -> Result<DataSize, Error> {
        let bytes = s.as_bytes();
        let str_len = {
            let tmp = bytes.len();
            match cmp_integers!(tmp, Self::MAX_DATA_SIZE) {
                Ordering::Greater => return Err(Error::new(
                    ErrorKind::Other, format!("Value::write_str() -> string too large ({} bytes)", &tmp)
                )),
                _ => tmp as DataSize,
            }
        };

        let total_size = sum!(
            str_len,
            // 1 for type, 1 for null terminator
            2 + if cmp_integers!(str_len, ::std::i8::MAX) == Ordering::Greater { 4 } else { 1 }
        )?;

        // Type
        match buf.write(&[ty])? {
            1 => (),
            other => return Err(Error::new(ErrorKind::Other, format!("Value::write_str() -> expected to write 1 byte; result: {}", &other))),
        };

        // Size
        // Note that null terminator does NOT count
        Self::write_size(str_len, buf)?;

        // Data
        let written = buf.write(bytes)?;
        match cmp_integers!(written, str_len) {
            Ordering::Equal => (),
            _ => return Err(Error::new(
                ErrorKind::Other, format!("Value::write_str() -> expected to write {} byte(s); result: {}", str_len, written)
            )),
        };

        // Null terminator
        match buf.write(&[0])? {
            1 => (),
            other => return Err(Error::new(ErrorKind::Other, format!("Value::write_str() -> expected to write 1 byte; result: {}", &other))),
        };

        Ok(total_size)
    }

    /// # Writes blob into the buffer
    fn write_blob(bytes: &[u8], buf: &mut Write) -> Result<DataSize, Error> {
        let len = {
            let tmp = bytes.len();
            match cmp_integers!(tmp, Self::MAX_DATA_SIZE) {
                Ordering::Greater => return Err(Error::new(ErrorKind::Other, format!("Value::write_blob() -> too large: {} byte(s)", tmp))),
                _ => tmp as DataSize,
            }
        };

        // Type
        let mut bytes_written = match buf.write(&[BLOB])? {
            1 => 1 as DataSize,
            other => return Err(Error::new(ErrorKind::Other, format!("Value::write_blob() -> expected to write 1 byte; result: {}", &other))),
        };

        // Size
        bytes_written = sum!(Self::write_size(len, buf)?, bytes_written)?;

        // Data
        let written = buf.write(bytes)?;
        match cmp_integers!(written, len) {
            Ordering::Equal => (),
            _ => return Err(Error::new(
                ErrorKind::Other, format!("Value::write_blob() -> expected to write {} byte(s); result: {}", &len, &written)
            )),
        };
        bytes_written = sum!(bytes_written, written)?;

        Ok(bytes_written)
    }

    /// # Writes a list into the buffer
    fn write_list(&self, size: DataSize, list: &'a Vec<Value<'a>>, buf: &mut Write) -> Result<DataSize, Error> {
        let mut result = sum!(
            // Type
            write_integer!(u8, LIST, buf)?,
            // Size
            Self::write_size(size, buf)?,
            // Count
            Self::write_size(list.len() as DataSize, buf)?
        )?;

        // Items
        for v in list {
            result = sum!(result, v.write(buf)?)?;
        }

        Ok(result)
    }

    /// # Writes a map into the buffer
    fn write_map(&self, size: DataSize, map: &'a BTreeMap<i32, Value<'a>>, buf: &mut Write) -> Result<DataSize, Error> {
        let mut result = sum!(
            // Type
            write_integer!(u8, MAP, buf)?,
            // Size
            Self::write_size(size, buf)?,
            // Count
            Self::write_size(map.len() as DataSize, buf)?
        )?;

        // Items
        for (key, value) in map {
            result = sum!(result, write_integer!(i32, key, buf)?, value.write(buf)?)?;
        }

        Ok(result)
    }

    /// # Writes an object into the buffer
    ///
    /// Caller _must_ verify that key lengths are valid. Calling [`len()`] will do that, the result can also be passed into `size`.
    ///
    /// [`len()`]: enum.Value.html#method.len
    fn write_object(&self, size: DataSize, object: &'a HashMap<&'a str, Value<'a>>, buf: &mut Write) -> Result<DataSize, Error> {
        let mut result = sum!(
            // Type
            write_integer!(u8, OBJECT, buf)?,
            // Size
            Self::write_size(size, buf)?,
            // Count
            Self::write_size(object.len() as DataSize, buf)?
        )?;

        // Items
        for (key, value) in object {
            // Caller already verified that key len is <= u8::MAX
            result = sum!(result, write_integer!(u8, key.len() as u8, buf)?)?;

            let written = buf.write(key.as_bytes())?;
            match cmp_integers!(written, key.len()) {
                Ordering::Equal => result = sum!(result, written)?,
                _ => return Err(Error::new(
                    ErrorKind::Other, format!("Value::write_object() -> expected to write {} byte(s) of key; result: {}", key.len(), &written)
                )),
            }

            result = sum!(result, value.write(buf)?)?;
        }

        Ok(result)
    }

    /// # TODO
    pub fn read(_buf: &mut Read) -> Result<Self, Error> {
        unimplemented!()
    }

}
