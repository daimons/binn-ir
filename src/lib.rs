// License: see LICENSE file at root directory of `master` branch

//! # An implementation of Binn - <https://github.com/liteserver/binn>
//!
//! ## Project
//!
//! - Repository: <https://bitbucket.org/haibison/binn-ir>
//! - License: Nice License 1.0.0 (see LICENSE file at root directory of `master` branch)
//! - _This project follows [Semantic Versioning 2.0.0]_
//!
//! ---
//!
//! ## Features
//!
//! - All official types are supported.
//! - User defined types are _not_ yet supported.
//!
//! ## Notes
//!
//! - `IR` stands for _implementation in Rust_.
//! - Documentation is built with all features. Some of them are optional. If you see components from other crates, you can view source to see
//!   what features are required.
//!
//! ## Security notes
//!
//! To prevent attacks when decoding from [`Read`][std::io/Read], consider limiting your source via [`Read::take()`][std::io/Read#take()].
//!
//! ## Quick examples
//!
//! This example demonstrates a simple file container:
//!
//! ```
//! use core::convert::TryFrom;
//! # #[cfg(feature="std")]
//! use binn_ir::{Blob, Decoder, Encoder, Map, Value};
//!
//! const MAGIC_NUMBER: u64 = 0xABCD;
//!
//! # #[cfg(feature="std")]
//! # fn test() -> binn_ir::IoResult<()> {
//! // Buffer
//! let mut buf: Vec<u8> = vec![];
//!
//! // Magic number
//! buf.encode_u64(MAGIC_NUMBER)?;
//!
//! // A single file header contains: name and hash
//! let mut file_header = binn_ir::map();
//! file_header.map_insert(0, "the-sun")?;  // name
//! file_header.map_insert(1, 0_u64)?;      // hash
//!
//! let file_content = Value::Blob(b"is hot".to_vec());
//!
//! // Encode data
//! file_header.encode(&mut buf)?;
//! file_content.encode(&mut buf)?;
//!
//! // Now decode data back
//! let mut cursor = std::io::Cursor::new(buf);
//! assert_eq!(cursor.decode_u64()?, Some(MAGIC_NUMBER));
//! assert_eq!(cursor.decode_map()?, Some(Map::try_from(file_header)?));
//! assert_eq!(cursor.decode_blob()?, Some(Blob::try_from(file_content)?));
//! assert_eq!(cursor.decode()?, None);
//! # Ok(()) }
//! # #[cfg(feature="std")]
//! # test().unwrap();
//! # Ok::<_, binn_ir::Error>(())
//! ```
//!
//! ## Thanks
//!
//! Special thanks to [Binn]'s authors for their hard work.
//!
//! Thank you,
//!
//! [Semantic Versioning 2.0.0]: https://semver.org/spec/v2.0.0.html
//! [Binn]: https://github.com/liteserver/binn
//!
//! [std::io/Read]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [std::io/Read#take()]: https://doc.rust-lang.org/std/io/trait.Read.html#method.take

#![warn(missing_docs)]
#![no_std]
#![deny(unsafe_code)]

// ?????????????????????????????????????????????????????????
// ???   IDENTIFIERS   ???
// ?????????????????????????????????????????????????????????

macro_rules! code_name  { () => { "binn-ir" }}
macro_rules! version    { () => { "0.15.0" }}

/// # Crate name
///
/// IR stands for _implementation in Rust_.
pub const NAME: &str = "Binn-IR";

/// # Crate code name
pub const CODE_NAME: &str = code_name!();

/// # ID of this crate
pub const ID: &str = concat!(
    "2f0f7b89-c460bfcf-9910298a-8bd68231-7ca09fc3-389c7e9a-15966f7b-81ea0014-",
    "58f3bd4b-517ea3b5-dbe67356-61440866-5c1034b0-2abb189b-efadbb32-1b2a48d8",
);

/// # Crate version
pub const VERSION: &str = version!();

/// # Crate release date (year/month/day)
pub const RELEASE_DATE: (u16, u8, u8) = (2021, 3, 14);

/// # Tag, which can be used for logging...
pub const TAG: &str = concat!(code_name!(), "::2f0f7b89::", version!());

// ??????????????????????????????????????????????????????????????????
// ???   IMPLEMENTATION   ???
// ??????????????????????????????????????????????????????????????????

extern crate alloc;

#[cfg(feature="std")]
extern crate std;

/// # Makes new Error with formatted string, or without one
macro_rules! err {
    () => {
        crate::Error::new(line!(), module_path!(), None)
    };
    ($s: literal) => {
        crate::Error::new(line!(), module_path!(), Some(alloc::borrow::Cow::Borrowed($s)))
    };
    ($s: literal, $($arg: tt)+) => {
        crate::Error::new(line!(), module_path!(), Some(alloc::borrow::Cow::Owned(alloc::format!($s, $($arg)+))))
    };
}

#[test]
fn test_macro_err() {
    use alloc::borrow::Cow;

    macro_rules! s_test { () => { "test" }}

    fn eq(first: Error, second: Error) -> bool {
        first.line() == second.line() && first.module_path() == second.module_path() && first.msg() == second.msg()
    }

    assert!(eq(err!(), Error::new(line!(), module_path!(), None)));
    assert!(eq(err!("test"), Error::new(line!(), module_path!(), Some(Cow::Borrowed(s_test!())))));
    assert!(eq(err!("{s:?}", s=s_test!()), Error::new(line!(), module_path!(), Some(Cow::Owned(alloc::format!("{:?}", s_test!()))))));
}

/// # Wrapper for format!(), which prefixes your optional message with: crate::TAG, module_path!(), line!()
macro_rules! __ {
    ($($arg: tt)+) => {
        alloc::format!(
            "[{tag}][{module_path}-{line}] {msg}", tag=crate::TAG, module_path=module_path!(), line=line!(), msg=alloc::format!($($arg)+),
        )
    };
    () => {
        alloc::format!("[{tag}][{module_path}-{line}] (internal error)", tag=crate::TAG, module_path=module_path!(), line=line!())
    };
}

mod cmp;
mod container_functions;
mod error;
mod types;
mod value_enum;

#[cfg(feature="std")]
mod decoder;
#[cfg(feature="std")]
mod decoding_functions;
#[cfg(feature="std")]
mod encoder;
#[cfg(feature="std")]
mod encoding_functions;

pub use self::{
    container_functions::*,
    error::*,
    types::*,
    value_enum::*,
};

#[cfg(feature="std")]
pub use self::{
    decoder::*,
    decoding_functions::*,
    encoder::*,
    encoding_functions::*,
};

pub mod specification;
pub mod storage;
pub mod value;
pub mod version_info;

/// # Result type used in this crate
pub type Result<T> = core::result::Result<T, Error>;

/// # Result for I/O functions
#[cfg(feature="std")]
pub type IoResult<T> = core::result::Result<T, std::io::Error>;

#[test]
fn test_crate_version() {
    assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
}
