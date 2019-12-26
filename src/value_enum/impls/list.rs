// License: see LICENSE file at root directory of `master` branch

//! # Shortcuts for `Value::List`

use {
    core::{
        convert::TryFrom,
        iter::FromIterator,
    },

    crate::{Error, List, Result, Value},
};

/// # Helper macro for Value::at()/mut_at()
macro_rules! at_or_mut_at { ($self: ident, $indexes: ident, $code: tt) => {{
    let mut value = Some($self);
    for (nth, idx) in $indexes.iter().enumerate() {
        match value {
            Some(Value::List(list)) => {
                value = list.$code(*idx);
                if nth + 1 == $indexes.len() {
                    return value.ok_or_else(|| Error::from(__!("Indexes are invalid: {:?}", $indexes)));
                }
            },
            Some(_) => return Err(Error::from(match nth {
                0 => __!("Value is not a List"),
                _ => __!("Value at {:?} is not a List", &$indexes[..nth]),
            })),
            None => return Err(Error::from(__!("There is no value at {:?}", &$indexes[..nth]))),
        };
    }

    Err(Error::from(__!("Indexes must not be empty")))
}}}

/// # Shortcuts for [`List`](#variant.List)
impl Value {

    /// # If the value is a list, pushes new item into it
    ///
    /// Returns an error if the value is not a list.
    pub fn push<T>(&mut self, value: T) -> Result<()> where T: Into<Self> {
        match self {
            Value::List(list) => Ok(crate::push(list, value)),
            _ => Err(Error::from(__!("Value is not a list"))),
        }
    }

    /// # Gets an immutable item from this list and its sub lists
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Indexes are empty or invalid.
    /// - The value or any of its sub items is not a list.
    ///
    /// ## Examples
    ///
    /// ```
    /// use binn_ir::Value;
    ///
    /// let mut list = binn_ir::list();
    /// list.push("first")?;
    /// list.push(vec![Value::False, "second".into()])?;
    ///
    /// assert_eq!(list.at(&[0])?.as_text()?, "first");
    /// assert_eq!(list.at(&[1, 1])?.as_text()?, "second");
    ///
    /// assert!(list.at(&[]).is_err());
    /// assert!(list.at(&[0, 1]).is_err());
    /// assert!(list.at(&[1, 2]).is_err());
    /// assert!(list.at(&[1, 0, 0]).is_err());
    /// assert!(list.at(&[1, 1, 2]).is_err());
    ///
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn at(&self, indexes: &[usize]) -> Result<&Self> {
        at_or_mut_at!(self, indexes, get)
    }

    /// # Gets a mutable item from this array and its sub arrays
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Indexes are empty or invalid.
    /// - The value or any of its sub items is not an array.
    pub fn mut_at(&mut self, indexes: &[usize]) -> Result<&mut Self> {
        at_or_mut_at!(self, indexes, get_mut)
    }

    /// # Takes an item from this list and its sub lists
    ///
    /// The function returns an error on one of these conditions:
    ///
    /// - Indexes are empty or invalid.
    /// - The value or any of its sub items is not a list.
    ///
    /// ## Examples
    ///
    /// ```
    /// use binn_ir::Value;
    ///
    /// let mut list = binn_ir::list();
    /// list.push("earth")?;
    /// list.push(vec![Value::False, "moon".into()])?;
    ///
    /// assert_eq!(list.take_at(&[0])?.as_text()?, "earth");
    /// assert_eq!(list.take_at(&[0, 1])?.as_text()?, "moon");
    ///
    /// assert!(list.take_at(&[]).is_err());
    /// assert!(list.take_at(&[0, 1]).is_err());
    ///
    /// # Ok::<_, binn_ir::Error>(())
    /// ```
    pub fn take_at(&mut self, indexes: &[usize]) -> Result<Self> {
        let mut value = Some(self);
        for (nth, idx) in indexes.iter().enumerate() {
            match value {
                Some(Value::List(list)) => match nth + 1 == indexes.len() {
                    true => return match idx >= &0 && idx < &list.len() {
                        true => Ok(list.remove(*idx)),
                        false => Err(Error::from(__!("Invalid indexes: {:?}", indexes))),
                    },
                    false => value = list.get_mut(*idx),
                },
                Some(_) => return Err(Error::from(match nth {
                    0 => __!("Value is not a List"),
                    _ => __!("Value at {:?} is not a List", &indexes[..nth]),
                })),
                None => return Err(Error::from(__!("There is no value at {:?}", &indexes[..nth]))),
            };
        }

        Err(Error::from(__!("Indexes must not be empty")))
    }

    /// # If the value is a list, returns an immutable reference of it
    ///
    /// Returns an error if the value is not a list.
    pub fn as_list(&self) -> Result<&List> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(Error::from(__!("Value is not a List"))),
        }
    }

    /// # If the value is a list, returns a mutable reference of it
    ///
    /// Returns an error if the value is not a list.
    pub fn as_mut_list(&mut self) -> Result<&mut List> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(Error::from(__!("Value is not a List"))),
        }
    }

}

impl From<List> for Value {

    fn from(list: List) -> Self {
        Value::List(list)
    }

}

impl FromIterator<Value> for Value {

    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=Self> {
        Value::List(iter.into_iter().collect())
    }

}

impl TryFrom<Value> for List {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::List(list) => Ok(list),
            _ => Err(Error::from(__!("Value is not a List"))),
        }
    }

}
