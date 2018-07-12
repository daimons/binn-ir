// License: see LICENSE file at root directory of `master` branch

//! # Values

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::io::{self, Error, ErrorKind, Read, Write};
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

/// # Size mask
const SIZE_MASK: u32 = 0x_8000_0000_u32;

/// # Object key's max length
const OBJECT_KEY_MAX_LEN: usize = 255;

#[test]
fn test_object_key_max_len() {
    assert_eq!(cmp_integers!(OBJECT_KEY_MAX_LEN, ::std::u8::MAX), Ordering::Equal);
}

/// # Values
#[derive(PartialEq)]
pub enum Value {

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
    Text(String),

    /// # Date time
    DateTime(String),

    /// # Date
    Date(String),

    /// # Time
    Time(String),

    /// # Decimal string
    DecimalStr(String),

    /// # Blob
    Blob(Vec<u8>),

    /// # List
    List(Vec<Value>),

    /// # Map
    Map(BTreeMap<i32, Value>),

    /// # Object
    Object(HashMap<String, Value>),

}

/// # Makes plural suffix
macro_rules! make_plural_suffix {
    ($size: expr, $one: expr, $other: expr) => {{
        match $size {
            1 => $one,
            _ => $other,
        }
    }};
    // This one makes 's' plural suffix
    ($size: expr) => {{
        make_plural_suffix!($size, "", "s")
    }};
}

/// # Displays a string in a formatter
///
/// If the string is too long, just displays a part of it.
///
/// Result: `Result<(), fmt::Error>`
macro_rules! display_str { ($formatter: ident, $prefix: expr, $s: ident, $suffix: expr) => {{
    write!($formatter, "{}", $prefix)?;

    let char_count = &$s.chars().count();
    let limit = 100;
    match char_count.checked_sub(limit) {
        Some(more) => write!($formatter, "\"{}...\" ({} more)", &$s.chars().take(limit).collect::<String>(), more)?,
        None => write!($formatter, "\"{}\"", &$s)?,
    };

    write!($formatter, "{}", $suffix)
}};}

/// # Max display items of list/map/object
const LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS: usize = 10;

/// # Displays a list in a formatter
///
/// If the items are too much, just displays a part of them.
///
/// Result: `Result<(), fmt::Error>`
macro_rules! display_list { ($formatter: ident, $value: ident, $list: ident) => {{
    let item_count = $list.len();

    match $value.len() {
        Ok(len) => {
            write!($formatter, "List({} item{}, {} byte{}: [", &item_count, make_plural_suffix!(&item_count), &len, make_plural_suffix!(&len))?;
            for (index, item) in $list.iter().enumerate() {
                if index > 0 {
                    if index >= LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS { break; }
                    write!($formatter, ", ")?;
                }
                write!($formatter, "{}", &item)?;
            }
            match item_count.checked_sub(LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS) {
                Some(more) if more > 0 => write!($formatter, ",... {} more]", &more),
                _ => write!($formatter, "]"),
            }
        },
        Err(err) => write!($formatter, "List({} item{}, unknown size ({}))", &item_count, make_plural_suffix!(&item_count), &err),
    }
}};}

/// # Displays a map in a formatter
///
/// If the items are too much, just displays a part of them.
///
/// Result: `Result<(), fmt::Error>`
macro_rules! display_map { ($formatter: ident, $value: ident, $map: ident) => {{
    let item_count = $map.len();

    match $value.len() {
        Ok(len) => {
            write!($formatter, "Map({} item{}, {} byte{}: {{", &item_count, make_plural_suffix!(&item_count), &len, make_plural_suffix!(&len))?;
            for (index, (key, value)) in $map.iter().enumerate() {
                if index > 0 {
                    if index >= LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS { break; }
                    write!($formatter, ", ")?;
                }
                write!($formatter, "{}: {}", &key, &value)?;
            }
            match item_count.checked_sub(LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS) {
                Some(more) if more > 0 => write!($formatter, ",... {} more}}", &more),
                _ => write!($formatter, "}}"),
            }
        },
        Err(err) => write!($formatter, "Map({} item{}, unknown size ({}))", &item_count, make_plural_suffix!(&item_count), &err),
    }
}};}

/// # Displays an object in a formatter
///
/// If the items are too much, just displays a part of them.
///
/// Result: `Result<(), fmt::Error>`
macro_rules! display_object { ($formatter: ident, $value: ident, $object: ident) => {{
    let item_count = $object.len();

    match $value.len() {
        Ok(len) => {
            write!(
                $formatter, "Object({} item{}, {} byte{}: {{", &item_count, make_plural_suffix!(&item_count), &len, make_plural_suffix!(&len)
            )?;
            for (index, (key, value)) in $object.iter().enumerate() {
                if index > 0 {
                    if index >= LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS { break; }
                    write!($formatter, ", ")?;
                }
                write!($formatter, "{:?}: {}", &key, &value)?;
            }
            match item_count.checked_sub(LIST_MAP_OBJECT_MAX_DISPLAY_ITEMS) {
                Some(more) if more > 0 => write!($formatter, ",... {} more}}", &more),
                _ => write!($formatter, "}}"),
            }
        },
        Err(err) => write!($formatter, "Object({} item{}, unknown size ({}))", &item_count, make_plural_suffix!(&item_count), &err),
    }
}};}

impl fmt::Display for Value {

    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Value::Null => write!(formatter, "Null"),
            Value::True => write!(formatter, "True"),
            Value::False => write!(formatter, "False"),
            Value::U8(ref u) => write!(formatter, "U8({})", &u),
            Value::I8(ref i) => write!(formatter, "I8({})", &i),
            Value::U16(ref u) => write!(formatter, "U16({})", &u),
            Value::I16(ref i) => write!(formatter, "I16({})", &i),
            Value::U32(ref u) => write!(formatter, "U32({})", &u),
            Value::I32(ref i) => write!(formatter, "I32({})", &i),
            Value::Float(ref f) => write!(formatter, "Float({})", &f),
            Value::U64(ref u) => write!(formatter, "U64({})", &u),
            Value::I64(ref i) => write!(formatter, "I64({})", &i),
            Value::Double(ref d) => write!(formatter, "Double({})", &d),
            Value::Text(ref s) => display_str!(formatter, "Text(", s, ')'),
            Value::DateTime(ref dt) => display_str!(formatter, "DateTime(", dt, ')'),
            Value::Date(ref d) => display_str!(formatter, "Date(", d, ')'),
            Value::Time(ref t) => display_str!(formatter, "Time(", t, ')'),
            Value::DecimalStr(ref ds) => display_str!(formatter, "DecimalStr(", ds, ')'),
            Value::Blob(ref blob) => {
                let len = blob.len();
                write!(formatter, "Blob({} byte{})", &len, make_plural_suffix!(&len))
            },
            Value::List(ref list) => display_list!(formatter, self, list),
            Value::Map(ref map) => display_map!(formatter, self, map),
            Value::Object(ref object) => display_object!(formatter, self, object),
        }
    }

}

impl fmt::Debug for Value {

    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "\"")?;
        fmt::Display::fmt(self, formatter)?;
        write!(formatter, "\"")
    }

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

    assert_eq!(as_bytes!(u16, 0x_03E8_u16.to_be()), [0x03, 0xE8]);
    assert_eq!(as_bytes!(u16, 0x_270F_u16.to_be()), [0x27, 0x0F]);

    assert_eq!(as_bytes!(i16, (-9999 as i16).to_be()), [0xD8, 0xF1]);
    assert_eq!(as_bytes!(i16, (-2000 as i16).to_be()), [0xF8, 0x30]);
    assert_eq!(as_bytes!(i16, (-1234 as i16).to_be()), [0xFB, 0x2E]);

    assert_eq!(as_bytes!(u32, 0x_075B_CD15_u32.to_be()), [0x07, 0x5B, 0xCD, 0x15]);
    assert_eq!(as_bytes!(u32, 0x_3ADE_68B1_u32.to_be()), [0x3A, 0xDE, 0x68, 0xB1]);
    assert_eq!(as_bytes!(u32, 0x_AABB_FFEE_u32.to_be()), [0xAA, 0xBB, 0xFF, 0xEE]);

    assert_eq!(as_bytes!(u64, 0x_8000_0000_0000_0000_u64.to_be()), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(as_bytes!(u64, 0x_FFFF_FFFF_0000_0000_u64.to_be()), [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]);

    assert_eq!(as_bytes!(i64, (-3372036854775808 as i64).to_be()), [0xFF, 0xF4, 0x05, 0x26, 0x7D, 0x1A, 0x00, 0x00]);

    assert_eq!(
        as_bytes!(u128, 0x_AAFF_0000_CCDD_8899_1234_8678_BCCB_0000_u128.to_be()),
        [0xAA, 0xFF, 0x00, 0x00, 0xCC, 0xDD, 0x88, 0x99, 0x12, 0x34, 0x86, 0x78, 0xBC, 0xCB, 0x00, 0x00]
    );
}

/// # Data size
pub type DataSize = u32;

/// # Converts an integer value to big-endian order and writes it into the buffer
///
/// Returns: number of bytes written, as `io::Result<DataSize>`.
macro_rules! write_int_be { ($ty: ty, $v: expr, $buf: ident) => {{
    let bytes = as_bytes!($ty, $v.to_be());
    match $buf.write(&bytes) {
        Ok(count) => match count == bytes.len() {
            true => Ok(count as DataSize),
            false => Err(Error::new(
                ErrorKind::Other, format!("value::write_int_be!() -> expected to write {} byte(s); result: {}", bytes.len(), count)
            )),
        },
        Err(err) => Err(err),
    }
}};}

/// # Reads an integer value in big-endian format from std::io::Read
///
/// Result: `io::Result<$ty>`.
macro_rules! read_int_be { ($ty: ty, $source: ident) => {{
    let mut buf = [0_u8; mem::size_of::<$ty>()];
    match $source.read_exact(&mut buf) {
        Ok(()) => Ok(<$ty>::from_be(unsafe { mem::transmute(buf) })),
        Err(err) => Err(err),
    }
}};}

/// # Writes size (u32) into the buffer
///
/// Result: number of bytes written - `io::Result<DataSize>`.
macro_rules! write_size { ($size: expr, $buf: ident) => {{
    let size = $size;
    match cmp_integers!(size, ::std::i8::MAX) {
        Ordering::Greater => write_int_be!(u32, size | SIZE_MASK, $buf),
        _ => write_int_be!(u8, size as u8, $buf),
    }
}};}

/// # Reads size from std::io::Read
///
/// Result: `io::Result<u32>`.
macro_rules! read_size { ($source: ident) => {{
    let first_byte = read_int_be!(u8, $source)?;
    match first_byte & 0b_1000_0000 {
        0b_1000_0000 => {
            let mut buf = [first_byte, 0, 0, 0];
            match $source.read_exact(&mut buf[1..]) {
                Ok(()) => {
                    let result = u32::from_be(unsafe { mem::transmute(buf) }) & !(SIZE_MASK);
                    match cmp_integers!(result, Value::MAX_DATA_SIZE) {
                        Ordering::Greater => Err(Error::new(ErrorKind::InvalidData, format!("value::read_size!() -> too large: {}", &result))),
                        _ => Ok(result),
                    }
                },
                Err(err) => Err(err),
            }
        },
        _ => Ok(first_byte as u32),
    }
}};}

/// # Calculates sum of first value (`DataSize`) with integer(s)
///
/// Result: `io::Result<DataSize>`.
///
/// If result > [`Value::MAX_DATA_SIZE`], an error is returned.
///
/// [`Value::MAX_DATA_SIZE`]: enum.Value.html#associatedconstant.MAX_DATA_SIZE
macro_rules! sum {
    ($a: expr, $($b: expr),+) => {{
        // Do NOT nest multiple calls to cmp_integers(...); or the compiler will hang up!!!
        let mut result: io::Result<DataSize> = Ok($a);
        $(
            match result {
                Ok(current) => result = {
                    let b = $b;
                    match cmp_integers!(b, Value::MAX_DATA_SIZE) {
                        Ordering::Greater => Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("sum!() -> too large for: {} + {} (max allowed: {})", &current, &b, Value::MAX_DATA_SIZE)
                        )),
                        _ => match current.checked_add(b as DataSize) {
                            Some(new) => match cmp_integers!(new, Value::MAX_DATA_SIZE) {
                                Ordering::Greater => Err(Error::new(
                                    ErrorKind::InvalidData,
                                    format!("sum!() -> too large for: {} + {} (max allowed: {})", &current, &b, Value::MAX_DATA_SIZE)
                                )),
                                _ => Ok(new),
                            },
                            None => Err(Error::new(ErrorKind::InvalidData, format!("sum!() -> can't add {} into {}", &b, &current))),
                        },
                    }
                },
                Err(_) => (),
            };
        )+

        result
    }};
}

/// # Makes new vector with capacity
///
/// Returns: `io::Result<Vec<_>>`
macro_rules! new_vec_with_capacity { ($capacity: expr) => {{
    let capacity = $capacity;
    match cmp_integers!(capacity, Value::MAX_DATA_SIZE) {
        Ordering::Greater => Err(Error::new(
            ErrorKind::WriteZero,
            format!(
                "value::new_vec_with_capacity!() -> cannot allocate a vector with capacity: {} (max allowed: {})",
                &capacity, Value::MAX_DATA_SIZE
            )
        )),
        _ => match cmp_integers!(capacity, ::std::usize::MAX) {
            Ordering::Greater => Err(Error::new(
                ErrorKind::WriteZero,
                format!(
                    "value::new_vec_with_capacity!() -> cannot allocate a vector with capacity: {} (max allowed: {})",
                    &capacity, ::std::usize::MAX
                )
            )),
            _ => Ok(Vec::with_capacity(capacity as usize)),
        },
    }
}};}

/// # Reads data into new vector
///
/// Returns: `io::Result<Vec<_>>`
macro_rules! read_into_new_vec { ($len: expr, $source: ident) => {{
    let len = $len;
    let mut result = new_vec_with_capacity!(len)?;

    // Notes:
    //
    // - `len` was verified via above call to `new_vec_with_capacity!()`, that it must be <= `Value::MAX_DATA_SIZE`
    // - `Value::MAX_DATA_SIZE` should be **tested** to be < `std::u64::MAX`
    match $source.take(len as u64).read_to_end(&mut result) {
        Ok(read) => match read == result.len() {
            true => Ok(result),
            false => Err(Error::new(
                ErrorKind::WriteZero, format!("value::read_into_new_vec!() -> expected to read {} bytes, but: {}", &len, &read)
            )),
        },
        Err(err) => Err(Error::new(ErrorKind::WriteZero, format!("value::read_into_new_vec!() -> failed to read {} bytes: {}", &len, &err))),
    }
}};}

/// # Reads a string from source
///
/// Returns: `io::Result<String>`
macro_rules! read_str { ($source: ident) => {{
    // Note that null terminator does NOT count
    let buf = read_into_new_vec!(read_size!($source)?, $source)?;
    match read_int_be!(u8, $source)? {
        0 => String::from_utf8(buf).map_err(|err|
            Error::new(ErrorKind::InvalidData, format!("value::read_str!() -> failed to decode UTF-8: {}", &err))
        ),
        other => Err(Error::new(
            ErrorKind::InvalidData, format!("value::read_str!() -> expected to read a null terminator ('\\0'), got: {}", &other)
        )),
    }
}};}

/// # Reads a list from source
///
/// Returns: `io::Result<Value>`
macro_rules! read_list { ($source: ident) => {{
    let size = read_size!($source)?;
    let item_count = read_size!($source)?;

    let mut result = vec![];
    let mut read: DataSize = 0;
    for _ in 0..item_count {
        let value = Self::read($source)?;
        read = match read.checked_add(value.len()?) {
            Some(v) => match cmp_integers!(size, v) {
                Ordering::Greater => v,
                _ => return Err(Error::new(
                    ErrorKind::InvalidData, format!("value::read_list!() -> expected to read less than {} bytes, got: {}", &size, &v)
                )),
            },
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("value::read_list!() -> invalid list size -> expected: {}, current: {}, new item: {:?}", &size, &read, &value)
            )),
        };
        result.push(value);
    }

    Ok(Value::List(result))
}};}

/// # Reads a map from source
///
/// Returns: `io::Result<Value>`
macro_rules! read_map { ($source: ident) => {{
    let size = read_size!($source)?;
    let item_count = read_size!($source)?;

    let mut result = BTreeMap::new();
    let mut read: DataSize = 0;
    for _ in 0..item_count {
        let key = read_int_be!(i32, $source)?;
        let value = Self::read($source)?;
        read = match read.checked_add(value.len()?) {
            Some(v) => match cmp_integers!(size, v) {
                Ordering::Greater => v,
                _ => return Err(Error::new(
                    ErrorKind::InvalidData, format!("value::read_map!() -> expected to read less than {} bytes, got: {}", &size, &v)
                )),
            },
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "value::read_map!() -> invalid map size -> expected: {}, current: {}, new item: {} -> {:?}",
                    &size, &read, &key, &value,
                )
            )),
        };
        result.insert(key, value);
    }

    Ok(Value::Map(result))
}};}

/// # Reads an object from source
///
/// Returns: `io::Result<Value>`
macro_rules! read_object { ($source: ident) => {{
    let size = read_size!($source)?;
    let item_count = read_size!($source)?;

    let mut result = HashMap::new();
    let mut read: DataSize = 0;
    for _ in 0..item_count {
        // Read key (note that there's NO null terminator)
        let key_len = read_size!($source)?;
        match cmp_integers!(key_len, OBJECT_KEY_MAX_LEN) {
            Ordering::Greater => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("value::read_object!() -> key length is limited to {} bytes, got: {}", OBJECT_KEY_MAX_LEN, key_len)
            )),
            _ => read = match read.checked_add(key_len) {
                Some(v) => match cmp_integers!(size, v) {
                    Ordering::Greater => v,
                    _ => return Err(Error::new(
                        ErrorKind::InvalidData, format!("value::read_object!() -> expected to read less than {} bytes, got: {}",
                        &size, &v)
                    )),
                },
                None => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "value::read_object!() -> invalid object size -> expected: {}, current: {}, new key length: {}",
                        &size, &read, &key_len,
                    )
                )),
            },
        };
        let key = String::from_utf8(read_into_new_vec!(key_len, $source)?).map_err(|err|
            Error::new(ErrorKind::InvalidData, format!("value::read_object!() -> failed to decode UTF-8: {}", &err))
        )?;

        // Read value
        let value = Self::read($source)?;
        read = match read.checked_add(value.len()?) {
            Some(v) => match cmp_integers!(size, v) {
                Ordering::Greater => v,
                _ => return Err(Error::new(
                    ErrorKind::InvalidData, format!("value::read_object!() -> expected to read less than {} bytes, got: {}", &size, &v)
                )),
            },
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("value::read_object!() -> invalid object size -> expected: {}, current: {}, new value: {:?}",
                &size, &read, &value)
            )),
        };
        result.insert(key, value);
    }

    Ok(Value::Object(result))
}};}

impl Value {

    /// # Max data size, in bytes
    pub const MAX_DATA_SIZE: DataSize = ::std::i32::MAX as DataSize;

    /// # Calculates length of this value
    pub fn len(&self) -> io::Result<DataSize> {
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
            Value::Text(ref t) => sum!(Self::bytes_for_len(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DateTime(ref dt) => sum!(Self::bytes_for_len(dt.len())?, 2, dt.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Date(ref d) => sum!(Self::bytes_for_len(d.len())?, 2, d.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::Time(ref t) => sum!(Self::bytes_for_len(t.len())?, 2, t.len()),
            // 1 byte for type, 1 byte for null terminator
            Value::DecimalStr(ref ds) => sum!(Self::bytes_for_len(ds.len())?, 2, ds.len()),
            // 1 byte for type
            Value::Blob(ref bytes) => sum!(Self::bytes_for_len(bytes.len())?, 1, bytes.len()),
            Value::List(ref list) => Self::list_len(list),
            Value::Map(ref map) => Self::map_len(map),
            Value::Object(ref object) => Self::object_len(object),
        }
    }

    /// # Calculates bytes needed for a length
    fn bytes_for_len(len: usize) -> io::Result<DataSize> {
        match cmp_integers!(len, ::std::i8::MAX) {
            Ordering::Greater => match cmp_integers!(len, Self::MAX_DATA_SIZE) {
                Ordering::Greater => Err(Error::new(ErrorKind::InvalidData, format!("Value::bytes_for_len() -> too large: {} bytes", len))),
                _ => Ok(4),
            },
            _ => Ok(1),
        }
    }

    /// # Calculates list length
    fn list_len(list: &Vec<Self>) -> io::Result<DataSize> {
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
            false => Err(Error::new(ErrorKind::InvalidData, format!("Value::list_len() -> data too large: {} bytes", result))),
        }
    }

    /// # Calculates map length
    fn map_len(map: &BTreeMap<i32, Self>) -> io::Result<DataSize> {
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
            false => Err(Error::new(ErrorKind::InvalidData, format!("Value::map_len() -> data too large: {} bytes", result))),
        }
    }

    /// # Calculates object length
    fn object_len(object: &HashMap<String, Self>) -> io::Result<DataSize> {
        // Type + count
        let mut result = sum!(Self::bytes_for_len(object.len())?, 1)?;
        // Items
        for (key, value) in object {
            // Key has NO null terminator
            let key_len = key.len();
            if key_len > OBJECT_KEY_MAX_LEN {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Value::object_len() -> key size is limited to {} bytes; got: {}", OBJECT_KEY_MAX_LEN, &key_len)
                ));
            }
            result = sum!(result, key_len, value.len()?, 1)?;
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
            false => Err(Error::new(ErrorKind::InvalidData, format!("len() -> data too large: {} bytes", result))),
        }
    }

    /// # Writes this value into a buffer
    ///
    /// Returns the number of bytes written.
    pub fn write(&self, buf: &mut Write) -> io::Result<DataSize> {
        let expected_result = self.len()?;

        let result = match *self {
            Value::Null => write_int_be!(u8, NULL, buf)?,
            Value::True => write_int_be!(u8, TRUE, buf)?,
            Value::False => write_int_be!(u8, FALSE, buf)?,
            Value::U8(u) => sum!(write_int_be!(u8, U8, buf)?, write_int_be!(u8, u, buf)?)?,
            Value::I8(i) => sum!(write_int_be!(u8, I8, buf)?, write_int_be!(i8, i, buf)?)?,
            Value::U16(u) => sum!(write_int_be!(u8, U16, buf)?, write_int_be!(u16, u, buf)?)?,
            Value::I16(i) => sum!(write_int_be!(u8, I16, buf)?, write_int_be!(i16, i, buf)?)?,
            Value::U32(u) => sum!(write_int_be!(u8, U32, buf)?, write_int_be!(u32, u, buf)?)?,
            Value::I32(i) => sum!(write_int_be!(u8, I32, buf)?, write_int_be!(i32, i, buf)?)?,
            Value::Float(f) => sum!(write_int_be!(u8, FLOAT, buf)?, write_int_be!(u32, f.to_bits(), buf)?)?,
            Value::U64(u) => sum!(write_int_be!(u8, U64, buf)?, write_int_be!(u64, u, buf)?)?,
            Value::I64(i) => sum!(write_int_be!(u8, I64, buf)?, write_int_be!(i64, i, buf)?)?,
            Value::Double(f) => sum!(write_int_be!(u8, DOUBLE, buf)?, write_int_be!(u64, f.to_bits(), buf)?)?,
            Value::Text(ref t) => Self::write_str(TEXT, t.as_str(), buf)?,
            Value::DateTime(ref dt) => Self::write_str(DATE_TIME, dt.as_str(), buf)?,
            Value::Date(ref d) => Self::write_str(DATE, d.as_str(), buf)?,
            Value::Time(ref t) => Self::write_str(TIME, t.as_str(), buf)?,
            Value::DecimalStr(ref ds) => Self::write_str(DECIMAL_STR, ds.as_str(), buf)?,
            Value::Blob(ref bytes) => Self::write_blob(bytes.as_slice(), buf)?,
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

    /// # Writes a string into the buffer
    fn write_str(ty: u8, s: &str, buf: &mut Write) -> io::Result<DataSize> {
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
        write_size!(str_len, buf)?;

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
    fn write_blob(bytes: &[u8], buf: &mut Write) -> io::Result<DataSize> {
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
        bytes_written = sum!(write_size!(len, buf)?, bytes_written)?;

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
    fn write_list(&self, size: DataSize, list: &Vec<Self>, buf: &mut Write) -> io::Result<DataSize> {
        let mut result = sum!(
            // Type
            write_int_be!(u8, LIST, buf)?,
            // Size
            write_size!(size, buf)?,
            // Count
            // We don't have to verify this value. Since at the beginning of ::write(), we already called ::len(), which verified the whole
            // container's size.
            write_size!(list.len() as DataSize, buf)?
        )?;

        // Items
        for v in list {
            result = sum!(result, v.write(buf)?)?;
        }

        Ok(result)
    }

    /// # Writes a map into the buffer
    fn write_map(&self, size: DataSize, map: &BTreeMap<i32, Self>, buf: &mut Write) -> io::Result<DataSize> {
        let mut result = sum!(
            // Type
            write_int_be!(u8, MAP, buf)?,
            // Size
            write_size!(size, buf)?,
            // Count
            // We don't have to verify this value. Since at the beginning of ::write(), we already called ::len(), which verified the whole
            // container's size.
            write_size!(map.len() as DataSize, buf)?
        )?;

        // Items
        for (key, value) in map {
            result = sum!(result, write_int_be!(i32, key, buf)?, value.write(buf)?)?;
        }

        Ok(result)
    }

    /// # Writes an object into the buffer
    ///
    /// [`len()`]: enum.Value.html#method.len
    fn write_object(&self, size: DataSize, object: &HashMap<String, Self>, buf: &mut Write) -> io::Result<DataSize> {
        let mut result = sum!(
            // Type
            write_int_be!(u8, OBJECT, buf)?,
            // Size
            write_size!(size, buf)?,
            // Count
            // We don't have to verify this value. Since at the beginning of ::write(), we already called ::len(), which verified the whole
            // container's size.
            write_size!(object.len() as DataSize, buf)?
        )?;

        // Items
        for (key, value) in object {
            let key_len = key.len();
            result = match key_len <= OBJECT_KEY_MAX_LEN {
                true => sum!(result, write_int_be!(u8, key_len as u8, buf)?)?,
                false => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Value::write_object() -> key length is limited to {} bytes, got: {}", OBJECT_KEY_MAX_LEN, &key_len)
                )),
            };

            let written = buf.write(key.as_bytes())?;
            match cmp_integers!(written, key_len) {
                Ordering::Equal => result = sum!(result, written)?,
                _ => return Err(Error::new(
                    ErrorKind::Other, format!("Value::write_object() -> expected to write {} byte(s) of key; result: {}", &key_len, &written)
                )),
            }

            result = sum!(result, value.write(buf)?)?;
        }

        Ok(result)
    }

    /// # Reads a value from source
    pub fn read(source: &mut Read) -> io::Result<Self> {
        match read_int_be!(u8, source)? {
            self::NULL => Ok(Value::Null),
            self::TRUE => Ok(Value::True),
            self::FALSE => Ok(Value::False),
            self::U8 => Ok(Value::U8(read_int_be!(u8, source)?)),
            self::I8 => Ok(Value::I8(read_int_be!(i8, source)?)),
            self::U16 => Ok(Value::U16(read_int_be!(u16, source)?)),
            self::I16 => Ok(Value::I16(read_int_be!(i16, source)?)),
            self::U32 => Ok(Value::U32(read_int_be!(u32, source)?)),
            self::I32 => Ok(Value::I32(read_int_be!(i32, source)?)),
            self::FLOAT => Ok(Value::Float(f32::from_bits(read_int_be!(u32, source)?))),
            self::U64 => Ok(Value::U64(read_int_be!(u64, source)?)),
            self::I64 => Ok(Value::I64(read_int_be!(i64, source)?)),
            self::DOUBLE => Ok(Value::Double(f64::from_bits(read_int_be!(u64, source)?))),
            self::TEXT => Ok(Value::Text(read_str!(source)?)),
            self::DATE_TIME => Ok(Value::DateTime(read_str!(source)?)),
            self::DATE => Ok(Value::Date(read_str!(source)?)),
            self::TIME => Ok(Value::Time(read_str!(source)?)),
            self::DECIMAL_STR => Ok(Value::DecimalStr(read_str!(source)?)),
            self::BLOB => Ok(Value::Blob(read_into_new_vec!(read_size!(source)?, source)?)),
            self::LIST => read_list!(source),
            self::MAP => read_map!(source),
            self::OBJECT => read_object!(source),
            other => Err(Error::new(
                ErrorKind::InvalidData, format!("Value::read() -> data type is either invalid or not supported: {}", &other)
            )),
        }
    }

}

/// # Reads a [`Null`] from source
///
/// [`Null`]: enum.Value.html#variant.Null
pub fn read_null(source: &mut Read) -> io::Result<()> {
    match Value::read(source)? {
        Value::Null => Ok(()),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_null() -> got: {:?}", &other))),
    }
}

/// # Reads a boolean value from source
pub fn read_bool(source: &mut Read) -> io::Result<bool> {
    match Value::read(source)? {
        Value::True => Ok(true),
        Value::False => Ok(false),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_bool() -> got: {:?}", &other))),
    }
}

/// # Reads a `u8` value from source
pub fn read_u8(source: &mut Read) -> io::Result<u8> {
    match Value::read(source)? {
        Value::U8(u) => Ok(u),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_u8() -> got: {:?}", &other))),
    }
}

/// # Reads an `i8` value from source
pub fn read_i8(source: &mut Read) -> io::Result<i8> {
    match Value::read(source)? {
        Value::I8(i) => Ok(i),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_i8() -> got: {:?}", &other))),
    }
}

/// # Reads a `u16` value from source
pub fn read_u16(source: &mut Read) -> io::Result<u16> {
    match Value::read(source)? {
        Value::U16(u) => Ok(u),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_u16() -> got: {:?}", &other))),
    }
}

/// # Reads an `i16` value from source
pub fn read_i16(source: &mut Read) -> io::Result<i16> {
    match Value::read(source)? {
        Value::I16(i) => Ok(i),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_i16() -> got: {:?}", &other))),
    }
}

/// # Reads a `u32` value from source
pub fn read_u32(source: &mut Read) -> io::Result<u32> {
    match Value::read(source)? {
        Value::U32(u) => Ok(u),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_u32() -> got: {:?}", &other))),
    }
}

/// # Reads an `i32` value from source
pub fn read_i32(source: &mut Read) -> io::Result<i32> {
    match Value::read(source)? {
        Value::I32(i) => Ok(i),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_i32() -> got: {:?}", &other))),
    }
}

/// # Reads a `u64` value from source
pub fn read_u64(source: &mut Read) -> io::Result<u64> {
    match Value::read(source)? {
        Value::U64(u) => Ok(u),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_u64() -> got: {:?}", &other))),
    }
}

/// # Reads an `i64` value from source
pub fn read_i64(source: &mut Read) -> io::Result<i64> {
    match Value::read(source)? {
        Value::I64(i) => Ok(i),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_i64() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Float`] value from source
///
/// [`Float`]: enum.Value.html#variant.Float
pub fn read_float(source: &mut Read) -> io::Result<f32> {
    match Value::read(source)? {
        Value::Float(f) => Ok(f),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_float() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Double`] value from source
///
/// [`Double`]: enum.Value.html#variant.Double
pub fn read_double(source: &mut Read) -> io::Result<f64> {
    match Value::read(source)? {
        Value::Double(d) => Ok(d),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_double() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Text`] from source
///
/// [`Text`]: enum.Value.html#variant.Text
pub fn read_text(source: &mut Read) -> io::Result<String> {
    match Value::read(source)? {
        Value::Text(t) => Ok(t),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_text() -> got: {:?}", &other))),
    }
}

/// # Reads a [`DateTime`] from source
///
/// [`DateTime`]: enum.Value.html#variant.DateTime
pub fn read_date_time(source: &mut Read) -> io::Result<String> {
    match Value::read(source)? {
        Value::DateTime(dt) => Ok(dt),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_date_time() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Date`] from source
///
/// [`Date`]: enum.Value.html#variant.Date
pub fn read_date(source: &mut Read) -> io::Result<String> {
    match Value::read(source)? {
        Value::Date(d) => Ok(d),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_date() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Time`] from source
///
/// [`Time`]: enum.Value.html#variant.Time
pub fn read_time(source: &mut Read) -> io::Result<String> {
    match Value::read(source)? {
        Value::Time(t) => Ok(t),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_time() -> got: {:?}", &other))),
    }
}

/// # Reads a [`DecimalStr`] from source
///
/// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
pub fn read_decimal_str(source: &mut Read) -> io::Result<String> {
    match Value::read(source)? {
        Value::DecimalStr(ds) => Ok(ds),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_decimal_str() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Blob`] from source
///
/// [`Blob`]: enum.Value.html#variant.Blob
pub fn read_blob(source: &mut Read) -> io::Result<Vec<u8>> {
    match Value::read(source)? {
        Value::Blob(bytes) => Ok(bytes),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_blob() -> got: {:?}", &other))),
    }
}

/// # Reads a [`List`] from source
///
/// [`List`]: enum.Value.html#variant.List
pub fn read_list(source: &mut Read) -> io::Result<Vec<Value>> {
    match Value::read(source)? {
        Value::List(list) => Ok(list),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_list() -> got: {:?}", &other))),
    }
}

/// # Reads a [`Map`] from source
///
/// [`Map`]: enum.Value.html#variant.Map
pub fn read_map(source: &mut Read) -> io::Result<BTreeMap<i32, Value>> {
    match Value::read(source)? {
        Value::Map(map) => Ok(map),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_map() -> got: {:?}", &other))),
    }
}

/// # Reads an [`Object`] from source
///
/// [`Object`]: enum.Value.html#variant.Object
pub fn read_object(source: &mut Read) -> io::Result<HashMap<String, Value>> {
    match Value::read(source)? {
        Value::Object(object) => Ok(object),
        other => Err(Error::new(ErrorKind::InvalidData, format!("Value::read_object() -> got: {:?}", &other))),
    }
}
