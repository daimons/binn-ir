// License: see LICENSE file at root directory of `master` branch

//! # Implementations

mod list;

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

mod map;
