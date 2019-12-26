// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for `Value::Blob`

use {
    crate::{Error, Result, Value},
};

/// # Shortcuts for [`Blob`](#variant.Blob)
impl Value {

    /// # If the value is a blob, returns an immutable reference of it
    ///
    /// Returns an error if the value is not a blob.
    pub fn as_blob(&self) -> Result<&[u8]> {
        match self {
            Value::Blob(blob) => Ok(blob),
            _ => Err(Error::from(__!("Value is not a Blob"))),
        }
    }

}
