// License: see LICENSE file at root directory of `master` branch

//! # Values

use std::collections::{BTreeMap, HashMap};

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
