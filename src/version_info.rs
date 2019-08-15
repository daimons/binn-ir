// License: see LICENSE file at root directory of `master` branch

//! # `0.13.1` _(August 16th, 2019)_
//!
//! ## Changes
//!
//! - Used `dyn` for trait objects.
//! - Trait bounds: replaced `fn(impl X)` with `fn<T>(T) where T: X`.
//! - Added new type:
//!
//!     ```ignore
//!     pub type Result<T> = std::result::Result<T, std::io::Error>;
//!     ```
//!
//! <!-- 6c7646a9-b83c08ea-293526a9-07b9e036-771f8426-e7b616d7-b4c80325-7c945520-63b0d381-cc74fde9-9b3a28d4-6fa6c0d9-7473c4c2-ff18119d-3a308dd1-9452dfff -->
//!
//! ## Dependencies
//!
//! No dependencies.
//!
//! ## Development dependencies
//!
//! No dependencies.
//!
//! ## Build dependencies
//!
//! No dependencies.
