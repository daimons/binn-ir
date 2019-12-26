// License: see LICENSE file at root directory of `master` branch

//! # Types

use {
    alloc::{
        collections::BTreeMap,
        string::String,
        vec::Vec,
    },

    crate::Value,
};

/// # Size
pub type Size = u32;

/// # Blob
pub type Blob = Vec<u8>;

/// # List
pub type List = Vec<Value>;

/// # Map
pub type Map = BTreeMap<i32, Value>;

/// # Object
pub type Object = BTreeMap<String, Value>;
