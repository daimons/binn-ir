// License: see LICENSE file at root directory of `master` branch

//! # Values

use {
    crate::Size,
};

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

/// # Object key's max length
pub const OBJECT_KEY_MAX_LEN: usize = 255;

/// # Max data size, in bytes
pub const MAX_DATA_SIZE: Size = i32::max_value() as Size;
