// License: see LICENSE file at root directory of `master` branch

//! # Values

use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
        string::{String, ToString},
        vec::Vec,
    },
    core::{
        cmp::Ordering,
        fmt::{self, Debug, Formatter, Write as FmtWrite},
    },

    crate::{
        // Error, Result,
        cmp::CmpTo,
    },
};

#[cfg(feature="std")]
use {
    core::mem,
    std::io::{self, ErrorKind, Read, Write},

    crate::IoResult,
};

/// # Blob
pub type Blob = Vec<u8>;

/// # List
pub type List = Vec<Value>;

/// # Map
pub type Map = BTreeMap<i32, Value>;

/// # Object
pub type Object = BTreeMap<String, Value>;

const MAX_I8_AS_USIZE: usize = i8::max_value() as usize;
const MAX_I8_AS_U32: u32 = i8::max_value() as u32;

/// # Null
///
/// Storage: [`NO_BYTES`][storage::NO_BYTES]
///
/// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
pub const NULL: u8 = 0b_0000_0000;

/// # True
///
/// Storage: [`NO_BYTES`][storage::NO_BYTES]
///
/// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
pub const TRUE: u8 = 0b_0000_0001;

/// # False
///
/// Storage: [`NO_BYTES`][storage::NO_BYTES]
///
/// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
pub const FALSE: u8 = 0b_0000_0010;

/// # 8-bit unsigned integer
///
/// Storage: [`BYTE`][storage::BYTE]
///
/// [storage::BYTE]: ../storage/constant.BYTE.html
pub const U8: u8 = 0b_0010_0000;

/// # 8-bit signed integer
///
/// Storage: [`BYTE`][storage::BYTE]
///
/// [storage::BYTE]: ../storage/constant.BYTE.html
pub const I8: u8 = 0b_0010_0001;

/// # 16-bit unsigned integer
///
/// Storage: [`WORD`][storage::WORD]
///
/// [storage::WORD]: ../storage/constant.WORD.html
pub const U16: u8 = 0b_0100_0000;

/// # 16-bit signed integer
///
/// Storage: [`WORD`][storage::WORD]
///
/// [storage::WORD]: ../storage/constant.WORD.html
pub const I16: u8 = 0b_0100_0001;

/// # 32-bit unsigned integer
///
/// Storage: [`DWORD`][storage::DWORD]
///
/// [storage::DWORD]: ../storage/constant.DWORD.html
pub const U32: u8 = 0b_0110_0000;

/// # 32-bit signed integer
///
/// Storage: [`DWORD`][storage::DWORD]
///
/// [storage::DWORD]: ../storage/constant.DWORD.html
pub const I32: u8 = 0b_0110_0001;

/// # Float
///
/// Storage: [`DWORD`][storage::DWORD]
///
/// [storage::DWORD]: ../storage/constant.DWORD.html
pub const FLOAT: u8 = 0b_0110_0010;

/// # 64-bit unsigned integer
///
/// Storage: [`QWORD`][storage::QWORD]
///
/// [storage::QWORD]: ../storage/constant.QWORD.html
pub const U64: u8 = 0b_1000_0000;

/// # 64-bit signed integer
///
/// Storage: [`QWORD`][storage::QWORD]
///
/// [storage::QWORD]: ../storage/constant.QWORD.html
pub const I64: u8 = 0b_1000_0001;

/// # Double
///
/// Storage: [`QWORD`][storage::QWORD]
///
/// [storage::QWORD]: ../storage/constant.QWORD.html
pub const DOUBLE: u8 = 0b_1000_0010;

/// # Text
///
/// Storage: [`STRING`][storage::STRING]
///
/// [storage::STRING]: ../storage/constant.STRING.html
pub const TEXT: u8 = 0b_1010_0000;

/// # Date time
///
/// Storage: [`STRING`][storage::STRING]
///
/// [storage::STRING]: ../storage/constant.STRING.html
pub const DATE_TIME: u8 = 0b_1010_0001;

/// # Date
///
/// Storage: [`STRING`][storage::STRING]
///
/// [storage::STRING]: ../storage/constant.STRING.html
pub const DATE: u8 = 0b_1010_0010;

/// # Time
///
/// Storage: [`STRING`][storage::STRING]
///
/// [storage::STRING]: ../storage/constant.STRING.html
pub const TIME: u8 = 0b_1010_0011;

/// # Decimal string
///
/// Storage: [`STRING`][storage::STRING]
///
/// [storage::STRING]: ../storage/constant.STRING.html
pub const DECIMAL_STR: u8 = 0b_1010_0100;

/// # Blob
///
/// Storage: [`BLOB`][storage::BLOB]
///
/// [storage::BLOB]: ../storage/constant.BLOB.html
pub const BLOB: u8 = 0b_1100_0000;

/// # List
///
/// Storage: [`CONTAINER`][storage::CONTAINER]
///
/// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
pub const LIST: u8 = 0b_1110_0000;

/// # Map
///
/// Storage: [`CONTAINER`][storage::CONTAINER]
///
/// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
pub const MAP: u8 = 0b_1110_0001;

/// # Object
///
/// Storage: [`CONTAINER`][storage::CONTAINER]
///
/// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
pub const OBJECT: u8 = 0b_1110_0010;

/// # Size mask
const SIZE_MASK: u32 = 0x_8000_0000_u32;

/// # Object key's max length
pub const OBJECT_KEY_MAX_LEN: usize = 255;

/// # Max data size, in bytes
pub const MAX_DATA_SIZE: u32 = i32::max_value() as u32;

/// # Values
#[derive(Clone, PartialEq)]
pub enum Value {

    /// # Null
    ///
    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`NULL`][value::NULL]
    ///
    /// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
    /// [value::NULL]: constant.NULL.html
    Null,

    /// # True
    ///
    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`TRUE`][value::TRUE]
    ///
    /// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
    /// [value::TRUE]: constant.TRUE.html
    True,

    /// # False
    ///
    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`FALSE`][value::FALSE]
    ///
    /// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
    /// [value::FALSE]: constant.FALSE.html
    False,

    /// # 8-bit unsigned integer
    ///
    /// - Storage: [`BYTE`][storage::BYTE]
    /// - Type: [`U8`][value::U8]
    ///
    /// [storage::BYTE]: ../storage/constant.BYTE.html
    /// [value::U8]: constant.U8.html
    U8(u8),

    /// # 8-bit signed integer
    ///
    /// - Storage: [`BYTE`][storage::BYTE]
    /// - Type: [`I8`][value::I8]
    ///
    /// [storage::BYTE]: ../storage/constant.BYTE.html
    /// [value::I8]: constant.I8.html
    I8(i8),

    /// # 16-bit unsigned integer
    ///
    /// - Storage: [`WORD`][storage::WORD]
    /// - Type: [`U16`][value::U16]
    ///
    /// [storage::WORD]: ../storage/constant.WORD.html
    /// [value::U16]: constant.U16.html
    U16(u16),

    /// # 16-bit signed integer
    ///
    /// - Storage: [`WORD`][storage::WORD]
    /// - Type: [`I16`][value::I16]
    ///
    /// [storage::WORD]: ../storage/constant.WORD.html
    /// [value::I16]: constant.I16.html
    I16(i16),

    /// # 32-bit unsigned integer
    ///
    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`U32`][value::U32]
    ///
    /// [storage::DWORD]: ../storage/constant.DWORD.html
    /// [value::U32]: constant.U32.html
    U32(u32),

    /// # 32-bit signed integer
    ///
    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`I32`][value::I32]
    ///
    /// [storage::DWORD]: ../storage/constant.DWORD.html
    /// [value::I32]: constant.I32.html
    I32(i32),

    /// # Float
    ///
    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`FLOAT`][value::FLOAT]
    ///
    /// [storage::DWORD]: ../storage/constant.DWORD.html
    /// [value::FLOAT]: constant.FLOAT.html
    Float(f32),

    /// # 64-bit unsigned integer
    ///
    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`U64`][value::U64]
    ///
    /// [storage::QWORD]: ../storage/constant.QWORD.html
    /// [value::U64]: constant.U64.html
    U64(u64),

    /// # 64-bit signed integer
    ///
    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`I64`][value::I64]
    ///
    /// [storage::QWORD]: ../storage/constant.QWORD.html
    /// [value::I64]: constant.I64.html
    I64(i64),

    /// # Double
    ///
    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`DOUBLE`][value::DOUBLE]
    ///
    /// [storage::QWORD]: ../storage/constant.QWORD.html
    /// [value::DOUBLE]: constant.DOUBLE.html
    Double(f64),

    /// # Text
    ///
    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`TEXT`][value::TEXT]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::TEXT]: constant.TEXT.html
    Text(String),

    /// # Date time
    ///
    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DATE_TIME`][value::DATE_TIME]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::DATE_TIME]: constant.DATE_TIME.html
    DateTime(String),

    /// # Date
    ///
    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DATE`][value::DATE]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::DATE]: constant.DATE.html
    Date(String),

    /// # Time
    ///
    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`TIME`][value::TIME]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::TIME]: constant.TIME.html
    Time(String),

    /// # Decimal string
    ///
    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DECIMAL_STR`][value::DECIMAL_STR]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::DECIMAL_STR]: constant.DECIMAL_STR.html
    DecimalStr(String),

    /// # Blob
    ///
    /// - Storage: [`BLOB`][storage::BLOB]
    /// - Type: [`BLOB`][value::BLOB]
    ///
    /// [storage::BLOB]: ../storage/constant.BLOB.html
    /// [value::BLOB]: constant.BLOB.html
    Blob(Blob),

    /// # List
    ///
    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`LIST`][value::LIST]
    ///
    /// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
    /// [value::LIST]: constant.LIST.html
    List(List),

    /// # Map
    ///
    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`MAP`][value::MAP]
    ///
    /// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
    /// [value::MAP]: constant.MAP.html
    Map(Map),

    /// # Object
    ///
    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`OBJECT`][value::OBJECT]
    ///
    /// ## Notes
    ///
    /// - Key lengths must be `<=` [`OBJECT_KEY_MAX_LEN`][value::OBJECT_KEY_MAX_LEN].
    ///
    /// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
    /// [value::OBJECT]: constant.OBJECT.html
    /// [value::OBJECT_KEY_MAX_LEN]: constant.OBJECT_KEY_MAX_LEN.html
    Object(Object),

}

impl Debug for Value {

    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), fmt::Error> {
        match self {
            Value::Null => f.write_str("Null"),
            Value::True => f.write_str("True"),
            Value::False => f.write_str("False"),
            Value::U8(u) => write!(f, "U8({})", u),
            Value::I8(i) => write!(f, "I8({})", i),
            Value::U16(u) => write!(f, "U16({})", u),
            Value::I16(i) => write!(f, "I16({})", i),
            Value::U32(u) => write!(f, "U32({})", u),
            Value::I32(i) => write!(f, "I32({})", i),
            Value::Float(float) => write!(f, "Float({})", float),
            Value::U64(u) => write!(f, "U64({})", u),
            Value::I64(i) => write!(f, "I64({})", i),
            Value::Double(d) => write!(f, "Double({})", d),
            Value::Text(s) => write!(f, "Text({:?})", s),
            Value::DateTime(dt) => write!(f, "DateTime({:?})", dt),
            Value::Date(d) => write!(f, "Date({:?})", d),
            Value::Time(t) => write!(f, "Time({:?})", t),
            Value::DecimalStr(ds) => write!(f, "DecimalStr({:?})", ds),
            Value::Blob(blob) => format_debugging_blob(f, blob),
            Value::List(list) => format_debugging_list(f, list),
            Value::Map(map) => format_debugging_map(f, map),
            Value::Object(object) => format_debugging_object(f, object),
        }
    }

}

/// # Formats debugging blob
fn format_debugging_blob(f: &mut Formatter, blob: &Blob) -> core::result::Result<(), fmt::Error> {
    f.write_str("Blob(")?;
    for (i, b) in blob.iter().enumerate() {
        if i > 0 {
            f.write_str(concat!(',', ' '))?;
        }
        write!(f, "0x{:02x}", b)?;
    }
    f.write_char(')')
}

/// # Formats debugging list
fn format_debugging_list(f: &mut Formatter, list: &List) -> core::result::Result<(), fmt::Error> {
    f.write_str("List(")?;
    for (i, v) in list.iter().enumerate() {
        if i > 0 {
            f.write_str(concat!(',', ' '))?;
        }
        v.fmt(f)?;
    }
    f.write_char(')')
}

/// # Formats debugging map
fn format_debugging_map(f: &mut Formatter, map: &Map) -> core::result::Result<(), fmt::Error> {
    f.write_str("Map(")?;
    for (i, (k, v)) in map.iter().enumerate() {
        if i > 0 {
            f.write_str(concat!(',', ' '))?;
        }
        write!(f, "{k}: {v:?}", k=k, v=v)?;
    }
    f.write_char(')')
}

/// # Formats debugging object
fn format_debugging_object(f: &mut Formatter, object: &Object) -> core::result::Result<(), fmt::Error> {
    f.write_str("Object(")?;
    for (i, (k, v)) in object.iter().enumerate() {
        if i > 0 {
            f.write_str(concat!(',', ' '))?;
        }
        write!(f, "{k:?}: {v:?}", k=k, v=v)?;
    }
    f.write_char(')')
}

impl From<()> for Value {

    /// # Converts input to a [`Null`]
    ///
    /// [`Null`]: enum.Value.html#variant.Null
    fn from(_: ()) -> Self {
        Value::Null
    }

}

impl From<bool> for Value {

    /// # Converts input to either [`True`] or [`False`]
    ///
    /// [`True`]: enum.Value.html#variant.True
    /// [`False`]: enum.Value.html#variant.False
    fn from(b: bool) -> Self {
        match b {
            true => Value::True,
            false => Value::False,
        }
    }

}

macro_rules! impl_from_numbers_for_value { ($($number: ty, $variant: tt,)+) => {
    $(
        impl From<$number> for Value {

            fn from(n: $number) -> Self {
                Value::$variant(n)
            }

        }
    )+
}}

impl_from_numbers_for_value!{
    i8, I8, i16, I16, i32, I32, i64, I64,
    u8, U8, u16, U16, u32, U32, u64, U64,
    f32, Float, f64, Double,
}

impl From<String> for Value {

    /// # Converts input to a [`Text`]
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn from(s: String) -> Self {
        Value::Text(s)
    }

}

impl From<&str> for Value {

    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }

}

impl From<Blob> for Value {

    /// # Converts input to a [`Blob`]
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn from(v: Blob) -> Self {
        Value::Blob(v)
    }

}

impl From<List> for Value {

    /// # Converts input to a [`List`]
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn from(list: List) -> Self {
        Value::List(list)
    }

}

impl From<Map> for Value {

    /// # Converts input to a [`Map`]
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn from(map: Map) -> Self {
        Value::Map(map)
    }

}

impl From<Object> for Value {

    /// # Converts input to an [`Object`]
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn from(object: Object) -> Self {
        Value::Object(object)
    }

}

/// # Converts an integer value to big-endian order and writes it into the buffer
///
/// Returns: number of bytes written, as `IoResult<u32>`.
macro_rules! write_int_be { ($ty: ty, $v: expr, $buf: ident) => {{
    let bytes = $v.to_be_bytes();
    $buf.write_all(&bytes).map(|()| bytes.len() as u32)
}};}

/// # Reads an integer value in big-endian format from std::io::Read
///
/// Result: `IoResult<$ty>`.
macro_rules! read_int_be { ($ty: ty, $source: ident) => {{
    let mut buf = [0_u8; mem::size_of::<$ty>()];
    $source.read_exact(&mut buf).map(|()| <$ty>::from_be_bytes(buf))
}};}

/// # Writes size (u32) into the buffer
///
/// Result: number of bytes written - `IoResult<u32>`.
macro_rules! write_size { ($size: expr, $buf: ident) => {{
    let size = $size;
    match size > MAX_I8_AS_U32 {
        true => write_int_be!(u32, size | SIZE_MASK, $buf),
        false => write_int_be!(u8, size as u8, $buf),
    }
}};}

/// # Reads size from source
///
/// Result:
///
/// - First value is size.
/// - Second value is total bytes read (the 'length' of first value).
fn read_size_and_its_length(source: &mut dyn Read) -> IoResult<(u32, u32)> {
    let first_byte = read_int_be!(u8, source)?;
    match first_byte & 0b_1000_0000 {
        0b_1000_0000 => {
            let mut buf = [first_byte, 0, 0, 0];
            source.read_exact(&mut buf[1..]).and_then(|()|
                Ok((u32::from_be_bytes(buf) & !(SIZE_MASK), mem::size_of::<u32>() as u32))
            )
        },
        _ => Ok((u32::from(first_byte), mem::size_of::<u8>() as u32)),
    }
}

/// # Reads size from source
fn read_size(source: &mut dyn Read) -> IoResult<u32> {
    read_size_and_its_length(source).and_then(|(size, _)| Ok(size))
}

#[test]
fn test_read_size_and_its_length() {
    use ::std::io::Cursor;

    const U32_SIZE: u32 = mem::size_of::<u32>() as u32;
    const MAX_U8: u8 = ::std::u8::MAX;

    assert_eq!(read_size_and_its_length(&mut Cursor::new(vec![MAX_U8, MAX_U8, MAX_U8, MAX_U8])).unwrap(), (MAX_DATA_SIZE, U32_SIZE));

    for bytes in vec![
        [0xF0, MAX_U8, MAX_U8, MAX_U8],
        [0x80, MAX_U8, MAX_U8, MAX_U8],
        [MAX_U8, MAX_U8, MAX_U8, 0xF0],
    ] {
        let (size, bytes_of_size) = read_size_and_its_length(&mut Cursor::new(bytes)).unwrap();
        assert!(size < MAX_DATA_SIZE);
        assert_ne!(size, U32_SIZE);
        assert_eq!(bytes_of_size, U32_SIZE);
    }
}

/// # Calculates sum of first value (`u32`) with integer(s)
///
/// Result: `IoResult<u32>`.
///
/// If result > [`MAX_DATA_SIZE`], an error is returned.
///
/// [`MAX_DATA_SIZE`]: constant.MAX_DATA_SIZE.html
macro_rules! sum {
    ($a: expr, $($b: expr),+) => {{
        // Do NOT nest multiple calls to cmp_integers(...); or the compiler will hang up!!!
        let mut result: IoResult<u32> = Ok($a);
        $(
            if let Ok(current) = result {
                result = {
                    let b = $b;
                    match b.cmp_to(&MAX_DATA_SIZE) {
                        Ordering::Greater => Err(io::Error::new(
                            ErrorKind::InvalidData, __!("too large for: {} + {} (max allowed: {})", &current, &b, MAX_DATA_SIZE)
                        )),
                        _ => match current.checked_add(b as u32) {
                            Some(new) => match new.cmp_to(&MAX_DATA_SIZE) {
                                Ordering::Greater => Err(io::Error::new(
                                    ErrorKind::InvalidData, __!("too large for: {} + {} (max allowed: {})", &current, &b, MAX_DATA_SIZE)
                                )),
                                _ => Ok(new),
                            },
                            None => Err(io::Error::new(ErrorKind::InvalidData, __!("can't add {} into {}", &b, &current))),
                        },
                    }
                };
            }
        )+

        result
    }};
}

/// # Makes new vector with capacity
///
/// Returns: `IoResult<Vec<_>>`
macro_rules! new_vec_with_capacity { ($capacity: expr) => {{
    let capacity = $capacity;
    match capacity.cmp_to(&MAX_DATA_SIZE) {
        Ordering::Greater => Err(io::Error::new(
            ErrorKind::WriteZero, __!("cannot allocate a vector with capacity: {} (max allowed: {})", &capacity, MAX_DATA_SIZE)
        )),
        _ => match capacity.cmp_to(&usize::max_value()) {
            Ordering::Greater => Err(io::Error::new(
                ErrorKind::WriteZero, __!("cannot allocate a vector with capacity: {} (max allowed: {})", &capacity, ::std::usize::MAX)
            )),
            _ => Ok(Vec::with_capacity(capacity as usize)),
        },
    }
}};}

/// # Reads data into new vector
///
/// Returns: `IoResult<Vec<_>>`
macro_rules! read_into_new_vec { ($len: expr, $source: ident) => {{
    let len = $len;
    let mut result = new_vec_with_capacity!(len)?;

    // Notes:
    // - `len` was verified via above call to `new_vec_with_capacity!()`, that it must be <= `MAX_DATA_SIZE`
    // - `MAX_DATA_SIZE` should be **tested** to be < `std::u64::MAX`
    match $source.take(u64::from(len)).read_to_end(&mut result) {
        Ok(read) => match read.cmp_to(&len) {
            Ordering::Equal => Ok(result),
            _ => Err(io::Error::new(ErrorKind::WriteZero, __!("expected to read {} bytes, but: {}", &len, &read))),
        },
        Err(err) => Err(io::Error::new(ErrorKind::WriteZero, __!("failed to read {} bytes: {}", &len, &err))),
    }
}};}

/// # Reads a string from source
///
/// Returns: `IoResult<String>`
macro_rules! read_str { ($source: ident) => {{
    // Note that null terminator does NOT count
    let buf = read_into_new_vec!(read_size_and_its_length($source)?.0, $source)?;
    match read_int_be!(u8, $source)? {
        0 => String::from_utf8(buf).map_err(|err| io::Error::new(ErrorKind::InvalidData, __!("failed to decode UTF-8: {}", &err))),
        other => Err(io::Error::new(ErrorKind::InvalidData, __!("expected to read a null terminator ('\\0'), got: {}", &other))),
    }
}};}

/// # Calculates bytes needed for a length
///
/// Result: `IoResult<u32>`
macro_rules! bytes_for_len { ($len: expr) => {{
    let len = $len;
    match len.cmp_to(&MAX_I8_AS_USIZE) {
        Ordering::Greater => match len.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => Err(io::Error::new(ErrorKind::InvalidData, __!("too large: {} bytes", &len))),
            _ => Ok(4_u32),
        },
        _ => Ok(1_u32),
    }
}};}

/// # Decodes a list from source
///
/// Returns: `IoResult<Option<Value>>`
macro_rules! decode_list { ($source: ident) => {{
    let (size, bytes_of_size) = read_size_and_its_length($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(io::Error::new(ErrorKind::InvalidData, __!("invalid declared size: {}", &size)));
    }

    let (item_count, bytes_of_item_count) = read_size_and_its_length($source)?;

    let mut result = vec![];
    let mut read: u32 = sum!(bytes_of_size, bytes_of_item_count)?;
    for item_index in 0..item_count {
        let value = match Value::decode($source)? {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("missing item #{}/{}", &item_index, &item_count))),
        };
        read = match read.checked_add(value.len()?) {
            Some(v) => match size.cmp_to(&v) {
                Ordering::Greater => v,
                _ => return Err(io::Error::new(ErrorKind::InvalidData, __!("expected to read less than {} bytes, got: {}", &size, &v))),
            },
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("expected: {}, current: {}, new item: {:?}", &size, &read, &value))),
        };
        result.push(value);
    }

    // Verify total read (1 byte for header)
    match read.checked_add(1) {
        Some(v) if v == size => Ok(Some(Value::List(result))),
        _ => Err(io::Error::new(ErrorKind::InvalidData, __!("size is declared: {}; but decoded (with or without header): {}", &size, &read))),
    }
}};}

/// # Decodes a map from source
///
/// Returns: `IoResult<Option<Value>>`
macro_rules! decode_map { ($source: ident) => {{
    let (size, bytes_of_size) = read_size_and_its_length($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(io::Error::new(ErrorKind::InvalidData, __!("invalid declared size: {}", &size)));
    }

    let (item_count, bytes_of_item_count) = read_size_and_its_length($source)?;

    let mut result = Map::new();
    let mut read: u32 = sum!(bytes_of_size, bytes_of_item_count)?;
    for _ in 0..item_count {
        let key = read_int_be!(i32, $source)?;
        let value = match Value::decode($source)? {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("missing value for key {}", &key))),
        };
        read = match read.checked_add(sum!(mem::size_of_val(&key) as u32, value.len()?)?) {
            Some(v) => match size.cmp_to(&v) {
                Ordering::Greater => v,
                _ => return Err(io::Error::new(ErrorKind::InvalidData, __!("expected to read less than {} bytes, got: {}", &size, &v))),
            },
            None => return Err(io::Error::new(
                ErrorKind::InvalidData, __!("invalid map size -> expected: {}, current: {}, new item: {} -> {:?}", &size, &read, &key, &value)
            )),
        };
        if let Some(old_value) = result.insert(key, value) {
            return Err(io::Error::new(ErrorKind::InvalidData, __!("duplicate key '{}' of old value: {:?}", &key, &old_value)));
        }
    }

    // Verify total read (1 byte for header)
    match read.checked_add(1) {
        Some(v) if v == size => Ok(Some(Value::Map(result))),
        _ => Err(io::Error::new(ErrorKind::InvalidData, __!("size is declared: {}; but decoded (with or without header): {}", &size, &read))),
    }
}};}

/// # Decodes an object from source
///
/// Returns: `IoResult<Option<Value>>`
macro_rules! decode_object { ($source: ident) => {{
    let (size, bytes_of_size) = read_size_and_its_length($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(io::Error::new(ErrorKind::InvalidData, __!("invalid declared size: {}", &size)));
    }

    let (item_count, bytes_of_item_count) = read_size_and_its_length($source)?;

    let mut result = Object::new();
    let mut read: u32 = sum!(bytes_of_size, bytes_of_item_count)?;
    for _ in 0..item_count {
        // Read key (note that there's NO null terminator)
        let (key_len, bytes_of_key_len) = read_size_and_its_length($source)?;
        match key_len.cmp_to(&OBJECT_KEY_MAX_LEN) {
            Ordering::Greater => return Err(io::Error::new(
                ErrorKind::InvalidData, __!("key length is limited to {} bytes, got: {}", OBJECT_KEY_MAX_LEN, key_len)
            )),
            _ => read = match read.checked_add(sum!(bytes_of_key_len, key_len)?) {
                Some(v) => match size.cmp_to(&v) {
                    Ordering::Greater => v,
                    _ => return Err(io::Error::new(ErrorKind::InvalidData, __!("expected to read less than {} bytes, got: {}", &size, &v))),
                },
                None => return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    __!("invalid object size -> expected: {}, current: {}, new key length: {} + {}", &size, &read, &bytes_of_key_len, &key_len)
                )),
            },
        };
        let key = String::from_utf8(read_into_new_vec!(key_len, $source)?).map_err(|err|
            io::Error::new(ErrorKind::InvalidData, __!("failed to decode UTF-8: {}", &err))
        )?;

        // Read value
        let value = match Value::decode($source)? {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("missing value for key {:?}", &key))),
        };
        read = match read.checked_add(value.len()?) {
            Some(v) => match size.cmp_to(&v) {
                Ordering::Greater => v,
                _ => return Err(io::Error::new(ErrorKind::InvalidData, __!("expected to read less than {} bytes, got: {}", &size, &v))),
            },
            None => return Err(io::Error::new(
                ErrorKind::InvalidData, __!("invalid object size -> expected: {}, current: {}, new value: {:?}", &size, &read, &value)
            )),
        };
        if let Some(old_value) = result.insert(key, value) {
            return Err(io::Error::new(ErrorKind::InvalidData, __!("duplicate key of old value: {:?}", &old_value)));
        }
    }

    // Verify total read (1 byte for header)
    match read.checked_add(1) {
        Some(v) if v == size => Ok(Some(Value::Object(result))),
        _ => Err(io::Error::new(ErrorKind::InvalidData, __!("size is declared: {}; but decoded (with or without header): {}", &size, &read))),
    }
}};}

#[allow(clippy::len_without_is_empty)]
impl Value {

    /// # Calculates length of this value
    pub fn len(&self) -> IoResult<u32> {
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
            Value::Text(ref t) => sum!(bytes_for_len!(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DateTime(ref dt) => sum!(bytes_for_len!(dt.len())?, 2, dt.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Date(ref d) => sum!(bytes_for_len!(d.len())?, 2, d.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Time(ref t) => sum!(bytes_for_len!(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DecimalStr(ref ds) => sum!(bytes_for_len!(ds.len())?, 2, ds.len()),
            // 1 byte for type
            Value::Blob(ref bytes) => sum!(bytes_for_len!(bytes.len())?, 1, bytes.len()),
            Value::List(ref list) => list_len(list),
            Value::Map(ref map) => map_len(map),
            Value::Object(ref object) => object_len(object),
        }
    }

    /// # Encodes this value into a buffer
    ///
    /// Returns the number of bytes written.
    pub fn encode(&self, buf: &mut dyn Write) -> IoResult<u32> {
        let expected_result = self.len()?;

        let result = match *self {
            Value::Null => buf.write_all(&[NULL]).and(Ok(1))?,
            Value::True => buf.write_all(&[TRUE]).and(Ok(1))?,
            Value::False => buf.write_all(&[FALSE]).and(Ok(1))?,
            Value::U8(u) => buf.write_all(&[U8, u]).and(Ok(2))?,
            Value::I8(i) => sum!(write_int_be!(u8, I8, buf)?, write_int_be!(i8, i, buf)?)?,
            Value::U16(u) => sum!(write_int_be!(u8, U16, buf)?, write_int_be!(u16, u, buf)?)?,
            Value::I16(i) => sum!(write_int_be!(u8, I16, buf)?, write_int_be!(i16, i, buf)?)?,
            Value::U32(u) => sum!(write_int_be!(u8, U32, buf)?, write_int_be!(u32, u, buf)?)?,
            Value::I32(i) => sum!(write_int_be!(u8, I32, buf)?, write_int_be!(i32, i, buf)?)?,
            Value::Float(f) => sum!(write_int_be!(u8, FLOAT, buf)?, write_int_be!(u32, f.to_bits(), buf)?)?,
            Value::U64(u) => sum!(write_int_be!(u8, U64, buf)?, write_int_be!(u64, u, buf)?)?,
            Value::I64(i) => sum!(write_int_be!(u8, I64, buf)?, write_int_be!(i64, i, buf)?)?,
            Value::Double(f) => sum!(write_int_be!(u8, DOUBLE, buf)?, write_int_be!(u64, f.to_bits(), buf)?)?,
            Value::Text(ref t) => encode_value_str(TEXT, t.as_str(), buf)?,
            Value::DateTime(ref dt) => encode_value_str(DATE_TIME, dt.as_str(), buf)?,
            Value::Date(ref d) => encode_value_str(DATE, d.as_str(), buf)?,
            Value::Time(ref t) => encode_value_str(TIME, t.as_str(), buf)?,
            Value::DecimalStr(ref ds) => encode_value_str(DECIMAL_STR, ds.as_str(), buf)?,
            Value::Blob(ref bytes) => encode_value_blob(bytes.as_slice(), buf)?,
            Value::List(ref list) => encode_value_list(expected_result, list, buf)?,
            Value::Map(ref map) => encode_value_map(expected_result, map, buf)?,
            Value::Object(ref object) => encode_value_object(expected_result, object, buf)?,
        };

        match result == expected_result {
            true => Ok(result),
            false => Err(io::Error::new(ErrorKind::Other, __!("expected to write {} bytes, result: {}", expected_result, result))),
        }
    }

    /// # Decodes a value from source
    ///
    /// If it returns `Ok(None)`, it means there's no more data to decode.
    pub fn decode(source: &mut dyn Read) -> IoResult<Option<Self>> {
        decode_value(None, source)
    }

}

/// # Encodes a [`Null`]
///
/// Result: total bytes that have been written.
///
/// [`Null`]: enum.Value.html#variant.Null
pub fn encode_null(buf: &mut dyn Write) -> IoResult<u32> {
    Value::Null.encode(buf)
}

/// # Encodes a `bool` via [`True`] or [`False`]
///
/// Result: total bytes that have been written.
///
/// [`True`]: enum.Value.html#variant.True
/// [`False`]: enum.Value.html#variant.False
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
/// [`U8`]: enum.Value.html#variant.U8
pub fn encode_u8<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u8> {
    Value::U8(u.into()).encode(buf)
}

/// # Encodes an [`I8`]
///
/// Result: total bytes that have been written.
///
/// [`I8`]: enum.Value.html#variant.I8
pub fn encode_i8<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i8> {
    Value::I8(i.into()).encode(buf)
}

/// # Encodes a [`U16`]
///
/// Result: total bytes that have been written.
///
/// [`U16`]: enum.Value.html#variant.U16
pub fn encode_u16<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u16> {
    Value::U16(u.into()).encode(buf)
}

/// # Encodes an [`I16`]
///
/// Result: total bytes that have been written.
///
/// [`I16`]: enum.Value.html#variant.I16
pub fn encode_i16<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i16> {
    Value::I16(i.into()).encode(buf)
}

/// # Encodes a [`U32`]
///
/// Result: total bytes that have been written.
///
/// [`U32`]: enum.Value.html#variant.U32
pub fn encode_u32<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u32> {
    Value::U32(u.into()).encode(buf)
}

/// # Encodes an [`I32`]
///
/// Result: total bytes that have been written.
///
/// [`I32`]: enum.Value.html#variant.I32
pub fn encode_i32<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i32> {
    Value::I32(i.into()).encode(buf)
}

/// # Encodes a [`U64`]
///
/// Result: total bytes that have been written.
///
/// [`U64`]: enum.Value.html#variant.U64
pub fn encode_u64<T>(buf: &mut dyn Write, u: T) -> IoResult<u32> where T: Into<u64> {
    Value::U64(u.into()).encode(buf)
}

/// # Encodes an [`I64`]
///
/// Result: total bytes that have been written.
///
/// [`I64`]: enum.Value.html#variant.I64
pub fn encode_i64<T>(buf: &mut dyn Write, i: T) -> IoResult<u32> where T: Into<i64> {
    Value::I64(i.into()).encode(buf)
}

/// # Encodes a [`Float`]
///
/// Result: total bytes that have been written.
///
/// [`Float`]: enum.Value.html#variant.Float
pub fn encode_float<T>(buf: &mut dyn Write, f: T) -> IoResult<u32> where T: Into<f32> {
    Value::Float(f.into()).encode(buf)
}

/// # Encodes a [`Double`]
///
/// Result: total bytes that have been written.
///
/// [`Double`]: enum.Value.html#variant.Double
pub fn encode_double<T>(buf: &mut dyn Write, d: T) -> IoResult<u32> where T: Into<f64> {
    Value::Double(d.into()).encode(buf)
}

/// # Encodes a [`Text`]
///
/// Result: total bytes that have been written.
///
/// [`Text`]: enum.Value.html#variant.Text
pub fn encode_text<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::Text(s.into()).encode(buf)
}

/// # Encodes a [`DateTime`]
///
/// Result: total bytes that have been written.
///
/// [`DateTime`]: enum.Value.html#variant.DateTime
pub fn encode_date_time<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::DateTime(s.into()).encode(buf)
}

/// # Encodes a [`Date`]
///
/// Result: total bytes that have been written.
///
/// [`Date`]: enum.Value.html#variant.Date
pub fn encode_date<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::Date(s.into()).encode(buf)
}

/// # Encodes a [`Time`]
///
/// Result: total bytes that have been written.
///
/// [`Time`]: enum.Value.html#variant.Time
pub fn encode_time<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::Time(s.into()).encode(buf)
}

/// # Encodes a [`DecimalStr`]
///
/// Result: total bytes that have been written.
///
/// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
pub fn encode_decimal_str<T>(buf: &mut dyn Write, s: T) -> IoResult<u32> where T: Into<String> {
    Value::DecimalStr(s.into()).encode(buf)
}

/// # Encodes a [`Blob`]
///
/// Result: total bytes that have been written.
///
/// [`Blob`]: enum.Value.html#variant.Blob
pub fn encode_blob<T>(buf: &mut dyn Write, bytes: T) -> IoResult<u32> where T: Into<Blob> {
    Value::Blob(bytes.into()).encode(buf)
}

/// # Encodes a [`List`]
///
/// Result: total bytes that have been written.
///
/// [`List`]: enum.Value.html#variant.List
pub fn encode_list<T>(buf: &mut dyn Write, list: T) -> IoResult<u32> where T: Into<List> {
    Value::List(list.into()).encode(buf)
}

/// # Encodes a [`Map`]
///
/// Result: total bytes that have been written.
///
/// [`Map`]: enum.Value.html#variant.Map
pub fn encode_map<T>(buf: &mut dyn Write, map: T) -> IoResult<u32> where T: Into<Map> {
    Value::Map(map.into()).encode(buf)
}

/// # Encodes an [`Object`]
///
/// Result: total bytes that have been written.
///
/// [`Object`]: enum.Value.html#variant.Object
pub fn encode_object<T>(buf: &mut dyn Write, object: T) -> IoResult<u32> where T: Into<Object> {
    Value::Object(object.into()).encode(buf)
}

/// # Decodes a value from source
///
/// If `filter` is provided, the function expects that next value from source is one of them, and returns an error if not.
///
/// If `filter` is `None`, the function decodes any value from source.
fn decode_value(filter: Option<&[u8]>, source: &mut dyn Read) -> IoResult<Option<Value>> {
    let source_value = match read_int_be!(u8, source) {
        Ok(source_value) => source_value,
        Err(err) => return match err.kind() {
            ErrorKind::UnexpectedEof => Ok(None),
            _ => Err(err),
        },
    };

    if let Some(ref expected_values) = filter {
        if expected_values.contains(&source_value) == false {
            return Err(io::Error::new(ErrorKind::InvalidData, __!("expected one of: {:?}, got: {}", &expected_values, &source_value)));
        }
    }

    match source_value {
        self::NULL => Ok(Some(Value::Null)),
        self::TRUE => Ok(Some(Value::True)),
        self::FALSE => Ok(Some(Value::False)),
        self::U8 => Ok(Some(Value::U8(read_int_be!(u8, source)?))),
        self::I8 => Ok(Some(Value::I8(read_int_be!(i8, source)?))),
        self::U16 => Ok(Some(Value::U16(read_int_be!(u16, source)?))),
        self::I16 => Ok(Some(Value::I16(read_int_be!(i16, source)?))),
        self::U32 => Ok(Some(Value::U32(read_int_be!(u32, source)?))),
        self::I32 => Ok(Some(Value::I32(read_int_be!(i32, source)?))),
        self::FLOAT => Ok(Some(Value::Float(f32::from_bits(read_int_be!(u32, source)?)))),
        self::U64 => Ok(Some(Value::U64(read_int_be!(u64, source)?))),
        self::I64 => Ok(Some(Value::I64(read_int_be!(i64, source)?))),
        self::DOUBLE => Ok(Some(Value::Double(f64::from_bits(read_int_be!(u64, source)?)))),
        self::TEXT => Ok(Some(Value::Text(read_str!(source)?))),
        self::DATE_TIME => Ok(Some(Value::DateTime(read_str!(source)?))),
        self::DATE => Ok(Some(Value::Date(read_str!(source)?))),
        self::TIME => Ok(Some(Value::Time(read_str!(source)?))),
        self::DECIMAL_STR => Ok(Some(Value::DecimalStr(read_str!(source)?))),
        self::BLOB => Ok(Some(Value::Blob(read_into_new_vec!(read_size(source)?, source)?))),
        self::LIST => decode_list!(source),
        self::MAP => decode_map!(source),
        self::OBJECT => decode_object!(source),
        _ => Err(io::Error::new(ErrorKind::InvalidData, __!("data type is either invalid or not supported: {}", &source_value))),
    }
}

/// # Decodes a [`Null`]
///
/// [`Null`]: enum.Value.html#variant.Null
pub fn decode_null(source: &mut dyn Read) -> IoResult<Option<()>> {
    match decode_value(Some(&[NULL]), source)? {
        Some(Value::Null) => Ok(Some(())),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected null, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a boolean value
pub fn decode_bool(source: &mut dyn Read) -> IoResult<Option<bool>> {
    match decode_value(Some(&[TRUE, FALSE]), source)? {
        Some(Value::True) => Ok(Some(true)),
        Some(Value::False) => Ok(Some(false)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected bool, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u8` value
pub fn decode_u8(source: &mut dyn Read) -> IoResult<Option<u8>> {
    match decode_value(Some(&[U8]), source)? {
        Some(Value::U8(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u8, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i8` value
pub fn decode_i8(source: &mut dyn Read) -> IoResult<Option<i8>> {
    match decode_value(Some(&[I8]), source)? {
        Some(Value::I8(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i8, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u16` value
pub fn decode_u16(source: &mut dyn Read) -> IoResult<Option<u16>> {
    match decode_value(Some(&[U16]), source)? {
        Some(Value::U16(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u16, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i16` value
pub fn decode_i16(source: &mut dyn Read) -> IoResult<Option<i16>> {
    match decode_value(Some(&[I16]), source)? {
        Some(Value::I16(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i16, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u32` value
pub fn decode_u32(source: &mut dyn Read) -> IoResult<Option<u32>> {
    match decode_value(Some(&[U32]), source)? {
        Some(Value::U32(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u32, got: {:?}", &other))),
        None => Ok(None),
    }
}
/// # Decodes an `i32` value
pub fn decode_i32(source: &mut dyn Read) -> IoResult<Option<i32>> {
    match decode_value(Some(&[I32]), source)? {
        Some(Value::I32(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i32, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a `u64` value
pub fn decode_u64(source: &mut dyn Read) -> IoResult<Option<u64>> {
    match decode_value(Some(&[U64]), source)? {
        Some(Value::U64(u)) => Ok(Some(u)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected u64, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an `i64` value
pub fn decode_i64(source: &mut dyn Read) -> IoResult<Option<i64>> {
    match decode_value(Some(&[I64]), source)? {
        Some(Value::I64(i)) => Ok(Some(i)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected i64, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Float`] value
///
/// [`Float`]: enum.Value.html#variant.Float
pub fn decode_float(source: &mut dyn Read) -> IoResult<Option<f32>> {
    match decode_value(Some(&[FLOAT]), source)? {
        Some(Value::Float(f)) => Ok(Some(f)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected float, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Double`] value
///
/// [`Double`]: enum.Value.html#variant.Double
pub fn decode_double(source: &mut dyn Read) -> IoResult<Option<f64>> {
    match decode_value(Some(&[DOUBLE]), source)? {
        Some(Value::Double(d)) => Ok(Some(d)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected double, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Text`]
///
/// [`Text`]: enum.Value.html#variant.Text
pub fn decode_text(source: &mut dyn Read) -> IoResult<Option<String>> {
    match decode_value(Some(&[TEXT]), source)? {
        Some(Value::Text(t)) => Ok(Some(t)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected text, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`DateTime`]
///
/// [`DateTime`]: enum.Value.html#variant.DateTime
pub fn decode_date_time(source: &mut dyn Read) -> IoResult<Option<String>> {
    match decode_value(Some(&[DATE_TIME]), source)? {
        Some(Value::DateTime(dt)) => Ok(Some(dt)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected date_time, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Date`]
///
/// [`Date`]: enum.Value.html#variant.Date
pub fn decode_date(source: &mut dyn Read) -> IoResult<Option<String>> {
    match decode_value(Some(&[DATE]), source)? {
        Some(Value::Date(d)) => Ok(Some(d)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected date, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Time`]
///
/// [`Time`]: enum.Value.html#variant.Time
pub fn decode_time(source: &mut dyn Read) -> IoResult<Option<String>> {
    match decode_value(Some(&[TIME]), source)? {
        Some(Value::Time(t)) => Ok(Some(t)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected time, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`DecimalStr`]
///
/// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
pub fn decode_decimal_str(source: &mut dyn Read) -> IoResult<Option<String>> {
    match decode_value(Some(&[DECIMAL_STR]), source)? {
        Some(Value::DecimalStr(ds)) => Ok(Some(ds)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected decimal_str, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Blob`]
///
/// [`Blob`]: enum.Value.html#variant.Blob
pub fn decode_blob(source: &mut dyn Read) -> IoResult<Option<Blob>> {
    match decode_value(Some(&[BLOB]), source)? {
        Some(Value::Blob(bytes)) => Ok(Some(bytes)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected blob, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`List`]
///
/// [`List`]: enum.Value.html#variant.List
pub fn decode_list(source: &mut dyn Read) -> IoResult<Option<List>> {
    match decode_value(Some(&[LIST]), source)? {
        Some(Value::List(list)) => Ok(Some(list)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected list, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes a [`Map`]
///
/// [`Map`]: enum.Value.html#variant.Map
pub fn decode_map(source: &mut dyn Read) -> IoResult<Option<Map>> {
    match decode_value(Some(&[MAP]), source)? {
        Some(Value::Map(map)) => Ok(Some(map)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected map, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Decodes an [`Object`]
///
/// [`Object`]: enum.Value.html#variant.Object
pub fn decode_object(source: &mut dyn Read) -> IoResult<Option<Object>> {
    match decode_value(Some(&[OBJECT]), source)? {
        Some(Value::Object(object)) => Ok(Some(object)),
        Some(other) => Err(io::Error::new(ErrorKind::InvalidData, __!("expected object, got: {:?}", &other))),
        None => Ok(None),
    }
}

/// # Calculates list length
fn list_len(list: &[Value]) -> IoResult<u32> {
    // Type + count
    let mut result: u32 = sum!(bytes_for_len!(list.len())?, 1)?;
    // Items
    for v in list {
        result = sum!(result, v.len()?)?;
    }
    // The len value itself:
    // First, assume that it needs just 1 byte
    result = sum!(result, 1)?;
    match result > MAX_I8_AS_U32 {
        // Now we need 3 more bytes
        true => result = sum!(result, 3)?,
        false => (),
    };
    match result <= MAX_DATA_SIZE {
        true => Ok(result),
        false => Err(io::Error::new(ErrorKind::InvalidData, __!("data too large: {} bytes", result))),
    }
}

/// # Calculates map length
fn map_len(map: &Map) -> IoResult<u32> {
    // Type + count
    let mut result = sum!(bytes_for_len!(map.len())?, 1)?;
    // Items
    for v in map.values() {
        result = sum!(result, mem::size_of::<i32>(), v.len()?)?;
    }
    // The len value itself:
    // First, assume that it needs just 1 byte
    result = sum!(result, 1)?;
    match result > MAX_I8_AS_U32 {
        // Now we need 3 more bytes
        true => result = sum!(result, 3)?,
        false => (),
    };
    match result <= MAX_DATA_SIZE {
        true => Ok(result),
        false => Err(io::Error::new(ErrorKind::InvalidData, __!("data too large: {} bytes", result))),
    }
}

/// # Calculates object length
fn object_len(object: &Object) -> IoResult<u32> {
    // Type + count
    let mut result = sum!(bytes_for_len!(object.len())?, 1)?;
    // Items
    for (key, value) in object {
        // Key has NO null terminator
        let key_len = key.len();
        if key_len > OBJECT_KEY_MAX_LEN {
            return Err(io::Error::new(ErrorKind::InvalidData, __!("key size is limited to {} bytes; got: {}", OBJECT_KEY_MAX_LEN, &key_len)));
        }
        result = sum!(result, key_len, value.len()?, 1)?;
    }
    // The len value itself:
    // First, assume that it needs just 1 byte
    result = sum!(result, 1)?;
    match result > MAX_I8_AS_U32 {
        // Now we need 3 more bytes
        true => result = sum!(result, 3)?,
        false => (),
    };
    match result <= MAX_DATA_SIZE {
        true => Ok(result),
        false => Err(io::Error::new(ErrorKind::InvalidData, __!("data too large: {} bytes", result))),
    }
}

/// # Encodes a `Value`'s string into the buffer
fn encode_value_str(ty: u8, s: &str, buf: &mut dyn Write) -> IoResult<u32> {
    let bytes = s.as_bytes();
    let str_len = {
        let tmp = bytes.len();
        match tmp.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => return Err(io::Error::new(ErrorKind::Other, __!("string too large ({} bytes)", &tmp))),
            _ => tmp as u32,
        }
    };

    let total_size = sum!(
        str_len,
        // 1 for type, 1 for null terminator
        2 + match str_len > MAX_I8_AS_U32 { true => 4, false => 1 }
    )?;

    // Type
    match buf.write(&[ty])? {
        1 => (),
        other => return Err(io::Error::new(ErrorKind::Other, __!("expected to write 1 byte; result: {}", &other))),
    };

    // Size
    // Note that null terminator does NOT count
    write_size!(str_len, buf)?;

    // Data
    let written = buf.write(bytes)?;
    match written.cmp_to(&str_len) {
        Ordering::Equal => (),
        _ => return Err(io::Error::new(ErrorKind::Other, __!("expected to write {} byte(s); result: {}", str_len, written))),
    };

    // Null terminator
    match buf.write(&[0])? {
        1 => (),
        other => return Err(io::Error::new(ErrorKind::Other, __!("expected to write 1 byte; result: {}", &other))),
    };

    Ok(total_size)
}

/// # Encodes `Value`'s blob into the buffer
fn encode_value_blob(bytes: &[u8], buf: &mut dyn Write) -> IoResult<u32> {
    let len = {
        let tmp = bytes.len();
        match tmp.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => return Err(io::Error::new(ErrorKind::Other, __!("too large: {} byte(s)", tmp))),
            _ => tmp as u32,
        }
    };

    // Type
    let mut bytes_written = match buf.write(&[BLOB])? {
        1 => 1 as u32,
        other => return Err(io::Error::new(ErrorKind::Other, __!("expected to write 1 byte; result: {}", &other))),
    };

    // Size
    bytes_written = sum!(write_size!(len, buf)?, bytes_written)?;

    // Data
    let written = buf.write(bytes)?;
    match written.cmp_to(&len) {
        Ordering::Equal => (),
        _ => return Err(io::Error::new(ErrorKind::Other, __!("expected to write {} byte(s); result: {}", &len, &written))),
    };
    bytes_written = sum!(bytes_written, written)?;

    Ok(bytes_written)
}

/// # Encodes a `Value`'s list into the buffer
fn encode_value_list(size: u32, list: &[Value], buf: &mut dyn Write) -> IoResult<u32> {
    let mut result = sum!(
        // Type
        write_int_be!(u8, LIST, buf)?,
        // Size
        write_size!(size, buf)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called ::len(), which verified the whole
        // container's size.
        write_size!(list.len() as u32, buf)?
    )?;

    // Items
    for v in list {
        result = sum!(result, v.encode(buf)?)?;
    }

    Ok(result)
}

/// # Encodes a `Value`'s map into the buffer
fn encode_value_map(size: u32, map: &Map, buf: &mut dyn Write) -> IoResult<u32> {
    let mut result = sum!(
        // Type
        write_int_be!(u8, MAP, buf)?,
        // Size
        write_size!(size, buf)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called ::len(), which verified the whole
        // container's size.
        write_size!(map.len() as u32, buf)?
    )?;

    // Items
    for (key, value) in map {
        result = sum!(result, write_int_be!(i32, key, buf)?, value.encode(buf)?)?;
    }

    Ok(result)
}

/// # Encodes a `Value`'s object into the buffer
///
/// ## Parameters
///
/// - `size`: should be calculated by `Value::len()`.
fn encode_value_object(size: u32, object: &Object, buf: &mut dyn Write) -> IoResult<u32> {
    let mut result = sum!(
        // Type
        write_int_be!(u8, OBJECT, buf)?,
        // Size
        write_size!(size, buf)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called ::len(), which verified the whole
        // container's size.
        write_size!(object.len() as u32, buf)?
    )?;

    // Items
    for (key, value) in object {
        let key_len = key.len();
        result = match key_len <= OBJECT_KEY_MAX_LEN {
            true => sum!(result, write_int_be!(u8, key_len as u8, buf)?)?,
            false => return Err(io::Error::new(
                ErrorKind::InvalidData, __!("key length is limited to {} bytes, got: {}", OBJECT_KEY_MAX_LEN, &key_len)
            )),
        };

        let written = buf.write(key.as_bytes())?;
        match written.cmp_to(&key_len) {
            Ordering::Equal => result = sum!(result, written)?,
            _ => return Err(io::Error::new(ErrorKind::Other, __!("expected to write {} byte(s) of key; result: {}", &key_len, &written))),
        }

        result = sum!(result, value.encode(buf)?)?;
    }

    Ok(result)
}

/// # Encoder
///
/// ---
///
/// Default implementors are copied from [`Write`]'s.
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
pub trait Encoder: Write + Sized {

    /// # Encodes a value
    ///
    /// Result: total bytes that have been written.
    fn encode<T>(&mut self, value: T) -> IoResult<u32> where T: AsRef<Value> {
        value.as_ref().encode(self)
    }

    /// # Encodes a [`Null`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Null`]: enum.Value.html#variant.Null
    fn encode_null(&mut self) -> IoResult<u32> {
        encode_null(self)
    }

    /// # Encodes a `bool` via [`True`] or [`False`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`True`]: enum.Value.html#variant.True
    /// [`False`]: enum.Value.html#variant.False
    fn encode_bool<T>(&mut self, b: T) -> IoResult<u32> where T: Into<bool> {
        encode_bool(self, b)
    }

    /// # Encodes a [`U8`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U8`]: enum.Value.html#variant.U8
    fn encode_u8<T>(&mut self, u: T) -> IoResult<u32> where T: Into<u8> {
        encode_u8(self, u)
    }

    /// # Encodes an [`I8`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I8`]: enum.Value.html#variant.I8
    fn encode_i8<T>(&mut self, i: T) -> IoResult<u32> where T: Into<i8> {
        encode_i8(self, i)
    }

    /// # Encodes a [`U16`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U16`]: enum.Value.html#variant.U16
    fn encode_u16<T>(&mut self, u: T) -> IoResult<u32> where T: Into<u16> {
        encode_u16(self, u)
    }

    /// # Encodes an [`I16`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I16`]: enum.Value.html#variant.I16
    fn encode_i16<T>(&mut self, i: T) -> IoResult<u32> where T: Into<i16> {
        encode_i16(self, i)
    }

    /// # Encodes a [`U32`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U32`]: enum.Value.html#variant.U32
    fn encode_u32<T>(&mut self, u: T) -> IoResult<u32> where T: Into<u32> {
        encode_u32(self, u)
    }

    /// # Encodes an [`I32`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I32`]: enum.Value.html#variant.I32
    fn encode_i32<T>(&mut self, i: T) -> IoResult<u32> where T: Into<i32> {
        encode_i32(self, i)
    }

    /// # Encodes a [`U64`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U64`]: enum.Value.html#variant.U64
    fn encode_u64<T>(&mut self, u: T) -> IoResult<u32> where T: Into<u64> {
        encode_u64(self, u)
    }

    /// # Encodes an [`I64`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I64`]: enum.Value.html#variant.I64
    fn encode_i64<T>(&mut self, i: T) -> IoResult<u32> where T: Into<i64> {
        encode_i64(self, i)
    }

    /// # Encodes a [`Float`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Float`]: enum.Value.html#variant.Float
    fn encode_float<T>(&mut self, f: T) -> IoResult<u32> where T: Into<f32> {
        encode_float(self, f)
    }

    /// # Encodes a [`Double`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Double`]: enum.Value.html#variant.Double
    fn encode_double<T>(&mut self, d: T) -> IoResult<u32> where T: Into<f64> {
        encode_double(self, d)
    }

    /// # Encodes a [`Text`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn encode_text<T>(&mut self, s: T) -> IoResult<u32> where T: Into<String> {
        encode_text(self, s)
    }

    /// # Encodes a [`DateTime`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`DateTime`]: enum.Value.html#variant.DateTime
    fn encode_date_time<T>(&mut self, s: T) -> IoResult<u32> where T: Into<String> {
        encode_date_time(self, s)
    }

    /// # Encodes a [`Date`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Date`]: enum.Value.html#variant.Date
    fn encode_date<T>(&mut self, s: T) -> IoResult<u32> where T: Into<String> {
        encode_date(self, s)
    }

    /// # Encodes a [`Time`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Time`]: enum.Value.html#variant.Time
    fn encode_time<T>(&mut self, s: T) -> IoResult<u32> where T: Into<String> {
        encode_time(self, s)
    }

    /// # Encodes a [`DecimalStr`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
    fn encode_decimal_str<T>(&mut self, s: T) -> IoResult<u32> where T: Into<String> {
        encode_decimal_str(self, s)
    }

    /// # Encodes a [`Blob`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn encode_blob<T>(&mut self, bytes: T) -> IoResult<u32> where T: Into<Blob> {
        encode_blob(self, bytes)
    }

    /// # Encodes a [`List`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn encode_list<T>(&mut self, list: T) -> IoResult<u32> where T: Into<List> {
        encode_list(self, list)
    }

    /// # Encodes a [`Map`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn encode_map<T>(&mut self, map: T) -> IoResult<u32> where T: Into<Map> {
        encode_map(self, map)
    }

    /// # Encodes an [`Object`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn encode_object<T>(&mut self, object: T) -> IoResult<u32> where T: Into<Object> {
        encode_object(self, object)
    }

}

impl Encoder for ::std::fs::File {}
impl<'a> Encoder for &'a ::std::fs::File {}
impl<W: Write> Encoder for ::std::io::BufWriter<W> {}
impl<'a> Encoder for ::std::io::Cursor<&'a mut [u8]> {}
impl<'a> Encoder for ::std::io::Cursor<&'a mut Blob> {}
impl Encoder for ::std::io::Cursor<Blob> {}
impl Encoder for ::std::io::Cursor<Box<[u8]>> {}
impl<W: Write + ?Sized> Encoder for Box<W> {}
impl<'a> Encoder for &'a mut [u8] {}
impl Encoder for Blob {}
impl Encoder for ::std::io::Sink {}
impl Encoder for ::std::io::Stdout {}
impl<'a> Encoder for ::std::io::StdoutLock<'a> {}
impl Encoder for ::std::io::Stderr {}
impl<'a> Encoder for ::std::io::StderrLock<'a> {}
impl Encoder for ::std::net::TcpStream {}
impl<'a> Encoder for &'a ::std::net::TcpStream {}
impl Encoder for ::std::process::ChildStdin {}
#[cfg(unix)]
impl Encoder for ::std::os::unix::net::UnixStream {}
#[cfg(unix)]
impl<'a> Encoder for &'a ::std::os::unix::net::UnixStream {}

/// # Decoder
///
/// ## Usage
///
/// ### Decoding any values
///
/// You can use [`::decode()`] and a `match` to filter values. This function will hand you the values after _finishing_ decoding process.
///
/// ### Decoding desired values
///
/// You can use `::decode_*()`. However, please note that: if an un-expected value is detected, the whole decoding operation _might_ be
/// **broken**. It's because those functions just decode the header of a value, and stop if not matched. So at that point, data stream _might_
/// already be broken.
///
/// In contrast, with [`::decode()`], when you expect an [`Object`] but get a [`List`], you can still continue decoding next values.
///
/// ### Notes
///
/// - If a `::decode*()` function returns an `Ok(None)`, it means there's no more data to decode.
/// - Default implementors are copied from [`Read`]'s.
///
/// [`::decode()`]: trait.Decoder.html#method.decode
/// [`Object`]: enum.Value.html#variant.Object
/// [`List`]: enum.Value.html#variant.List
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait Decoder: Read + Sized {

    /// # Decodes a value
    fn decode(&mut self) -> IoResult<Option<Value>> {
        Value::decode(self)
    }

    /// # Decodes a [`Null`]
    ///
    /// [`Null`]: enum.Value.html#variant.Null
    fn decode_null(&mut self) -> IoResult<Option<()>> {
        decode_null(self)
    }

    /// # Decodes a boolean value
    fn decode_bool(&mut self) -> IoResult<Option<bool>> {
        decode_bool(self)
    }

    /// # Decodes a `u8` value
    fn decode_u8(&mut self) -> IoResult<Option<u8>> {
        decode_u8(self)
    }

    /// # Decodes an `i8` value
    fn decode_i8(&mut self) -> IoResult<Option<i8>> {
        decode_i8(self)
    }

    /// # Decodes a `u16` value
    fn decode_u16(&mut self) -> IoResult<Option<u16>> {
        decode_u16(self)
    }

    /// # Decodes an `i16` value
    fn decode_i16(&mut self) -> IoResult<Option<i16>> {
        decode_i16(self)
    }

    /// # Decodes a `u32` value
    fn decode_u32(&mut self) -> IoResult<Option<u32>> {
        decode_u32(self)
    }
    /// # Decodes an `i32` value
    fn decode_i32(&mut self) -> IoResult<Option<i32>> {
        decode_i32(self)
    }

    /// # Decodes a `u64` value
    fn decode_u64(&mut self) -> IoResult<Option<u64>> {
        decode_u64(self)
    }

    /// # Decodes an `i64` value
    fn decode_i64(&mut self) -> IoResult<Option<i64>> {
        decode_i64(self)
    }

    /// # Decodes a [`Float`] value
    ///
    /// [`Float`]: enum.Value.html#variant.Float
    fn decode_float(&mut self) -> IoResult<Option<f32>> {
        decode_float(self)
    }

    /// # Decodes a [`Double`] value
    ///
    /// [`Double`]: enum.Value.html#variant.Double
    fn decode_double(&mut self) -> IoResult<Option<f64>> {
        decode_double(self)
    }

    /// # Decodes a [`Text`]
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn decode_text(&mut self) -> IoResult<Option<String>> {
        decode_text(self)
    }

    /// # Decodes a [`DateTime`]
    ///
    /// [`DateTime`]: enum.Value.html#variant.DateTime
    fn decode_date_time(&mut self) -> IoResult<Option<String>> {
        decode_date_time(self)
    }

    /// # Decodes a [`Date`]
    ///
    /// [`Date`]: enum.Value.html#variant.Date
    fn decode_date(&mut self) -> IoResult<Option<String>> {
        decode_date(self)
    }

    /// # Decodes a [`Time`]
    ///
    /// [`Time`]: enum.Value.html#variant.Time
    fn decode_time(&mut self) -> IoResult<Option<String>> {
        decode_time(self)
    }

    /// # Decodes a [`DecimalStr`]
    ///
    /// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
    fn decode_decimal_str(&mut self) -> IoResult<Option<String>> {
        decode_decimal_str(self)
    }

    /// # Decodes a [`Blob`]
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn decode_blob(&mut self) -> IoResult<Option<Blob>> {
        decode_blob(self)
    }

    /// # Decodes a [`List`]
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn decode_list(&mut self) -> IoResult<Option<List>> {
        decode_list(self)
    }

    /// # Decodes a [`Map`]
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn decode_map(&mut self) -> IoResult<Option<Map>> {
        decode_map(self)
    }

    /// # Decodes an [`Object`]
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn decode_object(&mut self) -> IoResult<Option<Object>> {
        decode_object(self)
    }

}

impl Decoder for ::std::fs::File {}
impl<'a> Decoder for &'a ::std::fs::File {}
impl<R: Read> Decoder for ::std::io::BufReader<R> {}
impl<T> Decoder for ::std::io::Cursor<T> where T: AsRef<[u8]> {}
impl<'a, R: Read + ?Sized> Decoder for &'a mut R {}
impl<R: Read + ?Sized> Decoder for Box<R> {}
impl<'a> Decoder for &'a [u8] {}
impl Decoder for ::std::io::Empty {}
impl Decoder for ::std::io::Repeat {}
impl Decoder for ::std::io::Stdin {}
impl<'a> Decoder for ::std::io::StdinLock<'a> {}
impl<T: Read, U: Read> Decoder for ::std::io::Chain<T, U> {}
impl<T: Read> Decoder for ::std::io::Take<T> {}
impl Decoder for ::std::net::TcpStream {}
impl<'a> Decoder for &'a ::std::net::TcpStream {}
impl Decoder for ::std::process::ChildStdout {}
impl Decoder for ::std::process::ChildStderr {}
#[cfg(unix)]
impl Decoder for ::std::os::unix::net::UnixStream {}
#[cfg(unix)]
impl<'a> Decoder for &'a ::std::os::unix::net::UnixStream {}
