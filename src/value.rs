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
pub const OBJECT_KEY_MAX_LEN: usize = 255;

/// # Max data size, in bytes
pub const MAX_DATA_SIZE: u32 = ::std::i32::MAX as u32;

/// # Values
#[derive(Clone, PartialEq)]
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
    let limit = 50;
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

impl AsRef<Value> for Value {

    fn as_ref(&self) -> &Self {
        self
    }

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

impl From<u8> for Value {

    /// # Converts input to a [`U8`]
    ///
    /// [`U8`]: enum.Value.html#variant.U8
    fn from(u: u8) -> Self {
        Value::U8(u)
    }

}

impl From<i8> for Value {

    /// # Converts input to an [`I8`]
    ///
    /// [`I8`]: enum.Value.html#variant.I8
    fn from(i: i8) -> Self {
        Value::I8(i)
    }

}

impl From<u16> for Value {

    /// # Converts input to a [`U16`]
    ///
    /// [`U16`]: enum.Value.html#variant.U16
    fn from(u: u16) -> Self {
        Value::U16(u)
    }

}

impl From<i16> for Value {

    /// # Converts input to an [`I16`]
    ///
    /// [`I16`]: enum.Value.html#variant.I16
    fn from(i: i16) -> Self {
        Value::I16(i)
    }

}

impl From<u32> for Value {

    /// # Converts input to a [`U32`]
    ///
    /// [`U32`]: enum.Value.html#variant.U32
    fn from(u: u32) -> Self {
        Value::U32(u)
    }

}

impl From<i32> for Value {

    /// # Converts input to an [`I32`]
    ///
    /// [`I32`]: enum.Value.html#variant.I32
    fn from(i: i32) -> Self {
        Value::I32(i)
    }

}

impl From<f32> for Value {

    /// # Converts input to a [`Float`]
    ///
    /// [`Float`]: enum.Value.html#variant.Float
    fn from(f: f32) -> Self {
        Value::Float(f)
    }

}

impl From<u64> for Value {

    /// # Converts input to a [`U64`]
    ///
    /// [`U64`]: enum.Value.html#variant.U64
    fn from(u: u64) -> Self {
        Value::U64(u)
    }

}

impl From<i64> for Value {

    /// # Converts input to an [`I64`]
    ///
    /// [`I64`]: enum.Value.html#variant.I64
    fn from(i: i64) -> Self {
        Value::I64(i)
    }

}

impl From<f64> for Value {

    /// # Converts input to a [`Double`]
    ///
    /// [`Double`]: enum.Value.html#variant.Double
    fn from(f: f64) -> Self {
        Value::Double(f)
    }

}

impl From<String> for Value {

    /// # Converts input to a [`Text`]
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn from(s: String) -> Self {
        Value::Text(s)
    }

}

impl<'a> From<&'a str> for Value {

    /// # Converts input to a [`Text`]
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn from(s: &'a str) -> Self {
        // Note that some variants also accept a String, so forward this call to implementation of `From<String> for Value`, let it decide.
        Self::from(s.to_owned())
    }

}

impl From<Vec<u8>> for Value {

    /// # Converts input to a [`Blob`]
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }

}

impl<'a> From<&'a Vec<u8>> for Value {

    /// # Converts input to a [`Blob`]
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn from(v: &'a Vec<u8>) -> Self {
        Self::from(v.to_owned())
    }

}

impl<'a> From<&'a [u8]> for Value {

    /// # Converts input to a [`Blob`]
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn from(v: &'a [u8]) -> Self {
        Self::from(v.to_owned())
    }

}

impl From<Vec<Value>> for Value {

    /// # Converts input to a [`List`]
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn from(list: Vec<Value>) -> Self {
        Value::List(list)
    }

}

impl<'a> From<&'a Vec<Value>> for Value {

    /// # Converts input to a [`List`]
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn from(list: &'a Vec<Value>) -> Self {
        Self::from(list.to_owned())
    }

}

impl From<BTreeMap<i32, Value>> for Value {

    /// # Converts input to a [`Map`]
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn from(map: BTreeMap<i32, Value>) -> Self {
        Value::Map(map)
    }

}

impl<'a> From<&'a BTreeMap<i32, Value>> for Value {

    /// # Converts input to a [`Map`]
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn from(map: &'a BTreeMap<i32, Value>) -> Self {
        Self::from(map.to_owned())
    }

}

impl From<HashMap<String, Value>> for Value {

    /// # Converts input to an [`Object`]
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn from(object: HashMap<String, Value>) -> Self {
        Value::Object(object)
    }

}

impl<'a> From<&'a HashMap<String, Value>> for Value {

    /// # Converts input to an [`Object`]
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn from(object: &'a HashMap<String, Value>) -> Self {
        Self::from(object.to_owned())
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

/// # Converts an integer value to big-endian order and writes it into the buffer
///
/// Returns: number of bytes written, as `io::Result<u32>`.
macro_rules! write_int_be { ($ty: ty, $v: expr, $buf: ident) => {{
    let bytes = as_bytes!($ty, $v.to_be());
    match $buf.write(&bytes) {
        Ok(count) => match count == bytes.len() {
            true => Ok(count as u32),
            false => Err(Error::new(
                ErrorKind::Other, format!("{}::value::write_int_be!() -> expected to write {} byte(s); result: {}", ::TAG, bytes.len(), count)
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
/// Result: number of bytes written - `io::Result<u32>`.
macro_rules! write_size { ($size: expr, $buf: ident) => {{
    let size = $size;
    match cmp_integers!(size, ::std::i8::MAX) {
        Ordering::Greater => write_int_be!(u32, size | SIZE_MASK, $buf),
        _ => write_int_be!(u8, size as u8, $buf),
    }
}};}

/// # Reads size from source
///
/// Result:
///
/// - First value is size.
/// - Second value is total bytes read.
fn read_size(source: &mut Read) -> io::Result<(u32, u32)> {
    let first_byte = read_int_be!(u8, source)?;
    match first_byte & 0b_1000_0000 {
        0b_1000_0000 => {
            let mut buf = [first_byte, 0, 0, 0];
            source.read_exact(&mut buf[1..]).and_then(|()|
                Ok((u32::from_be(unsafe { mem::transmute(buf) }) & !(SIZE_MASK), mem::size_of::<u32>() as u32))
            )
        },
        _ => Ok((first_byte as u32, mem::size_of::<u8>() as u32)),
    }
}

#[test]
fn test_read_size() {
    use ::std::io::Cursor;

    const U32_SIZE: u32 = mem::size_of::<u32>() as u32;
    const MAX_U8: u8 = ::std::u8::MAX;

    assert_eq!(read_size(&mut Cursor::new(vec![MAX_U8, MAX_U8, MAX_U8, MAX_U8])).unwrap(), (MAX_DATA_SIZE, U32_SIZE));

    for bytes in vec![
        [0xF0, MAX_U8, MAX_U8, MAX_U8],
        [0x80, MAX_U8, MAX_U8, MAX_U8],
        [MAX_U8, MAX_U8, MAX_U8, 0xF0],
    ] {
        let (size, bytes_of_size) = read_size(&mut Cursor::new(bytes)).unwrap();
        assert!(size < MAX_DATA_SIZE);
        assert_ne!(size, U32_SIZE);
        assert_eq!(bytes_of_size, U32_SIZE);
    }
}

/// # Calculates sum of first value (`u32`) with integer(s)
///
/// Result: `io::Result<u32>`.
///
/// If result > [`MAX_DATA_SIZE`], an error is returned.
///
/// [`MAX_DATA_SIZE`]: constant.MAX_DATA_SIZE.html
macro_rules! sum {
    ($a: expr, $($b: expr),+) => {{
        // Do NOT nest multiple calls to cmp_integers(...); or the compiler will hang up!!!
        let mut result: io::Result<u32> = Ok($a);
        $(
            match result {
                Ok(current) => result = {
                    let b = $b;
                    match cmp_integers!(b, MAX_DATA_SIZE) {
                        Ordering::Greater => Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("{}::value::sum!() -> too large for: {} + {} (max allowed: {})", ::TAG, &current, &b, MAX_DATA_SIZE)
                        )),
                        _ => match current.checked_add(b as u32) {
                            Some(new) => match cmp_integers!(new, MAX_DATA_SIZE) {
                                Ordering::Greater => Err(Error::new(
                                    ErrorKind::InvalidData,
                                    format!("{}::value::sum!() -> too large for: {} + {} (max allowed: {})", ::TAG, &current, &b, MAX_DATA_SIZE)
                                )),
                                _ => Ok(new),
                            },
                            None => Err(Error::new(
                                ErrorKind::InvalidData, format!("{}::value::sum!() -> can't add {} into {}", ::TAG, &b, &current)
                            )),
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
    match cmp_integers!(capacity, MAX_DATA_SIZE) {
        Ordering::Greater => Err(Error::new(
            ErrorKind::WriteZero,
            format!(
                "{}::value::new_vec_with_capacity!() -> cannot allocate a vector with capacity: {} (max allowed: {})",
                ::TAG, &capacity, MAX_DATA_SIZE
            )
        )),
        _ => match cmp_integers!(capacity, ::std::usize::MAX) {
            Ordering::Greater => Err(Error::new(
                ErrorKind::WriteZero,
                format!(
                    "{}::value::new_vec_with_capacity!() -> cannot allocate a vector with capacity: {} (max allowed: {})",
                    ::TAG, &capacity, ::std::usize::MAX
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
    // - `len` was verified via above call to `new_vec_with_capacity!()`, that it must be <= `MAX_DATA_SIZE`
    // - `MAX_DATA_SIZE` should be **tested** to be < `std::u64::MAX`
    match $source.take(len as u64).read_to_end(&mut result) {
        Ok(read) => match cmp_integers!(read, len) {
            Ordering::Equal => Ok(result),
            _ => Err(Error::new(
                ErrorKind::WriteZero, format!("{}::value::read_into_new_vec!() -> expected to read {} bytes, but: {}", ::TAG, &len, &read)
            )),
        },
        Err(err) => Err(Error::new(
            ErrorKind::WriteZero, format!("{}::value::read_into_new_vec!() -> failed to read {} bytes: {}", ::TAG, &len, &err)
        )),
    }
}};}

/// # Reads a string from source
///
/// Returns: `io::Result<String>`
macro_rules! read_str { ($source: ident) => {{
    // Note that null terminator does NOT count
    let buf = read_into_new_vec!(read_size($source)?.0, $source)?;
    match read_int_be!(u8, $source)? {
        0 => String::from_utf8(buf).map_err(|err|
            Error::new(ErrorKind::InvalidData, format!("{}::value::read_str!() -> failed to decode UTF-8: {}", ::TAG, &err))
        ),
        other => Err(Error::new(
            ErrorKind::InvalidData, format!("{}::value::read_str!() -> expected to read a null terminator ('\\0'), got: {}", ::TAG, &other)
        )),
    }
}};}

/// # Calculates bytes needed for a length
///
/// Result: `io::Result<u32>`
macro_rules! bytes_for_len { ($len: expr) => {{
    let len = $len;
    match cmp_integers!(len, ::std::i8::MAX) {
        Ordering::Greater => match cmp_integers!(len, MAX_DATA_SIZE) {
            Ordering::Greater => Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::bytes_for_len!() -> too large: {} bytes", ::TAG, &len)
            )),
            _ => Ok(4_u32),
        },
        _ => Ok(1_u32),
    }
}};}

/// # Decodes a list from source
///
/// Returns: `io::Result<Option<Value>>`
macro_rules! decode_list { ($source: ident) => {{
    let (size, bytes_of_size) = read_size($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(Error::new(ErrorKind::InvalidData, format!("{}::value::decode_list!() -> invalid declared size: {}", ::TAG, &size)));
    }

    let (item_count, bytes_of_item_count) = read_size($source)?;

    let mut result = vec![];
    let mut read: u32 = sum!(bytes_of_size, bytes_of_item_count)?;
    for item_index in 0..item_count {
        let value = match Value::decode($source)? {
            Some(value) => value,
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{}::value::decode_list!() -> missing item #{}/{}", ::TAG, &item_index, &item_count)
            )),
        };
        read = match read.checked_add(value.len()?) {
            Some(v) => match cmp_integers!(size, v) {
                Ordering::Greater => v,
                _ => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("{}::value::decode_list!() -> expected to read less than {} bytes, got: {}", ::TAG, &size, &v)
                )),
            },
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "{}::value::decode_list!() -> invalid list size -> expected: {}, current: {}, new item: {:?}", ::TAG, &size, &read, &value,
                )
            )),
        };
        result.push(value);
    }

    // Verify total read (1 byte for header)
    match read.checked_add(1) {
        Some(v) if v == size => Ok(Some(Value::List(result))),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("{}::value::decode_list!() -> size is declared: {}; but decoded (with or without header): {}", ::TAG, &size, &read)
        )),
    }
}};}

/// # Decodes a map from source
///
/// Returns: `io::Result<Option<Value>>`
macro_rules! decode_map { ($source: ident) => {{
    let (size, bytes_of_size) = read_size($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(Error::new(ErrorKind::InvalidData, format!("{}::value::decode_map!() -> invalid declared size: {}", ::TAG, &size)));
    }

    let (item_count, bytes_of_item_count) = read_size($source)?;

    let mut result = BTreeMap::new();
    let mut read: u32 = sum!(bytes_of_size, bytes_of_item_count)?;
    for _ in 0..item_count {
        let key = read_int_be!(i32, $source)?;
        let value = match Value::decode($source)? {
            Some(value) => value,
            None => return Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::decode_map!() -> missing value for key {}", ::TAG, &key)
            )),
        };
        read = match read.checked_add(sum!(mem::size_of_val(&key) as u32, value.len()?)?) {
            Some(v) => match cmp_integers!(size, v) {
                Ordering::Greater => v,
                _ => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("{}::value::decode_map!() -> expected to read less than {} bytes, got: {}", ::TAG, &size, &v)
                )),
            },
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "{}::value::decode_map!() -> invalid map size -> expected: {}, current: {}, new item: {} -> {:?}",
                    ::TAG, &size, &read, &key, &value,
                )
            )),
        };
        match result.insert(key, value) {
            Some(old_value) => return Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::decode_map!() -> duplicate key '{}' of old value: {:?}", ::TAG, &key, &old_value)
            )),
            None => (),
        };
    }

    // Verify total read (1 byte for header)
    match read.checked_add(1) {
        Some(v) if v == size => Ok(Some(Value::Map(result))),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("{}::value::decode_map!() -> size is declared: {}; but decoded (with or without header): {}", ::TAG, &size, &read)
        )),
    }
}};}

/// # Decodes an object from source
///
/// Returns: `io::Result<Option<Value>>`
macro_rules! decode_object { ($source: ident) => {{
    let (size, bytes_of_size) = read_size($source)?;
    // 1 byte for header; at least 1 byte for size; at least 1 byte for item count
    if size < 3 {
        return Err(Error::new(ErrorKind::InvalidData, format!("{}::value::decode_object!() -> invalid declared size: {}", ::TAG, &size)));
    }

    let (item_count, bytes_of_item_count) = read_size($source)?;

    let mut result = HashMap::new();
    let mut read: u32 = sum!(bytes_of_size, bytes_of_item_count)?;
    for _ in 0..item_count {
        // Read key (note that there's NO null terminator)
        let (key_len, bytes_of_key_len) = read_size($source)?;
        match cmp_integers!(key_len, OBJECT_KEY_MAX_LEN) {
            Ordering::Greater => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{}::value::decode_object!() -> key length is limited to {} bytes, got: {}", ::TAG, OBJECT_KEY_MAX_LEN, key_len)
            )),
            _ => read = match read.checked_add(sum!(bytes_of_key_len, key_len)?) {
                Some(v) => match cmp_integers!(size, v) {
                    Ordering::Greater => v,
                    _ => return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("{}::value::decode_object!() -> expected to read less than {} bytes, got: {}", ::TAG, &size, &v)
                    )),
                },
                None => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "{}::value::decode_object!() -> invalid object size -> expected: {}, current: {}, new key length: {} + {}",
                        ::TAG, &size, &read, &bytes_of_key_len, &key_len,
                    )
                )),
            },
        };
        let key = String::from_utf8(read_into_new_vec!(key_len, $source)?).map_err(|err|
            Error::new(ErrorKind::InvalidData, format!("{}::value::decode_object!() -> failed to decode UTF-8: {}", ::TAG, &err))
        )?;

        // Read value
        let value = match Value::decode($source)? {
            Some(value) => value,
            None => return Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::decode_object!() -> missing value for key {:?}", ::TAG, &key)
            )),
        };
        read = match read.checked_add(value.len()?) {
            Some(v) => match cmp_integers!(size, v) {
                Ordering::Greater => v,
                _ => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("{}::value::decode_object!() -> expected to read less than {} bytes, got: {}", ::TAG, &size, &v)
                )),
            },
            None => return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "{}::value::decode_object!() -> invalid object size -> expected: {}, current: {}, new value: {:?}",
                    ::TAG, &size, &read, &value,
                )
            )),
        };
        match result.insert(key, value) {
            Some(old_value) => return Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::decode_object!() -> duplicate key of old value: {:?}", ::TAG, &old_value)
            )),
            None => (),
        };
    }

    // Verify total read (1 byte for header)
    match read.checked_add(1) {
        Some(v) if v == size => Ok(Some(Value::Object(result))),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("{}::value::decode_object!() -> size is declared: {}; but decoded (with or without header): {}", ::TAG, &size, &read)
        )),
    }
}};}

impl Value {

    /// # Calculates length of this value
    pub fn len(&self) -> io::Result<u32> {
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
    pub fn encode(&self, buf: &mut Write) -> io::Result<u32> {
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
            Value::Text(ref t) => write_str(TEXT, t.as_str(), buf)?,
            Value::DateTime(ref dt) => write_str(DATE_TIME, dt.as_str(), buf)?,
            Value::Date(ref d) => write_str(DATE, d.as_str(), buf)?,
            Value::Time(ref t) => write_str(TIME, t.as_str(), buf)?,
            Value::DecimalStr(ref ds) => write_str(DECIMAL_STR, ds.as_str(), buf)?,
            Value::Blob(ref bytes) => write_blob(bytes.as_slice(), buf)?,
            Value::List(ref list) => write_list(expected_result, list, buf)?,
            Value::Map(ref map) => write_map(expected_result, map, buf)?,
            Value::Object(ref object) => write_object(expected_result, object, buf)?,
        };

        match result == expected_result {
            true => Ok(result),
            false => Err(Error::new(
                ErrorKind::Other,
                format!("{}::value::Value::encode() -> expected to write {} bytes, result: {}", ::TAG, expected_result, result)
            )),
        }
    }

    /// # Decodes a value from source
    ///
    /// If it returns `Ok(None)`, it means there's no more data to decode.
    pub fn decode(source: &mut Read) -> io::Result<Option<Self>> {
        decode_value(None, source)
    }

}

/// # Decodes a value from source
///
/// If `filter` is provided, the function expects that next value from source is one of them, and returns an error if not.
///
/// If `filter` is `None`, the function decodes any value from source.
fn decode_value(filter: Option<&[u8]>, source: &mut Read) -> io::Result<Option<Value>> {
    let source_value = match read_int_be!(u8, source) {
        Ok(source_value) => source_value,
        Err(err) => return match err.kind() {
            ErrorKind::UnexpectedEof => Ok(None),
            _ => Err(err),
        },
    };

    if let Some(ref expected_values) = filter {
        if expected_values.contains(&source_value) == false {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{}::value::decode_value() -> expected one of: {:?}, got: {}", ::TAG, &expected_values, &source_value)
            ));
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
        self::BLOB => Ok(Some(Value::Blob(read_into_new_vec!(read_size(source)?.0, source)?))),
        self::LIST => decode_list!(source),
        self::MAP => decode_map!(source),
        self::OBJECT => decode_object!(source),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("{}::value::decode_value() -> data type is either invalid or not supported: {}", ::TAG, &source_value)
        )),
    }
}

/// # Calculates list length
fn list_len(list: &Vec<Value>) -> io::Result<u32> {
    // Type + count
    let mut result: u32 = sum!(bytes_for_len!(list.len())?, 1)?;
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
    match result <= MAX_DATA_SIZE {
        true => Ok(result),
        false => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::list_len() -> data too large: {} bytes", ::TAG, result))),
    }
}

/// # Calculates map length
fn map_len(map: &BTreeMap<i32, Value>) -> io::Result<u32> {
    // Type + count
    let mut result = sum!(bytes_for_len!(map.len())?, 1)?;
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
    match result <= MAX_DATA_SIZE {
        true => Ok(result),
        false => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::map_len() -> data too large: {} bytes", ::TAG, result))),
    }
}

/// # Calculates object length
fn object_len(object: &HashMap<String, Value>) -> io::Result<u32> {
    // Type + count
    let mut result = sum!(bytes_for_len!(object.len())?, 1)?;
    // Items
    for (key, value) in object {
        // Key has NO null terminator
        let key_len = key.len();
        if key_len > OBJECT_KEY_MAX_LEN {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{}::value::object_len() -> key size is limited to {} bytes; got: {}", ::TAG, OBJECT_KEY_MAX_LEN, &key_len)
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
    match result <= MAX_DATA_SIZE {
        true => Ok(result),
        false => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::object_len() -> data too large: {} bytes", ::TAG, result))),
    }
}

/// # Writes a string into the buffer
fn write_str(ty: u8, s: &str, buf: &mut Write) -> io::Result<u32> {
    let bytes = s.as_bytes();
    let str_len = {
        let tmp = bytes.len();
        match cmp_integers!(tmp, MAX_DATA_SIZE) {
            Ordering::Greater => return Err(Error::new(
                ErrorKind::Other, format!("{}::value::write_str() -> string too large ({} bytes)", ::TAG, &tmp)
            )),
            _ => tmp as u32,
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
        other => return Err(Error::new(
            ErrorKind::Other, format!("{}::value::write_str() -> expected to write 1 byte; result: {}", ::TAG, &other)
        )),
    };

    // Size
    // Note that null terminator does NOT count
    write_size!(str_len, buf)?;

    // Data
    let written = buf.write(bytes)?;
    match cmp_integers!(written, str_len) {
        Ordering::Equal => (),
        _ => return Err(Error::new(
            ErrorKind::Other, format!("{}::value::write_str() -> expected to write {} byte(s); result: {}", ::TAG, str_len, written)
        )),
    };

    // Null terminator
    match buf.write(&[0])? {
        1 => (),
        other => return Err(Error::new(
            ErrorKind::Other, format!("{}::value::write_str() -> expected to write 1 byte; result: {}", ::TAG, &other)
        )),
    };

    Ok(total_size)
}

/// # Writes blob into the buffer
fn write_blob(bytes: &[u8], buf: &mut Write) -> io::Result<u32> {
    let len = {
        let tmp = bytes.len();
        match cmp_integers!(tmp, MAX_DATA_SIZE) {
            Ordering::Greater => return Err(Error::new(
                ErrorKind::Other, format!("{}::value::write_blob() -> too large: {} byte(s)", ::TAG, tmp)
            )),
            _ => tmp as u32,
        }
    };

    // Type
    let mut bytes_written = match buf.write(&[BLOB])? {
        1 => 1 as u32,
        other => return Err(Error::new(
            ErrorKind::Other, format!("{}::value::write_blob() -> expected to write 1 byte; result: {}", ::TAG, &other)
        )),
    };

    // Size
    bytes_written = sum!(write_size!(len, buf)?, bytes_written)?;

    // Data
    let written = buf.write(bytes)?;
    match cmp_integers!(written, len) {
        Ordering::Equal => (),
        _ => return Err(Error::new(
            ErrorKind::Other, format!("{}::value::write_blob() -> expected to write {} byte(s); result: {}", ::TAG, &len, &written)
        )),
    };
    bytes_written = sum!(bytes_written, written)?;

    Ok(bytes_written)
}

/// # Writes a list into the buffer
fn write_list(size: u32, list: &Vec<Value>, buf: &mut Write) -> io::Result<u32> {
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

/// # Writes a map into the buffer
fn write_map(size: u32, map: &BTreeMap<i32, Value>, buf: &mut Write) -> io::Result<u32> {
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

/// # Writes an object into the buffer
///
/// [`len()`]: enum.Value.html#method.len
fn write_object(size: u32, object: &HashMap<String, Value>, buf: &mut Write) -> io::Result<u32> {
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
            false => return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{}::value::write_object() -> key length is limited to {} bytes, got: {}", ::TAG, OBJECT_KEY_MAX_LEN, &key_len)
            )),
        };

        let written = buf.write(key.as_bytes())?;
        match cmp_integers!(written, key_len) {
            Ordering::Equal => result = sum!(result, written)?,
            _ => return Err(Error::new(
                ErrorKind::Other,
                format!("{}::value::write_object() -> expected to write {} byte(s) of key; result: {}", ::TAG, &key_len, &written)
            )),
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
    fn encode(&mut self, value: impl AsRef<Value>) -> io::Result<u32> {
        value.as_ref().encode(self)
    }

    /// # Encodes a [`Null`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Null`]: enum.Value.html#variant.Null
    fn encode_null(&mut self) -> io::Result<u32> {
        Value::Null.encode(self)
    }

    /// # Encodes a `bool` via [`True`] or [`False`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`True`]: enum.Value.html#variant.True
    /// [`False`]: enum.Value.html#variant.False
    fn encode_bool(&mut self, b: impl Into<bool>) -> io::Result<u32> {
        match b.into() {
            true => Value::True.encode(self),
            false => Value::False.encode(self),
        }
    }

    /// # Encodes a [`U8`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U8`]: enum.Value.html#variant.U8
    fn encode_u8(&mut self, u: impl Into<u8>) -> io::Result<u32> {
        Value::U8(u.into()).encode(self)
    }

    /// # Encodes an [`I8`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I8`]: enum.Value.html#variant.I8
    fn encode_i8(&mut self, i: impl Into<i8>) -> io::Result<u32> {
        Value::I8(i.into()).encode(self)
    }

    /// # Encodes a [`U16`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U16`]: enum.Value.html#variant.U16
    fn encode_u16(&mut self, u: impl Into<u16>) -> io::Result<u32> {
        Value::U16(u.into()).encode(self)
    }

    /// # Encodes an [`I16`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I16`]: enum.Value.html#variant.I16
    fn encode_i16(&mut self, i: impl Into<i16>) -> io::Result<u32> {
        Value::I16(i.into()).encode(self)
    }

    /// # Encodes a [`U32`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U32`]: enum.Value.html#variant.U32
    fn encode_u32(&mut self, u: impl Into<u32>) -> io::Result<u32> {
        Value::U32(u.into()).encode(self)
    }

    /// # Encodes an [`I32`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I32`]: enum.Value.html#variant.I32
    fn encode_i32(&mut self, i: impl Into<i32>) -> io::Result<u32> {
        Value::I32(i.into()).encode(self)
    }

    /// # Encodes a [`U64`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`U64`]: enum.Value.html#variant.U64
    fn encode_u64(&mut self, u: impl Into<u64>) -> io::Result<u32> {
        Value::U64(u.into()).encode(self)
    }

    /// # Encodes an [`I64`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`I64`]: enum.Value.html#variant.I64
    fn encode_i64(&mut self, i: impl Into<i64>) -> io::Result<u32> {
        Value::I64(i.into()).encode(self)
    }

    /// # Encodes a [`Float`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Float`]: enum.Value.html#variant.Float
    fn encode_float(&mut self, f: impl Into<f32>) -> io::Result<u32> {
        Value::Float(f.into()).encode(self)
    }

    /// # Encodes a [`Double`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Double`]: enum.Value.html#variant.Double
    fn encode_double(&mut self, f: impl Into<f64>) -> io::Result<u32> {
        Value::Double(f.into()).encode(self)
    }

    /// # Encodes a [`Text`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn encode_text(&mut self, s: impl Into<String>) -> io::Result<u32> {
        Value::Text(s.into()).encode(self)
    }

    /// # Encodes a [`DateTime`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`DateTime`]: enum.Value.html#variant.DateTime
    fn encode_date_time(&mut self, s: impl Into<String>) -> io::Result<u32> {
        Value::DateTime(s.into()).encode(self)
    }

    /// # Encodes a [`Date`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Date`]: enum.Value.html#variant.Date
    fn encode_date(&mut self, s: impl Into<String>) -> io::Result<u32> {
        Value::Date(s.into()).encode(self)
    }

    /// # Encodes a [`Time`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Time`]: enum.Value.html#variant.Time
    fn encode_time(&mut self, s: impl Into<String>) -> io::Result<u32> {
        Value::Time(s.into()).encode(self)
    }

    /// # Encodes a [`DecimalStr`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
    fn encode_decimal_str(&mut self, s: impl Into<String>) -> io::Result<u32> {
        Value::DecimalStr(s.into()).encode(self)
    }

    /// # Encodes a [`Blob`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn encode_blob(&mut self, bytes: impl Into<Vec<u8>>) -> io::Result<u32> {
        Value::Blob(bytes.into()).encode(self)
    }

    /// # Encodes a [`List`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn encode_list(&mut self, list: impl Into<Vec<Value>>) -> io::Result<u32> {
        Value::List(list.into()).encode(self)
    }

    /// # Encodes a [`Map`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn encode_map(&mut self, map: impl Into<BTreeMap<i32, Value>>) -> io::Result<u32> {
        Value::Map(map.into()).encode(self)
    }

    /// # Encodes an [`Object`]
    ///
    /// Result: total bytes that have been written.
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn encode_object(&mut self, object: impl Into<HashMap<String, Value>>) -> io::Result<u32> {
        Value::Object(object.into()).encode(self)
    }

}

impl Encoder for ::std::fs::File {}
impl<'a> Encoder for &'a ::std::fs::File {}
impl<W: Write> Encoder for ::std::io::BufWriter<W> {}
impl<'a> Encoder for ::std::io::Cursor<&'a mut [u8]> {}
impl<'a> Encoder for ::std::io::Cursor<&'a mut Vec<u8>> {}
impl Encoder for ::std::io::Cursor<Vec<u8>> {}
impl Encoder for ::std::io::Cursor<Box<[u8]>> {}
impl<W: Write + ?Sized> Encoder for Box<W> {}
impl<'a> Encoder for &'a mut [u8] {}
impl Encoder for Vec<u8> {}
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
    fn decode(&mut self) -> io::Result<Option<Value>> {
        Value::decode(self)
    }

    /// # Decodes a [`Null`]
    ///
    /// [`Null`]: enum.Value.html#variant.Null
    fn decode_null(&mut self) -> io::Result<Option<()>> {
        match decode_value(Some(&[NULL]), self)? {
            Some(Value::Null) => Ok(Some(())),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_null() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a boolean value
    fn decode_bool(&mut self) -> io::Result<Option<bool>> {
        match decode_value(Some(&[TRUE, FALSE]), self)? {
            Some(Value::True) => Ok(Some(true)),
            Some(Value::False) => Ok(Some(false)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_bool() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a `u8` value
    fn decode_u8(&mut self) -> io::Result<Option<u8>> {
        match decode_value(Some(&[U8]), self)? {
            Some(Value::U8(u)) => Ok(Some(u)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_u8() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes an `i8` value
    fn decode_i8(&mut self) -> io::Result<Option<i8>> {
        match decode_value(Some(&[I8]), self)? {
            Some(Value::I8(i)) => Ok(Some(i)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_i8() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a `u16` value
    fn decode_u16(&mut self) -> io::Result<Option<u16>> {
        match decode_value(Some(&[U16]), self)? {
            Some(Value::U16(u)) => Ok(Some(u)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_u16() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes an `i16` value
    fn decode_i16(&mut self) -> io::Result<Option<i16>> {
        match decode_value(Some(&[I16]), self)? {
            Some(Value::I16(i)) => Ok(Some(i)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_i16() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a `u32` value
    fn decode_u32(&mut self) -> io::Result<Option<u32>> {
        match decode_value(Some(&[U32]), self)? {
            Some(Value::U32(u)) => Ok(Some(u)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_u32() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }
    /// # Decodes an `i32` value
    fn decode_i32(&mut self) -> io::Result<Option<i32>> {
        match decode_value(Some(&[I32]), self)? {
            Some(Value::I32(i)) => Ok(Some(i)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_i32() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a `u64` value
    fn decode_u64(&mut self) -> io::Result<Option<u64>> {
        match decode_value(Some(&[U64]), self)? {
            Some(Value::U64(u)) => Ok(Some(u)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_u64() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes an `i64` value
    fn decode_i64(&mut self) -> io::Result<Option<i64>> {
        match decode_value(Some(&[I64]), self)? {
            Some(Value::I64(i)) => Ok(Some(i)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_i64() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Float`] value
    ///
    /// [`Float`]: enum.Value.html#variant.Float
    fn decode_float(&mut self) -> io::Result<Option<f32>> {
        match decode_value(Some(&[FLOAT]), self)? {
            Some(Value::Float(f)) => Ok(Some(f)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_float() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Double`] value
    ///
    /// [`Double`]: enum.Value.html#variant.Double
    fn decode_double(&mut self) -> io::Result<Option<f64>> {
        match decode_value(Some(&[DOUBLE]), self)? {
            Some(Value::Double(d)) => Ok(Some(d)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_double() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Text`]
    ///
    /// [`Text`]: enum.Value.html#variant.Text
    fn decode_text(&mut self) -> io::Result<Option<String>> {
        match decode_value(Some(&[TEXT]), self)? {
            Some(Value::Text(t)) => Ok(Some(t)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_text() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`DateTime`]
    ///
    /// [`DateTime`]: enum.Value.html#variant.DateTime
    fn decode_date_time(&mut self) -> io::Result<Option<String>> {
        match decode_value(Some(&[DATE_TIME]), self)? {
            Some(Value::DateTime(dt)) => Ok(Some(dt)),
            Some(other) => Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::Decoder::decode_date_time() -> got: {:?}", ::TAG, &other)
            )),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Date`]
    ///
    /// [`Date`]: enum.Value.html#variant.Date
    fn decode_date(&mut self) -> io::Result<Option<String>> {
        match decode_value(Some(&[DATE]), self)? {
            Some(Value::Date(d)) => Ok(Some(d)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_date() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Time`]
    ///
    /// [`Time`]: enum.Value.html#variant.Time
    fn decode_time(&mut self) -> io::Result<Option<String>> {
        match decode_value(Some(&[TIME]), self)? {
            Some(Value::Time(t)) => Ok(Some(t)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_time() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`DecimalStr`]
    ///
    /// [`DecimalStr`]: enum.Value.html#variant.DecimalStr
    fn decode_decimal_str(&mut self) -> io::Result<Option<String>> {
        match decode_value(Some(&[DECIMAL_STR]), self)? {
            Some(Value::DecimalStr(ds)) => Ok(Some(ds)),
            Some(other) => Err(Error::new(
                ErrorKind::InvalidData, format!("{}::value::Decoder::decode_decimal_str() -> got: {:?}", ::TAG, &other)
            )),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Blob`]
    ///
    /// [`Blob`]: enum.Value.html#variant.Blob
    fn decode_blob(&mut self) -> io::Result<Option<Vec<u8>>> {
        match decode_value(Some(&[BLOB]), self)? {
            Some(Value::Blob(bytes)) => Ok(Some(bytes)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_blob() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`List`]
    ///
    /// [`List`]: enum.Value.html#variant.List
    fn decode_list(&mut self) -> io::Result<Option<Vec<Value>>> {
        match decode_value(Some(&[LIST]), self)? {
            Some(Value::List(list)) => Ok(Some(list)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_list() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes a [`Map`]
    ///
    /// [`Map`]: enum.Value.html#variant.Map
    fn decode_map(&mut self) -> io::Result<Option<BTreeMap<i32, Value>>> {
        match decode_value(Some(&[MAP]), self)? {
            Some(Value::Map(map)) => Ok(Some(map)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_map() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
    }

    /// # Decodes an [`Object`]
    ///
    /// [`Object`]: enum.Value.html#variant.Object
    fn decode_object(&mut self) -> io::Result<Option<HashMap<String, Value>>> {
        match decode_value(Some(&[OBJECT]), self)? {
            Some(Value::Object(object)) => Ok(Some(object)),
            Some(other) => Err(Error::new(ErrorKind::InvalidData, format!("{}::value::Decoder::decode_object() -> got: {:?}", ::TAG, &other))),
            None => Ok(None),
        }
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
