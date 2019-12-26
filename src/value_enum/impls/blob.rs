// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for `Value::Blob`

use {
    core::convert::TryFrom,

    crate::{Blob, Error, Result, Value},
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

    /// # If the value is a blob, returns a mutable reference of it
    ///
    /// Returns an error if the value is not a blob.
    pub fn as_mut_blob(&mut self) -> Result<&mut Blob> {
        match self {
            Value::Blob(blob) => Ok(blob),
            _ => Err(Error::from(__!("Value is not a Blob"))),
        }
    }

}

impl From<Blob> for Value {

    fn from(v: Blob) -> Self {
        Value::Blob(v)
    }

}

impl TryFrom<Value> for Blob {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::Blob(blob) => Ok(blob),
            _ => Err(Error::from(__!("Value is not a Blob"))),
        }
    }

}
