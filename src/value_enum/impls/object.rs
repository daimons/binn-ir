// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for `Value::Object`

use {
    core::{
        convert::TryFrom,
        iter::FromIterator,
    },

    crate::{Error, Object, ObjectKey, Result, Value},
};

/// # Helper macro for Value::*_maybe_by()/*_maybe_mut_by()
macro_rules! maybe_by_or_mut_by { ($self: ident, $variant: tt, $keys: ident, $code: tt) => {{
    if $keys.is_empty() {
        return Err(err!("Keys must not be empty"));
    }

    let mut value = Some($self);
    for (nth, key) in $keys.iter().enumerate() {
        match value {
            Some(Value::$variant(variant)) => value = variant.$code(*key),
            Some(_) => return Err(match nth {
                0 => err!("Value is not {}", stringify!($variant)),
                _ => err!("Value at {keys:?} is not {variant}", keys=&$keys[..nth], variant=stringify!($variant)),
            }),
            None => return Err(err!("There is no value at {:?}", &$keys[..nth])),
        };
    }

    Ok(value)
}}}

/// # Helper macro for Value::*_take_by()
macro_rules! maybe_take_by { ($self: ident, $variant: tt, $keys: ident) => {{
    let mut value = Some($self);
    for (nth, key) in $keys.iter().enumerate() {
        match value {
            Some(Value::$variant(variant)) => match nth + 1 == $keys.len() {
                true => return Ok(variant.remove(*key)),
                false => value = variant.get_mut(*key),
            },
            Some(_) => return Err(match nth {
                0 => err!("Value is not {}", stringify!($variant)),
                _ => err!("Value at {keys:?} is not {variant}", keys=&$keys[..nth], variant=stringify!($variant)),
            }),
            None => return Err(err!("There is no value at {:?}", &$keys[..nth])),
        };
    }

    Err(err!("Keys must not be empty"))
}}}

/// # Shortcuts for [`Object`](#variant.Object)
impl Value {

    /// # If the value is an object, inserts new item into it
    ///
    /// On success, returns previous value (if it existed).
    ///
    /// Returns an error if the value is not an object.
    pub fn object_insert<K, V>(&mut self, key: K, value: V) -> Result<Option<Value>> where K: Into<ObjectKey>, V: Into<Self> {
        match self {
            Value::Object(object) => Ok(crate::object_insert(object, key, value)),
            _ => Err(err!("Value is not an object")),
        }
    }

    /// # Gets an immutable item from this object and its sub objects
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not an object.
    ///
    /// ## Examples
    ///
    /// ```
    /// use core::convert::TryFrom;
    ///
    /// let mut object = binn_ir::object();
    /// object.object_insert("zero", true)?;
    /// object.object_insert("one", {
    ///     let mut object = binn_ir::Object::new();
    ///     binn_ir::object_insert(&mut object, "two", 99);
    ///     object
    /// })?;
    ///
    /// assert_eq!(bool::try_from(object.object_by(&["zero"])?)?, true);
    /// assert_eq!(u8::try_from(object.object_by(&["one", "two"])?)?, 99);
    ///
    /// assert!(object.object_by(&["two"]).is_err());
    /// assert!(object.object_maybe_by(&["two"])?.is_none());
    ///
    /// assert!(object.object_by(&[]).is_err());
    /// assert!(object.object_by(&["zero", "two"]).is_err());
    /// assert!(object.object_by(&["one", "two", "three"]).is_err());
    ///
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn object_by(&self, keys: &[&str]) -> Result<&Self> {
        self.object_maybe_by(keys)?.ok_or_else(|| err!("There is no value at: {:?}", keys))
    }

    /// # Gets an optional immutable item from this object and its sub objects
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not an object.
    pub fn object_maybe_by(&self, keys: &[&str]) -> Result<Option<&Self>> {
        maybe_by_or_mut_by!(self, Object, keys, get)
    }

    /// # Gets a mutable item from this object and its sub objects
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not an object.
    pub fn object_mut_by(&mut self, keys: &[&str]) -> Result<&mut Self> {
        self.object_maybe_mut_by(keys)?.ok_or_else(|| err!("There is no value at: {:?}", keys))
    }

    /// # Gets an optional mutable item from this object and its sub objects
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not an object.
    pub fn object_maybe_mut_by(&mut self, keys: &[&str]) -> Result<Option<&mut Self>> {
        maybe_by_or_mut_by!(self, Object, keys, get_mut)
    }

    /// # Takes an item from this object and its sub objects
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not an object.
    ///
    /// ## Examples
    ///
    /// ```
    /// let mut object = binn_ir::object();
    /// object.object_insert("first", "1st")?;
    /// object.object_insert("second", {
    ///     let mut object = binn_ir::Object::new();
    ///     binn_ir::object_insert(&mut object, "third", "3rd");
    ///     object
    /// })?;
    ///
    /// assert_eq!(object.object_take_by(&["first"])?.as_text()?, "1st");
    /// assert_eq!(object.object_take_by(&["second", "third"])?.as_text()?, "3rd");
    ///
    /// assert!(object.object_take_by(&["zero"]).is_err());
    /// assert!(object.object_maybe_take_by(&["zero"])?.is_none());
    /// assert!(object.object_maybe_take_by(&["second", "fourth"])?.is_none());
    ///
    /// assert!(object.object_take_by(&[]).is_err());
    /// assert!(object.object_take_by(&["third", "fourth"]).is_err());
    ///
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn object_take_by(&mut self, keys: &[&str]) -> Result<Self> {
        self.object_maybe_take_by(keys)?.ok_or_else(|| err!("There is no value at: {:?}", keys))
    }

    /// # Takes an optional item from this object and its sub objects
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not an object.
    pub fn object_maybe_take_by(&mut self, keys: &[&str]) -> Result<Option<Self>> {
        maybe_take_by!(self, Object, keys)
    }

    /// # If the value is an object, returns an immutable reference of it
    ///
    /// Returns an error if the value is not an object.
    pub fn as_object(&self) -> Result<&Object> {
        match self {
            Value::Object(object) => Ok(object),
            _ => Err(err!("Value is not an Object")),
        }
    }

    /// # If the value is an object, returns a mutable reference of it
    ///
    /// Returns an error if the value is not an object.
    pub fn as_mut_object(&mut self) -> Result<&mut Object> {
        match self {
            Value::Object(object) => Ok(object),
            _ => Err(err!("Value is not an Object")),
        }
    }

}

impl From<Object> for Value {

    fn from(object: Object) -> Self {
        Value::Object(object)
    }

}

impl FromIterator<(ObjectKey, Value)> for Value {

    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=(ObjectKey, Self)> {
        Value::Object(iter.into_iter().collect())
    }

}

impl TryFrom<Value> for Object {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::Object(object) => Ok(object),
            _ => Err(err!("Value is not an Object")),
        }
    }

}
