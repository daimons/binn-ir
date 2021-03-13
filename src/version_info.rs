// License: see LICENSE file at root directory of `master` branch

//! # `0.15.0` _(March 14th, 2021)_
//!
//! ## Changes
//!
//! Re-designed [`Error`][struct:Error]:
//!
//! - Added new functions:
//!
//!     ```ignore
//!     pub const fn line(&self) -> u32;
//!     pub const fn module_path(&self) -> &str;
//!     ```
//!
//! - Changed functions:
//!
//!     ```ignore
//!     pub fn msg(&self) -> &str;
//!     ```
//!
//!     into:
//!
//!     ```ignore
//!     pub fn msg(&self) -> Option<&str>;
//!     ```
//!
//! - Removed implementation: ~~`From<String>`~~.
//!
//! [struct:Error]: ../struct.Error.html
//!
//! <!-- 6c7646a9-b83c08ea-293526a9-07b9e036-771f8426-e7b616d7-b4c80325-7c945520-63b0d381-cc74fde9-9b3a28d4-6fa6c0d9-7473c4c2-ff18119d-3a308dd1-9452dfff -->
//!
//! ## Dependencies
//!
//! | Name | Version | Normal | Development | Build |
//! | ---- | ------- | :----: | :---------: | :---: |
//! | [`kib`][crate:kib] | `^4` |  | ✅ |  |
//!
//! [crate:kib]: https://bitbucket.org/haibison/kib
//!
//! <!-- 6c7646a9-b83c08ea-293526a9-07b9e036-771f8426-e7b616d7-b4c80325-7c945520-63b0d381-cc74fde9-9b3a28d4-6fa6c0d9-7473c4c2-ff18119d-3a308dd1-9452dfff:END -->
