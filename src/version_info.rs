// License: see LICENSE file at root directory of `master` branch

//! # `0.14.0` _(December 27th, 2019)_
//!
//! ## Breaking Changes
//!
//! The whole project is refactored.
//!
//! Basicallly changes are:
//!
//! - [`Value`][::Value]:
//!
//!     + Moved from [`value`][mod:value] into root.
//!     + Renamed `len()` to [`size()`][::Value/size()].
//!     + [`Object`][::Value#Object]: changed from [`HashMap`][std::collections/HashMap] to [`BTreeMap`][alloc::collections/btree_map/BTreeMap].
//!     + Removed expensive implementations of [`From`][core::convert/From], which copied large collections (blob, map, object...)
//!     + Removed some other rarely used implementations.
//!     + Replaced [`Display`][core::fmt/Display] implementation with [`Debug`][core::fmt/Debug].
//!
//! - Moved traits [`Encoder`][::Encoder], [`Decoder`][::Decoder] from [`value`][mod:value] into root. Added more implementors for
//!   them:
//!
//!     + [`Encoder`][::Encoder]: implementors are now all instances of [`Write`][std::io/Write].
//!     + [`Decoder`][::Decoder]: implementors are now all instances of [`Read`][std::io/Read].
//!
//! - Moved encoding/decoding functions from [`value`][mod:value] to root.
//! - Added new optional feature: `std`.
//!
//! ...and some others.
//!
//! ## Changes
//!
//! - Added new type aliases: [`Size`][::Size], [`Blob`][::Blob], [`List`][::List], [`Map`][::Map], [`MapKey`][::MapKey], [`Object`][::Object],
//!   [`ObjectKey`][::ObjectKey], [`IoResult`][::IoResult]...
//! - Added [`Error`][::Error].
//! - Added basic support for `no-std`.
//! - Moved specification from root documentation into new module [`specification`][mod:specification].
//! - [`Value`][::Value]: added shortcuts for most variants: [booleans][::Value#shortcuts-for-booleans], [`Null`][::Value#shortcuts-for-null],
//!   [strings][::Value#shortcuts-for-strings], [`Blob`][::Value#shortcuts-for-blob], [`List`][::Value#shortcuts-for-list],
//!   [`Map`][::Value#shortcuts-for-map], [`Object`][::Value#shortcuts-for-object]...
//! - Optimized code.
//!
//! ...and some others.
//!
//! [::Blob]: ../type.Blob.html
//! [::Decoder]: ../trait.Decoder.html
//! [::Encoder]: ../trait.Encoder.html
//! [::Error]: ../struct.Error.html
//! [::IoResult]: ../type.IoResult.html
//! [::List]: ../type.List.html
//! [::Map]: ../type.Map.html
//! [::MapKey]: ../type.MapKey.html
//! [::Object]: ../type.Object.html
//! [::ObjectKey]: ../type.ObjectKey.html
//! [::Size]: ../type.Size.html
//! [::Value]: ../enum.Value.html
//! [::Value/size()]: ../enum.Value.html#method.size
//! [::Value#Object]: ../enum.Value.html#variant.Object
//! [::Value#shortcuts-for-blob]: ../enum.Value.html#shortcuts-for-blob
//! [::Value#shortcuts-for-booleans]: ../enum.Value.html#shortcuts-for-booleans
//! [::Value#shortcuts-for-list]: ../enum.Value.html#shortcuts-for-list
//! [::Value#shortcuts-for-map]: ../enum.Value.html#shortcuts-for-map
//! [::Value#shortcuts-for-null]: ../enum.Value.html#shortcuts-for-null
//! [::Value#shortcuts-for-object]: ../enum.Value.html#shortcuts-for-object
//! [::Value#shortcuts-for-strings]: ../enum.Value.html#shortcuts-for-strings
//!
//! [mod:specification]: ../specification/index.html
//! [mod:value]: ../value/index.html
//!
//! [alloc::collections/btree_map/BTreeMap]: https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html
//! [core::convert/From]: https://doc.rust-lang.org/core/convert/trait.From.html
//! [core::fmt/Debug]: https://doc.rust-lang.org/core/fmt/trait.Debug.html
//! [core::fmt/Display]: https://doc.rust-lang.org/core/fmt/trait.Display.html
//! [std::collections/HashMap]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
//! [std::io/Read]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [std::io/Write]: https://doc.rust-lang.org/std/io/trait.Write.html
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
