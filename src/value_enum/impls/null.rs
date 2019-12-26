// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for `Value::Null`

use {
    core::convert::TryFrom,

    crate::{Error, Result, Value},
};

/// # Shortcuts for [`Null`](#variant.Null)
impl Value {

    /// # Checks to see if this value is [`Null`][#Null]
    ///
    /// [#Null]: #variant.Null
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    /// # Tries to convert this value into something
    ///
    /// If this is [`Null`][#Null], returns `default`.
    ///
    /// If your default value would be a result of a function call, you should use [`try_into_or_else()`][try_into_or_else()].
    ///
    /// ## Examples
    ///
    /// ```
    /// assert!(binn_ir::Value::Null.try_into_or(true)?);
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    ///
    /// [#Null]: #variant.Null
    /// [try_into_or_else()]: #method.try_into_or_else
    pub fn try_into_or<T>(self, default: T) -> Result<T> where T: TryFrom<Self, Error=Error> {
        match self {
            Value::Null => Ok(default),
            _ => T::try_from(self),
        }
    }

    /// # Tries to convert this value into something
    ///
    /// If this is [`Null`][#Null], calls `default()` and returns its result.
    ///
    /// [#Null]: #variant.Null
    pub fn try_into_or_else<T, F>(self, default: F) -> Result<T> where T: TryFrom<Self, Error=Error>, F: FnOnce() -> T {
        match self {
            Value::Null => Ok(default()),
            _ => T::try_from(self),
        }
    }

    /// # Tries to convert a reference of this value into something
    ///
    /// If this is [`Null`][#Null], returns `default`.
    ///
    /// If your default value would be a result of a function call, you should use [`try_ref_into_or_else()`][try_ref_into_or_else()].
    ///
    /// ## Examples
    ///
    /// ```
    /// assert_eq!(binn_ir::Value::Null.try_ref_into_or(0)?, 0);
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    ///
    /// [#Null]: #variant.Null
    /// [try_ref_into_or_else()]: #method.try_ref_into_or_else
    pub fn try_ref_into_or<'a, T>(&'a self, default: T) -> Result<T> where T: TryFrom<&'a Self, Error=Error> {
        match self {
            Value::Null => Ok(default),
            _ => T::try_from(self),
        }
    }

    /// # Tries to convert a reference of this value into something
    ///
    /// If this is [`Null`][#Null], calls `default()` and returns its result.
    ///
    /// [#Null]: #variant.Null
    pub fn try_ref_into_or_else<'a, T, F>(&'a self, default: F) -> Result<T> where T: TryFrom<&'a Self, Error=Error>, F: FnOnce() -> T {
        match self {
            Value::Null => Ok(default()),
            _ => T::try_from(self),
        }
    }

}
