// License: see LICENSE file at root directory of `master` branch

//! # Storages

/// # No bytes
pub const NO_BYTES: u8 = 0b_000;

/// # Byte
pub const BYTE: u8 = 0b_001;

/// # Word
pub const WORD: u8 = 0b_010;

/// # D-word
pub const DWORD: u8 = 0b_011;

/// # Q-word
pub const QWORD: u8 = 0b_100;

/// # String
pub const STRING: u8 = 0b_101;

/// # Blob
pub const BLOB: u8 = 0b_110;

/// # Container
///
/// Container might be:
///
/// - [`List`]
/// - [`Map`]
/// - [`Object`]
///
/// [`List`]: ../enum.Value.html#variant.List
/// [`Map`]: ../enum.Value.html#variant.Map
/// [`Object`]: ../enum.Value.html#variant.Object
pub const CONTAINER: u8 = 0b_111;
