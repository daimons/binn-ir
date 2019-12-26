// License: see LICENSE file at root directory of `master` branch

//! # Numbers

use {
    core::convert::TryFrom,

    crate::{Error, Value},
};

macro_rules! impl_from_numbers_for_value { ($($number: ty, $variant: tt,)+) => {
    $(
        impl From<$number> for Value {

            fn from(n: $number) -> Self {
                Value::$variant(n)
            }

        }
    )+
}}

impl_from_numbers_for_value!{
    i8, I8, i16, I16, i32, I32, i64, I64,
    u8, U8, u16, U16, u32, U32, u64, U64,
    f32, Float, f64, Double,
}

macro_rules! impl_try_from_value_for_integers { ($($ty: ty,)+) => {
    $(
        impl TryFrom<&Value> for $ty {

            type Error = Error;

            fn try_from(v: &Value) -> core::result::Result<Self, Self::Error> {
                match v {
                    Value::I8(i) => Self::try_from(*i).map_err(|e| Error::from(__!("{}", e))),
                    Value::U8(u) => Self::try_from(*u).map_err(|e| Error::from(__!("{}", e))),
                    Value::I16(i) => Self::try_from(*i).map_err(|e| Error::from(__!("{}", e))),
                    Value::U16(u) => Self::try_from(*u).map_err(|e| Error::from(__!("{}", e))),
                    Value::I32(i) => Self::try_from(*i).map_err(|e| Error::from(__!("{}", e))),
                    Value::U32(u) => Self::try_from(*u).map_err(|e| Error::from(__!("{}", e))),
                    Value::I64(i) => Self::try_from(*i).map_err(|e| Error::from(__!("{}", e))),
                    Value::U64(u) => Self::try_from(*u).map_err(|e| Error::from(__!("{}", e))),
                    _ => Err(Error::from(__!("Value is not an integer"))),
                }
            }

        }

        impl TryFrom<Value> for $ty {

            type Error = Error;

            fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
                Self::try_from(&v)
            }

        }
    )+
}}

impl_try_from_value_for_integers! {
    i8, i16, i32, i64,
    u8, u16, u32, u64,
}

impl TryFrom<&Value> for f32 {

    type Error = Error;

    fn try_from(v: &Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::I8(i) => Ok(Self::from(*i)),
            Value::U8(u) => Ok(Self::from(*u)),
            Value::I16(i) => Ok(Self::from(*i)),
            Value::U16(u) => Ok(Self::from(*u)),
            Value::Float(f) => Ok(*f),
            _ => Err(Error::from(__!("Cannot convert this value to f32"))),
        }
    }

}

impl TryFrom<Value> for f32 {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        Self::try_from(&v)
    }

}

impl TryFrom<&Value> for f64 {

    type Error = Error;

    fn try_from(v: &Value) -> core::result::Result<Self, Self::Error> {
        match v {
            Value::I8(i) => Ok(Self::from(*i)),
            Value::U8(u) => Ok(Self::from(*u)),
            Value::I16(i) => Ok(Self::from(*i)),
            Value::U16(u) => Ok(Self::from(*u)),
            Value::I32(i) => Ok(Self::from(*i)),
            Value::U32(u) => Ok(Self::from(*u)),
            Value::Float(f) => Ok(Self::from(*f)),
            Value::Double(d) => Ok(*d),
            _ => Err(Error::from(__!("Cannot convert this value to f64"))),
        }
    }

}

impl TryFrom<Value> for f64 {

    type Error = Error;

    fn try_from(v: Value) -> core::result::Result<Self, Self::Error> {
        Self::try_from(&v)
    }

}
