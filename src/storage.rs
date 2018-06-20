// License: see LICENSE file at root directory of `master` branch

//! # Storages

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
