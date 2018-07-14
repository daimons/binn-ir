// License: see LICENSE file at root directory of `master` branch

//! # An implementation of <https://github.com/liteserver/binn>
//!
//! # Project
//!
//! - Repository: <https://bitbucket.org/haibison/binn-ir>
//! - License: [Free Public License 1.0.0](https://opensource.org/licenses/FPL-1.0.0)
//! - _This project follows [Semantic Versioning 2.0.0]_
//!
//! ---
//!
//! # Features
//!
//! - All official types are supported.
//! - User defined types are _not_ yet supported.
//!
//! # Notes
//!
//! - `#![no_std]` _might_ be supported when [`alloc`][crate:alloc] crate is stabilized.
//! - `IR` stands for _implementation in Rust_.
//! - Core encoder and decoder are almost done (except decoder for user defined types). However API might change, as the project is still in
//!   early development phase.
//!
//! # Quick examples
//!
//! Most functionalities are provided via 2 traits: [`Encoder`][trait:value::Encoder], [`Decoder`][trait:value::Decoder].
//!
//! This example demonstrates a simple file container:
//!
//! ```
//! extern crate binn_ir;
//!
//! use std::collections::HashMap;
//! use std::io::Cursor;
//!
//! use binn_ir::value::{Value, Encoder, Decoder};
//!
//! const MAGIC_NUMBER: u64 = 0xABCD;
//!
//! // Buffer
//! let mut buf: Vec<u8> = vec![];
//!
//! // Magic number
//! buf.encode_u64(MAGIC_NUMBER).unwrap();
//!
//! // A single file header contains: name and hash
//! let file_header = {
//!     let mut map = HashMap::new();
//!     map.insert(String::from("name"), Value::from("sun"));
//!     map.insert(String::from("hash"), Value::U64(0));
//!     map
//! };
//! let file_content = "is hot";
//! buf.encode_object(file_header.clone());
//! buf.encode_blob(file_content.as_bytes());
//!
//! // Now read data back
//! let mut cursor = Cursor::new(&buf);
//! assert_eq!(cursor.decode_u64().unwrap(), MAGIC_NUMBER);
//! assert_eq!(cursor.decode_object().unwrap(), file_header);
//! assert_eq!(cursor.decode_blob().unwrap(), file_content.as_bytes());
//! assert_eq!(cursor.position(), buf.len() as u64);
//! ```
//!
//! ---
//!
//! _Special thanks to [Binn]'s authors for their hard work._
//!
//! [Semantic Versioning 2.0.0]: https://semver.org/spec/v2.0.0.html
//! [Binn]: https://github.com/liteserver/binn
//! [crate:alloc]: https://doc.rust-lang.org/alloc/
//! [trait:value::Encoder]: value/trait.Encoder.html
//! [trait:value::Decoder]: value/trait.Decoder.html

// TODO: enable this flag when `alloc` crate is stabilized
// #![no_std]

// ╔═════════════════╗
// ║   IDENTIFIERS   ║
// ╚═════════════════╝

macro_rules! crate_code_name    { () => { "binn-ir" }}
macro_rules! crate_version      { () => { "0.3.0" }}

/// # Crate name
///
/// IR stands for _implementation in Rust_.
pub const CRATE_NAME: &'static str = "Binn-IR";

/// # Crate code name
pub const CRATE_CODE_NAME: &'static str = crate_code_name!();

/// # Crate version
pub const CRATE_VERSION: &'static str = crate_version!();

/// # Crate release date (year/month/day)
pub const CRATE_RELEASE_DATE: (u16, u8, u8) = (2018, 7, 13);

/// # Unique universally identifier of this crate. Its CRC-32 is `149dc8a5`.
pub const UUID: &'static str = "acea8479-f233-4686-af1c-fe198f506ddb";

/// # Tag, which can be used for logging...
pub const TAG: &'static str = concat!(crate_code_name!(), "_149dc8a5_", crate_version!());

// ╔════════════════════╗
// ║   IMPLEMENTATION   ║
// ╚════════════════════╝

#[test]
fn test_crate_version() {
    assert_eq!(CRATE_VERSION, env!("CARGO_PKG_VERSION"));
}

#[macro_use]
mod cmp_integers;

pub mod storage;
pub mod value;
