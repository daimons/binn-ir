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
//! # TODO
//!
//! [Semantic Versioning 2.0.0]: https://semver.org/spec/v2.0.0.html
//! [Binn]: https://github.com/liteserver/binn

// TODO: enable this flag when `alloc` crate is stabilized
// #![no_std]

// ╔═════════════════╗
// ║   IDENTIFIERS   ║
// ╚═════════════════╝

macro_rules! crate_code_name    { () => { "binn-ir" }}
macro_rules! crate_version      { () => { "0.0.1" }}

/// # Crate name
///
/// IR stands for _implementation in Rust_.
pub const CRATE_NAME: &'static str = "Binn-IR";

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

#[macro_use]
mod tmp;

pub mod storage;
pub mod value;
