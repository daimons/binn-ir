// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for `Value::Map`

use {
    core::{
        convert::TryFrom,
        iter::FromIterator,
    },

    crate::{Error, Map, MapKey, Result, Value},
};

/// # Helper macro for Value::*_maybe_by()/*_maybe_mut_by()
macro_rules! maybe_by_or_mut_by { ($self: ident, $variant: tt, $keys: ident, $code: tt) => {{
    if $keys.is_empty() {
        return Err(Error::from(__!("Keys must not be empty")));
    }

    let mut value = Some($self);
    for (nth, key) in $keys.iter().enumerate() {
        match value {
            Some(Value::$variant(variant)) => value = variant.$code(key),
            Some(_) => return Err(Error::from(match nth {
                0 => __!("Value is not {}", stringify!($variant)),
                _ => __!("Value at {keys:?} is not {variant}", keys=&$keys[..nth], variant=stringify!($variant)),
            })),
            None => return Err(Error::from(__!("There is no value at {:?}", &$keys[..nth]))),
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
                true => return Ok(variant.remove(key)),
                false => value = variant.get_mut(key),
            },
            Some(_) => return Err(Error::from(match nth {
                0 => __!("Value is not {}", stringify!($variant)),
                _ => __!("Value at {keys:?} is not {variant}", keys=&$keys[..nth], variant=stringify!($variant)),
            })),
            None => return Err(Error::from(__!("There is no value at {:?}", &$keys[..nth]))),
        };
    }

    Err(Error::from(__!("Keys must not be empty")))
}}}

/// # Shortcuts for [`Map`](#variant.Map)
impl Value {

    /// # If the value is a map, inserts new item into it
    ///
    /// On success, returns previous value (if it existed).
    ///
    /// Returns an error if the value is not a map.
    pub fn map_insert<V>(&mut self, key: MapKey, value: V) -> Result<Option<Self>> where V: Into<Self> {
        match self {
            Value::Map(map) => Ok(crate::map_insert(map, key, value)),
            _ => Err(Error::from(__!("Value is not a map"))),
        }
    }

    /// # Gets an immutable item from this map and its sub maps
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not a map.
    ///
    /// ## Examples
    ///
    /// ```
    /// use core::convert::TryFrom;
    ///
    /// let mut map = binn_ir::map();
    /// map.map_insert(0, true)?;
    /// map.map_insert(1, {
    ///     let mut map = binn_ir::Map::new();
    ///     binn_ir::map_insert(&mut map, 2, 99);
    ///     map
    /// })?;
    ///
    /// assert_eq!(bool::try_from(map.map_by(&[0])?)?, true);
    /// assert_eq!(u8::try_from(map.map_by(&[1, 2])?)?, 99);
    ///
    /// assert!(map.map_by(&[2]).is_err());
    /// assert!(map.map_maybe_by(&[2])?.is_none());
    ///
    /// assert!(map.map_by(&[]).is_err());
    /// assert!(map.map_by(&[0, 2]).is_err());
    /// assert!(map.map_by(&[1, 2, 3]).is_err());
    ///
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn map_by(&self, keys: &[MapKey]) -> Result<&Self> {
        self.map_maybe_by(keys)?.ok_or_else(|| Error::from(__!("There is no value at: {:?}", keys)))
    }

    /// # Gets an optional immutable item from this map and its sub maps
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not a map.
    pub fn map_maybe_by(&self, keys: &[MapKey]) -> Result<Option<&Self>> {
        maybe_by_or_mut_by!(self, Map, keys, get)
    }

    /// # Gets a mutable item from this map and its sub maps
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not a map.
    pub fn map_mut_by(&mut self, keys: &[MapKey]) -> Result<&mut Self> {
        self.map_maybe_mut_by(keys)?.ok_or_else(|| Error::from(__!("There is no value at: {:?}", keys)))
    }

    /// # Gets an optional mutable item from this map and its sub maps
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not a map.
    pub fn map_maybe_mut_by(&mut self, keys: &[MapKey]) -> Result<Option<&mut Self>> {
        maybe_by_or_mut_by!(self, Map, keys, get_mut)
    }

    /// # Takes an item from this map and its sub maps
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not a map.
    ///
    /// ## Examples
    ///
    /// ```
    /// let mut map = binn_ir::map();
    /// map.map_insert(0, "zero")?;
    /// map.map_insert(1, {
    ///     let mut map = binn_ir::Map::new();
    ///     binn_ir::map_insert(&mut map, 2, "two");
    ///     map
    /// })?;
    ///
    /// assert_eq!(map.map_take_by(&[0])?.as_text()?, "zero");
    /// assert_eq!(map.map_take_by(&[1, 2])?.as_text()?, "two");
    ///
    /// assert!(map.map_take_by(&[0]).is_err());
    /// assert!(map.map_maybe_take_by(&[0])?.is_none());
    /// assert!(map.map_maybe_take_by(&[1, 2])?.is_none());
    ///
    /// assert!(map.map_take_by(&[]).is_err());
    /// assert!(map.map_take_by(&[3, 4]).is_err());
    ///
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn map_take_by(&mut self, keys: &[MapKey]) -> Result<Self> {
        self.map_maybe_take_by(keys)?.ok_or_else(|| Error::from(__!("There is no value at: {:?}", keys)))
    }

    /// # Takes an optional item from this map and its sub maps
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Keys are empty.
    /// - The value or any of its sub items is not a map.
    pub fn map_maybe_take_by(&mut self, keys: &[MapKey]) -> Result<Option<Self>> {
        maybe_take_by!(self, Map, keys)
    }

    /// # If the value is a map, returns an immutable reference of it
    ///
    /// Returns an error if the value is not a map.
    pub fn as_map(&self) -> Result<&Map> {
        match self {
            Value::Map(map) => Ok(map),
            _ => Err(Error::from(__!("Value is not a Map"))),
        }
    }

    /// # If the value is a map, returns a mutable reference of it
    ///
    /// Returns an error if the value is not a map.
    pub fn as_mut_map(&mut self) -> Result<&mut Map> {
        match self {
            Value::Map(map) => Ok(map),
            _ => Err(Error::from(__!("Value is not a Map"))),
        }
    }

}

impl From<Map> for Value {

    fn from(map: Map) -> Self {
        Value::Map(map)
    }

}

impl FromIterator<(MapKey, Value)> for Value {

    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=(MapKey, Self)> {
        Value::Map(iter.into_iter().collect())
    }

}

impl TryFrom<Value> for Map {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::Map(map) => Ok(map),
            _ => Err(Error::from(__!("Value is not a Map"))),
        }
    }

}
