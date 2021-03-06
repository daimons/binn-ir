// License: see LICENSE file at root directory of `master` branch

//! # Value enum

use {
    alloc::string::String,
    core::{
        cmp::Ordering,
        fmt::{self, Debug, Formatter, Write as FmtWrite},
        mem,
    },

    crate::{
        Blob, List, Map, Object, Result, Size,
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

mod impls;

const MAX_I8_AS_USIZE: usize = i8::max_value() as usize;
const MAX_I8_AS_U32: Size = i8::max_value() as Size;

/// # Size mask
#[cfg(feature="std")]
const SIZE_MASK: Size = 0x_8000_0000;

/// # Values
///
/// ## Usage
///
/// ### Converting data into `Value`
///
/// This is straightforward. You can make this enum directly, or via implementations of [`From`][core::convert/From],
/// [`FromIterator`][core::iter/FromIterator]...
///
/// ### Extracting data from `Value`
///
/// There are several options:
///
/// - By using the good old `match`.
/// - Or implementations of [`TryFrom`][core::convert/TryFrom].
/// - Or via shortcut functions.
///
/// For numbers, if your have a strict specification of your data, it's better to use `match`. Otherwise, you can use
/// [`TryFrom`][core::convert/TryFrom] implementations.
///
/// [core::convert/From]: https://doc.rust-lang.org/core/convert/trait.From.html
/// [core::convert/TryFrom]: https://doc.rust-lang.org/core/convert/trait.TryFrom.html
/// [core::iter/FromIterator]: https://doc.rust-lang.org/core/iter/trait.FromIterator.html
#[derive(Clone, PartialEq)]
pub enum Value {

    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`NULL`][value::NULL]
    ///
    /// [_Shortcuts_](#shortcuts-for-null)
    ///
    /// [storage::NO_BYTES]: storage/constant.NO_BYTES.html
    /// [value::NULL]: value/constant.NULL.html
    Null,

    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`TRUE`][value::TRUE]
    ///
    /// [_Shortcuts_](#shortcuts-for-booleans)
    ///
    /// [storage::NO_BYTES]: storage/constant.NO_BYTES.html
    /// [value::TRUE]: value/constant.TRUE.html
    True,

    /// - Storage: [`NO_BYTES`][storage::NO_BYTES]
    /// - Type: [`FALSE`][value::FALSE]
    ///
    /// [_Shortcuts_](#shortcuts-for-booleans)
    ///
    /// [storage::NO_BYTES]: storage/constant.NO_BYTES.html
    /// [value::FALSE]: value/constant.FALSE.html
    False,

    /// - Storage: [`BYTE`][storage::BYTE]
    /// - Type: [`U8`][value::U8]
    ///
    /// [storage::BYTE]: storage/constant.BYTE.html
    /// [value::U8]: value/constant.U8.html
    U8(u8),

    /// - Storage: [`BYTE`][storage::BYTE]
    /// - Type: [`I8`][value::I8]
    ///
    /// [storage::BYTE]: storage/constant.BYTE.html
    /// [value::I8]: value/constant.I8.html
    I8(i8),

    /// - Storage: [`WORD`][storage::WORD]
    /// - Type: [`U16`][value::U16]
    ///
    /// [storage::WORD]: storage/constant.WORD.html
    /// [value::U16]: value/constant.U16.html
    U16(u16),

    /// - Storage: [`WORD`][storage::WORD]
    /// - Type: [`I16`][value::I16]
    ///
    /// [storage::WORD]: storage/constant.WORD.html
    /// [value::I16]: value/constant.I16.html
    I16(i16),

    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`U32`][value::U32]
    ///
    /// [storage::DWORD]: storage/constant.DWORD.html
    /// [value::U32]: value/constant.U32.html
    U32(u32),

    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`I32`][value::I32]
    ///
    /// [storage::DWORD]: storage/constant.DWORD.html
    /// [value::I32]: value/constant.I32.html
    I32(i32),

    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`U64`][value::U64]
    ///
    /// [storage::QWORD]: storage/constant.QWORD.html
    /// [value::U64]: value/constant.U64.html
    U64(u64),

    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`I64`][value::I64]
    ///
    /// [storage::QWORD]: storage/constant.QWORD.html
    /// [value::I64]: value/constant.I64.html
    I64(i64),

    /// - Storage: [`DWORD`][storage::DWORD]
    /// - Type: [`FLOAT`][value::FLOAT]
    ///
    /// [storage::DWORD]: storage/constant.DWORD.html
    /// [value::FLOAT]: value/constant.FLOAT.html
    Float(f32),

    /// - Storage: [`QWORD`][storage::QWORD]
    /// - Type: [`DOUBLE`][value::DOUBLE]
    ///
    /// [storage::QWORD]: storage/constant.QWORD.html
    /// [value::DOUBLE]: value/constant.DOUBLE.html
    Double(f64),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`TEXT`][value::TEXT]
    ///
    /// [_Shortcuts_](#shortcuts-for-strings)
    ///
    /// [storage::STRING]: storage/constant.STRING.html
    /// [value::TEXT]: value/constant.TEXT.html
    Text(String),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DATE_TIME`][value::DATE_TIME]
    ///
    /// [_Shortcuts_](#shortcuts-for-strings)
    ///
    /// [storage::STRING]: storage/constant.STRING.html
    /// [value::DATE_TIME]: value/constant.DATE_TIME.html
    DateTime(String),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DATE`][value::DATE]
    ///
    /// [_Shortcuts_](#shortcuts-for-strings)
    ///
    /// [storage::STRING]: storage/constant.STRING.html
    /// [value::DATE]: value/constant.DATE.html
    Date(String),

    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`TIME`][value::TIME]
    ///
    /// [_Shortcuts_](#shortcuts-for-strings)
    ///
    /// [storage::STRING]: storage/constant.STRING.html
    /// [value::TIME]: value/constant.TIME.html
    Time(String),

    /// <small>_(Decimal string)_</small>
    ///
    /// - Storage: [`STRING`][storage::STRING]
    /// - Type: [`DECIMAL_STR`][value::DECIMAL_STR]
    ///
    /// [_Shortcuts_](#shortcuts-for-strings)
    ///
    /// [storage::STRING]: storage/constant.STRING.html
    /// [value::DECIMAL_STR]: value/constant.DECIMAL_STR.html
    DecimalStr(String),

    /// - Storage: [`BLOB`][storage::BLOB]
    /// - Type: [`BLOB`][value::BLOB]
    ///
    /// [_Shortcuts_](#shortcuts-for-blob)
    ///
    /// [storage::BLOB]: storage/constant.BLOB.html
    /// [value::BLOB]: value/constant.BLOB.html
    Blob(Blob),

    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`LIST`][value::LIST]
    ///
    /// [_Shortcuts_](#shortcuts-for-list)
    ///
    /// [storage::CONTAINER]: storage/constant.CONTAINER.html
    /// [value::LIST]: value/constant.LIST.html
    List(List),

    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`MAP`][value::MAP]
    ///
    /// [_Shortcuts_](#shortcuts-for-map)
    ///
    /// [storage::CONTAINER]: storage/constant.CONTAINER.html
    /// [value::MAP]: value/constant.MAP.html
    Map(Map),

    /// - Storage: [`CONTAINER`][storage::CONTAINER]
    /// - Type: [`OBJECT`][value::OBJECT]
    ///
    /// [_Shortcuts_](#shortcuts-for-object)
    ///
    /// ## Notes
    ///
    /// - Key lengths must be `<=` [`OBJECT_KEY_MAX_LEN`][value::OBJECT_KEY_MAX_LEN].
    ///
    /// [storage::CONTAINER]: storage/constant.CONTAINER.html
    /// [value::OBJECT]: value/constant.OBJECT.html
    /// [value::OBJECT_KEY_MAX_LEN]: value/constant.OBJECT_KEY_MAX_LEN.html
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

impl<T> From<Option<T>> for Value where T: Into<Value> {

    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }

}

/// # Converts an integer value to big-endian order and writes it into the stream
///
/// Returns: number of bytes written, as `IoResult<Size>`.
#[cfg(feature="std")]
macro_rules! write_int_be { ($v: expr, $stream: ident) => {{
    let bytes = $v.to_be_bytes();
    $stream.write_all(&bytes).map(|()| bytes.len() as Size)
}};}

/// # Reads an integer value in big-endian format from std::io::Read
///
/// Result: `IoResult<$ty>`.
#[cfg(feature="std")]
macro_rules! read_int_be { ($ty: ty, $source: ident) => {{
    let mut buf = [0_u8; mem::size_of::<$ty>()];
    $source.read_exact(&mut buf).map(|()| <$ty>::from_be_bytes(buf))
}};}

/// # Writes size (u32) into the stream
///
/// Result: number of bytes written - `IoResult<Size>`.
#[cfg(feature="std")]
macro_rules! write_size { ($size: expr, $stream: ident) => {{
    let size = $size;
    match size > MAX_I8_AS_U32 {
        true => write_int_be!(size | SIZE_MASK, $stream),
        false => write_int_be!(size as u8, $stream),
    }
}};}

/// # Reads size from source
///
/// Result:
///
/// - First value is size.
/// - Second value is total bytes read (the 'length' of first value).
#[cfg(feature="std")]
fn read_size_and_its_length<R>(source: &mut R) -> IoResult<(Size, Size)> where R: Read {
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
fn read_size<R>(source: &mut R) -> IoResult<Size> where R: Read {
    read_size_and_its_length(source).and_then(|(size, _)| Ok(size))
}

#[test]
#[cfg(feature="std")]
fn test_read_size_and_its_length() {
    use ::std::io::Cursor;

    const U32_SIZE: Size = mem::size_of::<Size>() as Size;
    const MAX_U8: u8 = ::std::u8::MAX;

    assert_eq!(read_size_and_its_length(&mut Cursor::new(alloc::vec![MAX_U8, MAX_U8, MAX_U8, MAX_U8])).unwrap(), (MAX_DATA_SIZE, U32_SIZE));

    for bytes in alloc::vec![
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
                        Ordering::Greater => Err(err!("too large for: {} + {} (max allowed: {})", &current, &b, MAX_DATA_SIZE)),
                        _ => match current.checked_add(b as Size) {
                            Some(new) => match new.cmp_to(&MAX_DATA_SIZE) {
                                Ordering::Greater => Err(err!("too large for: {} + {} (max allowed: {})", &current, &b, MAX_DATA_SIZE)),
                                _ => Ok(new),
                            },
                            None => Err(err!("can't add {} into {}", &b, &current)),
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
        Ordering::Greater => Err(err!("cannot allocate a vector with capacity: {} (max allowed: {})", &capacity, MAX_DATA_SIZE)),
        _ => match capacity.cmp_to(&usize::max_value()) {
            Ordering::Greater => Err(err!("cannot allocate a vector with capacity: {} (max allowed: {})", &capacity, ::std::usize::MAX)),
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
            Ordering::Greater => Err(err!("too large: {} bytes", &len)),
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

    let mut result = alloc::vec![];
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

    /// # Encodes this value into a stream
    ///
    /// Returns the number of bytes written.
    #[cfg(feature="std")]
    pub fn encode<W>(&self, stream: &mut W) -> IoResult<Size> where W: Write {
        match self {
            Value::Null => stream.write_all(&[crate::value::NULL]).map(|()| 1),
            Value::True => stream.write_all(&[crate::value::TRUE]).map(|()| 1),
            Value::False => stream.write_all(&[crate::value::FALSE]).map(|()| 1),
            Value::U8(u) => stream.write_all(&[crate::value::U8, *u]).map(|()| 2),
            Value::I8(i) => Ok(write_int_be!(crate::value::I8, stream)? + write_int_be!(i, stream)?),
            Value::U16(u) => Ok(write_int_be!(crate::value::U16, stream)? + write_int_be!(u, stream)?),
            Value::I16(i) => Ok(write_int_be!(crate::value::I16, stream)? + write_int_be!(i, stream)?),
            Value::U32(u) => Ok(write_int_be!(crate::value::U32, stream)? + write_int_be!(u, stream)?),
            Value::I32(i) => Ok(write_int_be!(crate::value::I32, stream)? + write_int_be!(i, stream)?),
            Value::U64(u) => Ok(write_int_be!(crate::value::U64, stream)? + write_int_be!(u, stream)?),
            Value::I64(i) => Ok(write_int_be!(crate::value::I64, stream)? + write_int_be!(i, stream)?),
            Value::Float(f) => Ok(write_int_be!(crate::value::FLOAT, stream)? + write_int_be!(f.to_bits(), stream)?),
            Value::Double(f) => Ok(write_int_be!(crate::value::DOUBLE, stream)? + write_int_be!(f.to_bits(), stream)?),
            Value::Text(t) => encode_value_str(crate::value::TEXT, t.as_str(), stream),
            Value::DateTime(dt) => encode_value_str(crate::value::DATE_TIME, dt.as_str(), stream),
            Value::Date(d) => encode_value_str(crate::value::DATE, d.as_str(), stream),
            Value::Time(t) => encode_value_str(crate::value::TIME, t.as_str(), stream),
            Value::DecimalStr(ds) => encode_value_str(crate::value::DECIMAL_STR, ds.as_str(), stream),
            Value::Blob(bytes) => encode_value_blob(bytes.as_slice(), stream),
            Value::List(list) => encode_value_list(self.size()?, list, stream),
            Value::Map(map) => encode_value_map(self.size()?, map, stream),
            Value::Object(object) => encode_value_object(self.size()?, object, stream),
        }
    }

}

/// # Decodes a value from source
///
/// If `filter` is provided, the function expects that next value from source is one of them, and returns an error if not.
///
/// If `filter` is `None`, the function decodes any value from source.
#[cfg(feature="std")]
pub(crate) fn decode_value<R>(filter: Option<&[u8]>, source: &mut R) -> IoResult<Option<Value>> where R: Read {
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
        false => Err(err!("data too large: {} bytes", result)),
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
        false => Err(err!("data too large: {} bytes", result)),
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
            return Err(err!("key size is limited to {} bytes; got: {}", OBJECT_KEY_MAX_LEN, &key_len));
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
        false => Err(err!("data too large: {} bytes", result)),
    }
}

/// # Encodes a `Value`'s string into the stream
#[cfg(feature="std")]
fn encode_value_str<W>(ty: u8, s: &str, stream: &mut W) -> IoResult<Size> where W: Write {
    let bytes = s.as_bytes();
    let str_len = {
        let tmp = bytes.len();
        match tmp.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => return Err(io::Error::from(err!("string too large ({} bytes)", &tmp))),
            _ => tmp as Size,
        }
    };

    let total_size = sum!(
        str_len,
        // 1 for type, 1 for null terminator
        2 + match str_len > MAX_I8_AS_U32 { true => 4, false => 1 }
    )?;

    // Type
    match stream.write(&[ty])? {
        1 => (),
        other => return Err(io::Error::from(err!("expected to write 1 byte; result: {}", &other))),
    };

    // Size
    // Note that null terminator does NOT count
    write_size!(str_len, stream)?;

    // Data
    let written = stream.write(bytes)?;
    match written.cmp_to(&str_len) {
        Ordering::Equal => (),
        _ => return Err(io::Error::from(err!("expected to write {} byte(s); result: {}", str_len, written))),
    };

    // Null terminator
    match stream.write(&[0])? {
        1 => (),
        other => return Err(io::Error::from(err!("expected to write 1 byte; result: {}", &other))),
    };

    Ok(total_size)
}

/// # Encodes `Value`'s blob into the stream
#[cfg(feature="std")]
fn encode_value_blob<W>(bytes: &[u8], stream: &mut W) -> IoResult<Size> where W: Write {
    let len = {
        let tmp = bytes.len();
        match tmp.cmp_to(&MAX_DATA_SIZE) {
            Ordering::Greater => return Err(io::Error::from(err!("too large: {} byte(s)", tmp))),
            _ => tmp as Size,
        }
    };

    // Type
    let mut bytes_written = match stream.write(&[crate::value::BLOB])? {
        1 => 1 as Size,
        other => return Err(io::Error::from(err!("expected to write 1 byte; result: {}", &other))),
    };

    // Size
    bytes_written = sum!(write_size!(len, stream)?, bytes_written)?;

    // Data
    let written = stream.write(bytes)?;
    match written.cmp_to(&len) {
        Ordering::Equal => (),
        _ => return Err(io::Error::from(err!("expected to write {} byte(s); result: {}", &len, &written))),
    };
    bytes_written = sum!(bytes_written, written)?;

    Ok(bytes_written)
}

/// # Encodes a `Value`'s list into the stream
#[cfg(feature="std")]
fn encode_value_list<W>(size: Size, list: &[Value], stream: &mut W) -> IoResult<Size> where W: Write {
    let mut result = sum!(
        // Type
        write_int_be!(crate::value::LIST, stream)?,
        // Size
        write_size!(size, stream)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called size(), which verified the whole
        // container's size.
        write_size!(list.len() as Size, stream)?
    )?;

    // Items
    for v in list {
        result = sum!(result, v.encode(stream)?)?;
    }

    Ok(result)
}

/// # Encodes a `Value`'s map into the stream
#[cfg(feature="std")]
fn encode_value_map<W>(size: Size, map: &Map, stream: &mut W) -> IoResult<Size> where W: Write {
    let mut result = sum!(
        // Type
        write_int_be!(crate::value::MAP, stream)?,
        // Size
        write_size!(size, stream)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called size(), which verified the whole
        // container's size.
        write_size!(map.len() as Size, stream)?
    )?;

    // Items
    for (key, value) in map {
        result = sum!(result, write_int_be!(key, stream)?, value.encode(stream)?)?;
    }

    Ok(result)
}

/// # Encodes a `Value`'s object into the stream
///
/// ## Parameters
///
/// - `size`: should be calculated by `Value::size()`.
#[cfg(feature="std")]
fn encode_value_object<W>(size: Size, object: &Object, stream: &mut W) -> IoResult<Size> where W: Write {
    let mut result = sum!(
        // Type
        write_int_be!(crate::value::OBJECT, stream)?,
        // Size
        write_size!(size, stream)?,
        // Count
        // We don't have to verify this value. Since at the beginning of Value::encode(), we already called size(), which verified the whole
        // container's size.
        write_size!(object.len() as Size, stream)?
    )?;

    // Items
    for (key, value) in object {
        let key_len = key.len();
        result = match key_len <= OBJECT_KEY_MAX_LEN {
            true => sum!(result, write_int_be!(key_len as u8, stream)?)?,
            false => return Err(io::Error::new(
                ErrorKind::InvalidData, __!("key length is limited to {} bytes, got: {}", OBJECT_KEY_MAX_LEN, &key_len)
            )),
        };

        let written = stream.write(key.as_bytes())?;
        match written.cmp_to(&key_len) {
            Ordering::Equal => result = sum!(result, written)?,
            _ => return Err(io::Error::from(err!("expected to write {} byte(s) of key; result: {}", &key_len, &written))),
        }

        result = sum!(result, value.encode(stream)?)?;
    }

    Ok(result)
}
