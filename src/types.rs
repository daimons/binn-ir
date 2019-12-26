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
pub type Map = BTreeMap<MapKey, Value>;

/// # Map key
pub type MapKey = i32;

/// # Object
pub type Object = BTreeMap<ObjectKey, Value>;

/// # Object key
pub type ObjectKey = String;
