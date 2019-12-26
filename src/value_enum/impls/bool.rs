// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for booleans

use {
    crate::{Error, Result, Value},
};

/// # Shortcuts for booleans
impl Value {

    /// # If the value is a boolean, returns it
    ///
    /// Returns an error if the value is not a boolean.
    ///
    /// ## Examples
    ///
    /// ```
    /// use binn_ir::Value;
    ///
    /// assert!(Value::True.is_true()?);
    /// assert!(Value::False.is_true()? == false);
    /// assert!(Value::Null.is_true().is_err());
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn is_true(&self) -> Result<bool> {
        match self {
            Value::True => Ok(true),
            Value::False => Ok(false),
            _ => Err(Error::from(__!("Value is not a boolean"))),
        }
    }

}
