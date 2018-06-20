// License: see LICENSE file at root directory of `master` branch

//! # An implementation of <https://github.com/liteserver/binn>
//!
//! # Project
//!
//! - Repository: <https://bitbucket.org/haibison/binnx>
//! - License: [Free Public License 1.0.0](https://opensource.org/licenses/FPL-1.0.0)
//! - _This project follows [Semantic Versioning 2.0.0]_
//!
//! ---
//!
//! # TODO
//!
//! [Semantic Versioning 2.0.0]: https://semver.org/spec/v2.0.0.html
//! [Binn]: https://github.com/liteserver/binn

#![no_std]

// ╔═════════════════╗
// ║   IDENTIFIERS   ║
// ╚═════════════════╝

macro_rules! crate_code_name    { () => { "binnx" }}
macro_rules! crate_version      { () => { "0.0.1" }}

/// # Crate name
pub const CRATE_NAME: &'static str = "Binn-X";

/// # Crate code name
pub const CRATE_CODE_NAME: &'static str = crate_code_name!();

/// # Crate version
pub const CRATE_VERSION: &'static str = crate_version!();

/// # Crate release date (year/month/day)
pub const CRATE_RELEASE_DATE: (u16, u8, u8) = (2018, 6, 20);

/// # Unique universally identifier of this crate. Its CRC-32 is `149dc8a5`.
pub const UUID: &'static str = "acea8479-f233-4686-af1c-fe198f506ddb";

/// # Tag, which can be used for logging...
pub const TAG: &'static str = concat!(crate_code_name!(), "_149dc8a5_", crate_version!());

// ╔════════════════════╗
// ║   IMPLEMENTATION   ║
// ╚════════════════════╝

/// # Storage
pub enum Storage<'a> {

    /// # No bytes
    NoBytes,

    /// # 1 byte
    Byte(u8),

    /// # Word (2 bytes)
    Word(u16),

    /// # DWord (4 bytes)
    DWord(u32),

    /// # QWord (8 bytes)
    QWord(u64),

    /// # String (UTF-8)
    String(&'a str),

    /// # Blob
    Blob(&'a [u8]),

    /// # Container
    Container,

}

impl<'a> Storage<'a> {

    /// # NOBYTES
    pub const NOBYTES: u8 = 0b000;

    /// # BYTE
    pub const BYTE: u8 = 0b001;

    /// # WORD
    pub const WORD: u8 = 0b010;

    /// # DWORD
    pub const DWORD: u8 = 0b011;

    /// # QWORD
    pub const QWORD: u8 = 0b100;

    /// # STRING
    pub const STRING: u8 = 0b101;

    /// # BLOB
    pub const BLOB: u8 = 0b110;

    /// # CONTAINER
    pub const CONTAINER: u8 = 0b111;

}
