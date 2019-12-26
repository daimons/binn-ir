// License: see LICENSE file at root directory of `master` branch

//! # Value enum

use {
    alloc::string::{String, ToString},
    core::{
        cmp::Ordering,
        fmt::{self, Debug, Formatter, Write as FmtWrite},
        mem,
    },

    crate::{
        Blob, Error, List, Map, Object, Result, Size,
        cmp::CmpTo,
        value::{MAX_DATA_SIZE, OBJECT_KEY_MAX_LEN},
    },
};

#[cfg(feature="std")]
use {
    alloc::vec::Vec,
    std::io::{self, ErrorKind, Read, Write},

    crate::IoResult,
};

const MAX_I8_AS_USIZE: usize = i8::max_value() as usize;
const MAX_I8_AS_U32: Size = i8::max_value() as Size;

/// # Size mask
#[cfg(feature="std")]
const SIZE_MASK: Size = 0x_8000_0000;

/// # Values
#[derive(Clone, PartialEq)]
pub enum Value {

    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`NULL`][value::NULL]
    ///
    /// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
    /// [value::NULL]: constant.NULL.html
    Null,

    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`TRUE`][value::TRUE]
    ///
    /// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
    /// [value::TRUE]: constant.TRUE.html
    True,

    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`FALSE`][value::FALSE]
    ///
    /// [storage::NO_BYTES]: ../storage/constant.NO_BYTES.html
    /// [value::FALSE]: constant.FALSE.html
    False,

    /// - Storage: [`BYTE`][storage::BYTE]
    /// - Type: [`U8`][value::U8]
    ///
    /// [storage::BYTE]: ../storage/constant.BYTE.html
    /// [value::U8]: constant.U8.html
    U8(u8),

    /// - Storage: [`BYTE`][storage::BYTE]
    /// - Type: [`I8`][value::I8]
    ///
    /// [storage::BYTE]: ../storage/constant.BYTE.html
    /// [value::I8]: constant.I8.html
    I8(i8),

    /// - Storage: [`WORD`][storage::WORD]
    /// - Type: [`U16`][value::U16]
    ///
    /// [storage::WORD]: ../storage/constant.WORD.html
    /// [value::U16]: constant.U16.html
    U16(u16),

    /// - Storage: [`WORD`][storage::WORD]
    /// - Type: [`I16`][value::I16]
    ///
    /// [storage::WORD]: ../storage/constant.WORD.html
    /// [value::I16]: constant.I16.html
    I16(i16),

    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`U32`][value::U32]
    ///
    /// [storage::DWORD]: ../storage/constant.DWORD.html
    /// [value::U32]: constant.U32.html
    U32(u32),

    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`I32`][value::I32]
    ///
    /// [storage::DWORD]: ../storage/constant.DWORD.html
    /// [value::I32]: constant.I32.html
    I32(i32),

    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`U64`][value::U64]
    ///
    /// [storage::QWORD]: ../storage/constant.QWORD.html
    /// [value::U64]: constant.U64.html
    U64(u64),

    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`I64`][value::I64]
    ///
    /// [storage::QWORD]: ../storage/constant.QWORD.html
    /// [value::I64]: constant.I64.html
    I64(i64),

    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`FLOAT`][value::FLOAT]
    ///
    /// [storage::DWORD]: ../storage/constant.DWORD.html
    /// [value::FLOAT]: constant.FLOAT.html
    Float(f32),

    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`DOUBLE`][value::DOUBLE]
    ///
    /// [storage::QWORD]: ../storage/constant.QWORD.html
    /// [value::DOUBLE]: constant.DOUBLE.html
    Double(f64),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`TEXT`][value::TEXT]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::TEXT]: constant.TEXT.html
    Text(String),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DATE_TIME`][value::DATE_TIME]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::DATE_TIME]: constant.DATE_TIME.html
    DateTime(String),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DATE`][value::DATE]
    ///
    /// [storage::STRING]: ../storage/constant.STRING.html
    /// [value::DATE]: constant.DATE.html
    Date(String),

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

    /// - Storage: [`BLOB`][storage::BLOB]
    /// - Type: [`BLOB`][value::BLOB]
    ///
    /// [storage::BLOB]: ../storage/constant.BLOB.html
    /// [value::BLOB]: constant.BLOB.html
    Blob(Blob),

    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`LIST`][value::LIST]
    ///
    /// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
    /// [value::LIST]: constant.LIST.html
    List(List),

    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`MAP`][value::MAP]
    ///
    /// [storage::CONTAINER]: ../storage/constant.CONTAINER.html
    /// [value::MAP]: constant.MAP.html
    Map(Map),

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

impl<T> From<Option<T>> for Value where T: Into<Value> {

    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => Value::Null,
        }
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
/// Returns: number of bytes written, as `IoResult<Size>`.
#[cfg(feature="std")]
macro_rules! write_int_be { ($v: expr, $buf: ident) => {{
    let bytes = $v.to_be_bytes();
    $buf.write_all(&bytes).map(|()| bytes.len() as Size)
}};}

/// # Reads an integer value in big-endian format from std::io::Read
///
/// Result: `IoResult<$ty>`.
#[cfg(feature="std")]
macro_rules! read_int_be { ($ty: ty, $source: ident) => {{
    let mut buf = [0_u8; mem::size_of::<$ty>()];
    $source.read_exact(&mut buf).map(|()| <$ty>::from_be_bytes(buf))
}};}

/// # Writes size (u32) into the buffer
///
/// Result: number of bytes written - `IoResult<Size>`.
#[cfg(feature="std")]
macro_rules! write_size { ($size: expr, $buf: ident) => {{
    let size = $size;
    match size > MAX_I8_AS_U32 {
        true => write_int_be!(size | SIZE_MASK, $buf),
        false => write_int_be!(size as u8, $buf),
    }
}};}

/// # Reads size from source
///
/// Result:
///
/// - First value is size.
/// - Second value is total bytes read (the 'length' of first value).
#[cfg(feature="std")]
fn read_size_and_its_length(source: &mut dyn Read) -> IoResult<(Size, Size)> {
    let first_byte = read_int_be!(u8, source)?;
    match first_byte & 0b_1000_0000 {
        0b_1000_0000 => {
            let mut buf = [first_byte, 0, 0, 0];
            source.read_exact(&mut buf[1..]).and_then(|()|
                Ok((Size::from_be_bytes(buf) & !(SIZE_MASK), mem::size_of::<Size>() as Size))
            )
        },
        _ => Ok((Size::from(first_byte), mem::size_of::<u8>() as Size)),
    }
}

/// # Reads size from source
#[cfg(feature="std")]
fn read_size(source: &mut dyn Read) -> IoResult<Size> {
    read_size_and_its_length(source).and_then(|(size, _)| Ok(size))
}

#[test]
#[cfg(feature="std")]
fn test_read_size_and_its_length() {
    use ::std::io::Cursor;

    const U32_SIZE: Size = mem::size_of::<Size>() as Size;
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

/// # Calculates sum of first value (`Size`) with integer(s)
///
/// Result: `IoResult<Size>`.
///
/// If result > [`MAX_DATA_SIZE`], an error is returned.
///
/// [`MAX_DATA_SIZE`]: constant.MAX_DATA_SIZE.html
macro_rules! sum {
    ($a: expr, $($b: expr),+) => {{
        let mut result: Result<Size> = Ok($a);
        $(
            if let Ok(current) = result {
                result = {
                    let b = $b;
                    match b.cmp_to(&MAX_DATA_SIZE) {
                        Ordering::Greater => Err(Error::from(__!("too large for: {} + {} (max allowed: {})", &current, &b, MAX_DATA_SIZE))),
                        _ => match current.checked_add(b as Size) {
                            Some(new) => match new.cmp_to(&MAX_DATA_SIZE) {
                                Ordering::Greater => Err(Error::from(
                                    __!("too large for: {} + {} (max allowed: {})", &current, &b, MAX_DATA_SIZE)
                                )),
                                _ => Ok(new),
                            },
                            None => Err(Error::from(__!("can't add {} into {}", &b, &current))),
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
#[cfg(feature="std")]
macro_rules! new_vec_with_capacity { ($capacity: expr) => {{
    let capacity = $capacity;
    match capacity.cmp_to(&MAX_DATA_SIZE) {
        Ordering::Greater => Err(Error::from(__!("cannot allocate a vector with capacity: {} (max allowed: {})", &capacity, MAX_DATA_SIZE))),
        _ => match capacity.cmp_to(&usize::max_value()) {
            Ordering::Greater => Err(Error::from(
                __!("cannot allocate a vector with capacity: {} (max allowed: {})", &capacity, ::std::usize::MAX)
            )),
            _ => Ok(Vec::with_capacity(capacity as usize)),
        },
    }
}};}

/// # Reads data into new vector
///
/// Returns: `IoResult<Vec<_>>`
#[cfg(feature="std")]
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
#[cfg(feature="std")]
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
/// Result: `Result<Size>`
macro_rules! bytes_for_len { ($len: expr) => {{
    let len = $len;
    match len.cmp_to(&MAX_I8_AS_USIZE) {
        Ordering::Greater => match len.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => Err(Error::from(__!("too large: {} bytes", &len))),
            _ => Ok(4_u32),
        },
        _ => Ok(1_u32),
    }
}};}

/// # Decodes a list from source
///
/// Returns: `IoResult<Option<Value>>`
#[cfg(feature="std")]
macro_rules! decode_list { ($source: ident) => {{
    let (size, bytes_of_size) = read_size_and_its_length($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(io::Error::new(ErrorKind::InvalidData, __!("invalid declared size: {}", &size)));
    }

    let (item_count, bytes_of_item_count) = read_size_and_its_length($source)?;

    let mut result = vec![];
    let mut read: Size = sum!(bytes_of_size, bytes_of_item_count)?;
    for item_index in 0..item_count {
        let value = match crate::decode($source)? {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("missing item #{}/{}", &item_index, &item_count))),
        };
        read = match read.checked_add(value.size()?) {
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
#[cfg(feature="std")]
macro_rules! decode_map { ($source: ident) => {{
    let (size, bytes_of_size) = read_size_and_its_length($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(io::Error::new(ErrorKind::InvalidData, __!("invalid declared size: {}", &size)));
    }

    let (item_count, bytes_of_item_count) = read_size_and_its_length($source)?;

    let mut result = Map::new();
    let mut read: Size = sum!(bytes_of_size, bytes_of_item_count)?;
    for _ in 0..item_count {
        let key = read_int_be!(i32, $source)?;
        let value = match crate::decode($source)? {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("missing value for key {}", &key))),
        };
        read = match read.checked_add(sum!(mem::size_of_val(&key) as Size, value.size()?)?) {
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
#[cfg(feature="std")]
macro_rules! decode_object { ($source: ident) => {{
    let (size, bytes_of_size) = read_size_and_its_length($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(io::Error::new(ErrorKind::InvalidData, __!("invalid declared size: {}", &size)));
    }

    let (item_count, bytes_of_item_count) = read_size_and_its_length($source)?;

    let mut result = Object::new();
    let mut read: Size = sum!(bytes_of_size, bytes_of_item_count)?;
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
        let value = match crate::decode($source)? {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::InvalidData, __!("missing value for key {:?}", &key))),
        };
        read = match read.checked_add(value.size()?) {
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

impl Value {

    /// # Calculates size of this value
    pub fn size(&self) -> Result<Size> {
        match self {
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
            Value::Text(t) => sum!(bytes_for_len!(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DateTime(dt) => sum!(bytes_for_len!(dt.len())?, 2, dt.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Date(d) => sum!(bytes_for_len!(d.len())?, 2, d.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Time(t) => sum!(bytes_for_len!(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DecimalStr(ds) => sum!(bytes_for_len!(ds.len())?, 2, ds.len()),
            // 1 byte for type
            Value::Blob(bytes) => sum!(bytes_for_len!(bytes.len())?, 1, bytes.len()),
            Value::List(list) => size_of_list(list),
            Value::Map(map) => size_of_map(map),
            Value::Object(object) => size_of_object(object),
        }
    }

    /// # Encodes this value into a buffer
    ///
    /// Returns the number of bytes written.
    #[cfg(feature="std")]
    pub fn encode(&self, buf: &mut dyn Write) -> IoResult<Size> {
        let expected_result = self.size()?;

        let result = match *self {
            Value::Null => buf.write_all(&[crate::value::NULL]).and(Ok(1))?,
            Value::True => buf.write_all(&[crate::value::TRUE]).and(Ok(1))?,
            Value::False => buf.write_all(&[crate::value::FALSE]).and(Ok(1))?,
            Value::U8(u) => buf.write_all(&[crate::value::U8, u]).and(Ok(2))?,
            Value::I8(i) => sum!(write_int_be!(crate::value::I8, buf)?, write_int_be!(i, buf)?)?,
            Value::U16(u) => sum!(write_int_be!(crate::value::U16, buf)?, write_int_be!(u, buf)?)?,
            Value::I16(i) => sum!(write_int_be!(crate::value::I16, buf)?, write_int_be!(i, buf)?)?,
            Value::U32(u) => sum!(write_int_be!(crate::value::U32, buf)?, write_int_be!(u, buf)?)?,
            Value::I32(i) => sum!(write_int_be!(crate::value::I32, buf)?, write_int_be!(i, buf)?)?,
            Value::Float(f) => sum!(write_int_be!(crate::value::FLOAT, buf)?, write_int_be!(f.to_bits(), buf)?)?,
            Value::U64(u) => sum!(write_int_be!(crate::value::U64, buf)?, write_int_be!(u, buf)?)?,
            Value::I64(i) => sum!(write_int_be!(crate::value::I64, buf)?, write_int_be!(i, buf)?)?,
            Value::Double(f) => sum!(write_int_be!(crate::value::DOUBLE, buf)?, write_int_be!(f.to_bits(), buf)?)?,
            Value::Text(ref t) => encode_value_str(crate::value::TEXT, t.as_str(), buf)?,
            Value::DateTime(ref dt) => encode_value_str(crate::value::DATE_TIME, dt.as_str(), buf)?,
            Value::Date(ref d) => encode_value_str(crate::value::DATE, d.as_str(), buf)?,
            Value::Time(ref t) => encode_value_str(crate::value::TIME, t.as_str(), buf)?,
            Value::DecimalStr(ref ds) => encode_value_str(crate::value::DECIMAL_STR, ds.as_str(), buf)?,
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

}

/// # Decodes a value from source
///
/// If `filter` is provided, the function expects that next value from source is one of them, and returns an error if not.
///
/// If `filter` is `None`, the function decodes any value from source.
#[cfg(feature="std")]
pub(crate) fn decode_value(filter: Option<&[u8]>, source: &mut dyn Read) -> IoResult<Option<Value>> {
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
        crate::value::NULL => Ok(Some(Value::Null)),
        crate::value::TRUE => Ok(Some(Value::True)),
        crate::value::FALSE => Ok(Some(Value::False)),
        crate::value::U8 => Ok(Some(Value::U8(read_int_be!(u8, source)?))),
        crate::value::I8 => Ok(Some(Value::I8(read_int_be!(i8, source)?))),
        crate::value::U16 => Ok(Some(Value::U16(read_int_be!(u16, source)?))),
        crate::value::I16 => Ok(Some(Value::I16(read_int_be!(i16, source)?))),
        crate::value::U32 => Ok(Some(Value::U32(read_int_be!(u32, source)?))),
        crate::value::I32 => Ok(Some(Value::I32(read_int_be!(i32, source)?))),
        crate::value::FLOAT => Ok(Some(Value::Float(f32::from_bits(read_int_be!(u32, source)?)))),
        crate::value::U64 => Ok(Some(Value::U64(read_int_be!(u64, source)?))),
        crate::value::I64 => Ok(Some(Value::I64(read_int_be!(i64, source)?))),
        crate::value::DOUBLE => Ok(Some(Value::Double(f64::from_bits(read_int_be!(u64, source)?)))),
        crate::value::TEXT => Ok(Some(Value::Text(read_str!(source)?))),
        crate::value::DATE_TIME => Ok(Some(Value::DateTime(read_str!(source)?))),
        crate::value::DATE => Ok(Some(Value::Date(read_str!(source)?))),
        crate::value::TIME => Ok(Some(Value::Time(read_str!(source)?))),
        crate::value::DECIMAL_STR => Ok(Some(Value::DecimalStr(read_str!(source)?))),
        crate::value::BLOB => Ok(Some(Value::Blob(read_into_new_vec!(read_size(source)?, source)?))),
        crate::value::LIST => decode_list!(source),
        crate::value::MAP => decode_map!(source),
        crate::value::OBJECT => decode_object!(source),
        _ => Err(io::Error::new(ErrorKind::InvalidData, __!("data type is either invalid or not supported: {}", &source_value))),
    }
}

/// # Calculates list size
fn size_of_list(list: &[Value]) -> Result<Size> {
    // Type + count
    let mut result: Size = sum!(bytes_for_len!(list.len())?, 1)?;
    // Items
    for v in list {
        result = sum!(result, v.size()?)?;
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
        false => Err(Error::from(__!("data too large: {} bytes", result))),
    }
}

/// # Calculates map size
fn size_of_map(map: &Map) -> Result<Size> {
    // Type + count
    let mut result = sum!(bytes_for_len!(map.len())?, 1)?;
    // Items
    for v in map.values() {
        result = sum!(result, mem::size_of::<i32>(), v.size()?)?;
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
        false => Err(Error::from(__!("data too large: {} bytes", result))),
    }
}

/// # Calculates object size
fn size_of_object(object: &Object) -> Result<Size> {
    // Type + count
    let mut result = sum!(bytes_for_len!(object.len())?, 1)?;
    // Items
    for (key, value) in object {
        // Key has NO null terminator
        let key_len = key.len();
        if key_len > OBJECT_KEY_MAX_LEN {
            return Err(Error::from(__!("key size is limited to {} bytes; got: {}", OBJECT_KEY_MAX_LEN, &key_len)));
        }
        result = sum!(result, key_len, value.size()?, 1)?;
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
        false => Err(Error::from(__!("data too large: {} bytes", result))),
    }
}

/// # Encodes a `Value`'s string into the buffer
#[cfg(feature="std")]
fn encode_value_str(ty: u8, s: &str, buf: &mut dyn Write) -> IoResult<Size> {
    let bytes = s.as_bytes();
    let str_len = {
        let tmp = bytes.len();
        match tmp.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => return Err(io::Error::new(ErrorKind::Other, __!("string too large ({} bytes)", &tmp))),
            _ => tmp as Size,
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
#[cfg(feature="std")]
fn encode_value_blob(bytes: &[u8], buf: &mut dyn Write) -> IoResult<Size> {
    let len = {
        let tmp = bytes.len();
        match tmp.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => return Err(io::Error::new(ErrorKind::Other, __!("too large: {} byte(s)", tmp))),
            _ => tmp as Size,
        }
    };

    // Type
    let mut bytes_written = match buf.write(&[crate::value::BLOB])? {
        1 => 1 as Size,
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
#[cfg(feature="std")]
fn encode_value_list(size: Size, list: &[Value], buf: &mut dyn Write) -> IoResult<Size> {
    let mut result = sum!(
        // Type
        write_int_be!(crate::value::LIST, buf)?,
        // Size
        write_size!(size, buf)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called size(), which verified the whole
        // container's size.
        write_size!(list.len() as Size, buf)?
    )?;

    // Items
    for v in list {
        result = sum!(result, v.encode(buf)?)?;
    }

    Ok(result)
}

/// # Encodes a `Value`'s map into the buffer
#[cfg(feature="std")]
fn encode_value_map(size: Size, map: &Map, buf: &mut dyn Write) -> IoResult<Size> {
    let mut result = sum!(
        // Type
        write_int_be!(crate::value::MAP, buf)?,
        // Size
        write_size!(size, buf)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called size(), which verified the whole
        // container's size.
        write_size!(map.len() as Size, buf)?
    )?;

    // Items
    for (key, value) in map {
        result = sum!(result, write_int_be!(key, buf)?, value.encode(buf)?)?;
    }

    Ok(result)
}

/// # Encodes a `Value`'s object into the buffer
///
/// ## Parameters
///
/// - `size`: should be calculated by `Value::size()`.
#[cfg(feature="std")]
fn encode_value_object(size: Size, object: &Object, buf: &mut dyn Write) -> IoResult<Size> {
    let mut result = sum!(
        // Type
        write_int_be!(crate::value::OBJECT, buf)?,
        // Size
        write_size!(size, buf)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called size(), which verified the whole
        // container's size.
        write_size!(object.len() as Size, buf)?
    )?;

    // Items
    for (key, value) in object {
        let key_len = key.len();
        result = match key_len <= OBJECT_KEY_MAX_LEN {
            true => sum!(result, write_int_be!(key_len as u8, buf)?)?,
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
