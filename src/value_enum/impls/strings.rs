// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for strings

use {
    alloc::{
        borrow::Cow,
        string::{String, ToString},
    },
    core::convert::TryFrom,

    crate::{Error, Result, Value},
};

/// # Shortcuts for strings
impl Value {

    /// # If the value is a [`Text`](#variant.Text), returns an immutable reference of it
    ///
    /// Returns an error if the value is not a [`Text`](#variant.Text).
    pub fn as_text(&self) -> Result<&str> {
        match self {
            Value::Text(s) => Ok(s),
            _ => Err(err!("Value is not a Text")),
        }
    }

    /// # If the value is a [`DateTime`](#variant.DateTime), returns an immutable reference of it
    ///
    /// Returns an error if the value is not a [`DateTime`](#variant.DateTime).
    pub fn as_date_time(&self) -> Result<&str> {
        match self {
            Value::DateTime(s) => Ok(s),
            _ => Err(err!("Value is not a DateTime")),
        }
    }

    /// # If the value is a [`Date`](#variant.Date), returns an immutable reference of it
    ///
    /// Returns an error if the value is not a [`Date`](#variant.Date).
    pub fn as_date(&self) -> Result<&str> {
        match self {
            Value::Date(s) => Ok(s),
            _ => Err(err!("Value is not a Date")),
        }
    }

    /// # If the value is a [`Time`](#variant.Time), returns an immutable reference of it
    ///
    /// Returns an error if the value is not a [`Time`](#variant.Time).
    pub fn as_time(&self) -> Result<&str> {
        match self {
            Value::Time(s) => Ok(s),
            _ => Err(err!("Value is not a Time")),
        }
    }

    /// # If the value is a [`DecimalStr`](#variant.DecimalStr), returns an immutable reference of it
    ///
    /// Returns an error if the value is not a [`DecimalStr`](#variant.DecimalStr).
    pub fn as_decimal_str(&self) -> Result<&str> {
        match self {
            Value::DecimalStr(s) => Ok(s),
            _ => Err(err!("Value is not a DecimalStr")),
        }
    }

}

/// # Converts input to a [`Text`](#variant.Text)
impl From<String> for Value {

    fn from(s: String) -> Self {
        Value::Text(s)
    }

}

/// # _Clones_ input and converts it to a [`Text`](#variant.Text)
impl From<&str> for Value {

    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }

}

impl From<Cow<'_, str>> for Value {

    fn from(s: Cow<str>) -> Self {
        Self::from(s.into_owned())
    }

}

impl TryFrom<Value> for String {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::Text(s) => Ok(s),
            _ => Err(err!("Value is not a Text")),
        }
    }

}
