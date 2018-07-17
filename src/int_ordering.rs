// License: see LICENSE file at root directory of `master` branch

use std::cmp::Ordering;

/// # Unique universally identifier of this kit
#[allow(dead_code)]
pub const UUID: &'static str = "d895be5b-7831-4a1e-9ea3-53d1c315ab82";

/// # Version
#[allow(dead_code)]
pub const VERSION: &'static str = "0.0.3";

/// # Release date (year/month/day)
#[allow(dead_code)]
pub const RELEASE_DATE: (u16, u8, u8) = (2018, 7, 18);

/// # This trait helps compare 2 instances of `Ord`
///
/// All primitive integer types are implemented.
pub trait IntOrdering<T: Ord>: Ord {

    /// # Comapres to one other
    fn cmp_int(&self, other: &T) -> Ordering;

}

// ╔═══════════════════════╗
// ║   SIGNED - UNSIGNED   ║
// ╚═══════════════════════╝

macro_rules! impl_signed_unsigned {
    ($($signed: ty, $unsigned: ty, $as_unsigned: ty,)+) => {
        $(
            impl IntOrdering<$signed> for $unsigned {

                fn cmp_int(&self, other: &$signed) -> Ordering {
                    match *other >= 0 {
                        true => (*self as $as_unsigned).cmp(&(*other as $as_unsigned)),
                        false => Ordering::Greater,
                    }
                }

            }
            impl IntOrdering<$unsigned> for $signed {

                fn cmp_int(&self, other: &$unsigned) -> Ordering {
                    match *self >= 0 {
                        true => (*self as $as_unsigned).cmp(&(*other as $as_unsigned)),
                        false => Ordering::Less,
                    }
                }

            }
        )+
    }
}

impl_signed_unsigned! {
    i8, u8, u8,
    i8, u16, u16,
    i8, u32, u32,
    i8, u64, u64,
    i8, u128, u128,

    i16, u8, u16,
    i16, u16, u16,
    i16, u32, u32,
    i16, u64, u64,
    i16, u128, u128,

    i32, u8, u32,
    i32, u16, u32,
    i32, u32, u32,
    i32, u64, u64,
    i32, u128, u128,

    i64, u8, u64,
    i64, u16, u64,
    i64, u32, u64,
    i64, u64, u64,
    i64, u128, u128,

    i128, u8, u128,
    i128, u16, u128,
    i128, u32, u128,
    i128, u64, u128,
    i128, u128, u128,
}

impl_signed_unsigned! {
    isize, usize, usize,
}

#[cfg(target_pointer_width = "8")]
impl_signed_unsigned! {
    isize, u8, u8,
    isize, u16, u16,
    isize, u32, u32,
    isize, u64, u64,
    isize, u128, u128,

    i8, usize, u8,
    i16, usize, u16,
    i32, usize, u32,
    i64, usize, u64,
    i128, usize, u128,
}

#[cfg(target_pointer_width = "16")]
impl_signed_unsigned! {
    isize, u8, u16,
    isize, u16, u16,
    isize, u32, u32,
    isize, u64, u64,
    isize, u128, u128,

    i8, usize, u16,
    i16, usize, u16,
    i32, usize, u32,
    i64, usize, u64,
    i128, usize, u128,
}

#[cfg(target_pointer_width = "32")]
impl_signed_unsigned! {
    isize, u8, u32,
    isize, u16, u32,
    isize, u32, u32,
    isize, u64, u64,
    isize, u128, u128,

    i8, usize, u32,
    i16, usize, u32,
    i32, usize, u32,
    i64, usize, u64,
    i128, usize, u128,
}

#[cfg(target_pointer_width = "64")]
impl_signed_unsigned! {
    isize, u8, u64,
    isize, u16, u64,
    isize, u32, u64,
    isize, u64, u64,
    isize, u128, u128,

    i8, usize, u64,
    i16, usize, u64,
    i32, usize, u64,
    i64, usize, u64,
    i128, usize, u128,
}

#[cfg(target_pointer_width = "128")]
impl_signed_unsigned! {
    isize, u8, u128,
    isize, u16, u128,
    isize, u32, u128,
    isize, u64, u128,
    isize, u128, u128,

    i8, usize, u128,
    i16, usize, u128,
    i32, usize, u128,
    i64, usize, u128,
    i128, usize, u128,
}

#[cfg(not(any(
    target_pointer_width = "8", target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64",
    target_pointer_width = "128",
)))]
impl_signed_unsigned! {
    isize, u8, usize,
    isize, u16, usize,
    isize, u32, usize,
    isize, u64, usize,
    isize, u128, usize,

    i8, usize, usize,
    i16, usize, usize,
    i32, usize, usize,
    i64, usize, usize,
    i128, usize, usize,
}

// ╔═══════════════╗
// ║   SAME SIGN   ║
// ╚═══════════════╝

macro_rules! impl_same_sign {
    ($($a: ty, $b: ty, $as: ty,)+) => {
        $(
            impl IntOrdering<$b> for $a {

                fn cmp_int(&self, other: &$b) -> Ordering {
                    (*self as $as).cmp(&(*other as $as))
                }

            }
        )+
    }
}

macro_rules! impl_same_sign_both {
    ($($a: ty, $b: ty, $as: ty,)+) => {
        $(
            impl IntOrdering<$b> for $a {

                fn cmp_int(&self, other: &$b) -> Ordering {
                    (*self as $as).cmp(&(*other as $as))
                }

            }
            impl IntOrdering<$a> for $b {

                fn cmp_int(&self, other: &$a) -> Ordering {
                    (*self as $as).cmp(&(*other as $as))
                }

            }
        )+
    }
}

impl_same_sign! {
    i8, i8, i8,
    i16, i16, i16,
    i32, i32, i32,
    i64, i64, i64,
    i128, i128, i128,
    isize, isize, isize,

    u8, u8, u8,
    u16, u16, u16,
    u32, u32, u32,
    u64, u64, u64,
    u128, u128, u128,
    usize, usize, usize,
}

impl_same_sign_both! {
    i8, i16, i16,
    i8, i32, i32,
    i8, i64, i64,
    i8, i128, i128,

    i16, i32, i32,
    i16, i64, i64,
    i16, i128, i128,

    i32, i64, i64,
    i32, i128, i128,

    i64, i128, i128,
}

impl_same_sign_both! {
    u8, u16, u16,
    u8, u32, u32,
    u8, u64, u64,
    u8, u128, u128,

    u16, u32, u32,
    u16, u64, u64,
    u16, u128, u128,

    u32, u64, u64,
    u32, u128, u128,

    u64, u128, u128,
}

#[cfg(target_pointer_width = "8")]
impl_same_sign_both! {
    isize, i8, i8,
    isize, i16, i16,
    isize, i32, i32,
    isize, i64, i64,
    isize, i128, i128,

    usize, u8, u8,
    usize, u16, u16,
    usize, u32, u32,
    usize, u64, u64,
    usize, u128, u128,
}

#[cfg(target_pointer_width = "16")]
impl_same_sign_both! {
    isize, i8, i16,
    isize, i16, i16,
    isize, i32, i32,
    isize, i64, i64,
    isize, i128, i128,

    usize, u8, u16,
    usize, u16, u16,
    usize, u32, u32,
    usize, u64, u64,
    usize, u128, u128,
}

#[cfg(target_pointer_width = "32")]
impl_same_sign_both! {
    isize, i8, i32,
    isize, i16, i32,
    isize, i32, i32,
    isize, i64, i64,
    isize, i128, i128,

    usize, u8, u32,
    usize, u16, u32,
    usize, u32, u32,
    usize, u64, u64,
    usize, u128, u128,
}

#[cfg(target_pointer_width = "64")]
impl_same_sign_both! {
    isize, i8, i64,
    isize, i16, i64,
    isize, i32, i64,
    isize, i64, i64,
    isize, i128, i128,

    usize, u8, u64,
    usize, u16, u64,
    usize, u32, u64,
    usize, u64, u64,
    usize, u128, u128,
}

#[cfg(target_pointer_width = "128")]
impl_same_sign_both! {
    isize, i8, i128,
    isize, i16, i128,
    isize, i32, i128,
    isize, i64, i128,
    isize, i128, i128,

    usize, u8, u128,
    usize, u16, u128,
    usize, u32, u128,
    usize, u64, u128,
    usize, u128, u128,
}

#[cfg(not(any(
    target_pointer_width = "8", target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64",
    target_pointer_width = "128",
)))]
impl_same_sign_both! {
    isize, i8, isize,
    isize, i16, isize,
    isize, i32, isize,
    isize, i64, isize,
    isize, i128, isize,

    usize, u8, usize,
    usize, u16, usize,
    usize, u32, usize,
    usize, u64, usize,
    usize, u128, usize,
}

#[test]
fn test_ordering_greater() {
    macro_rules! greater { ($($v: expr,)+) => {{
        $(
            let v = $v;

            assert_eq!(Ordering::Greater, v.cmp_int(&-1_i8));
            assert_eq!(Ordering::Greater, v.cmp_int(&-1_i16));
            assert_eq!(Ordering::Greater, v.cmp_int(&-1_i32));
            assert_eq!(Ordering::Greater, v.cmp_int(&-1_i64));
            assert_eq!(Ordering::Greater, v.cmp_int(&-1_i128));
            assert_eq!(Ordering::Greater, v.cmp_int(&-1_isize));

            assert_eq!(Ordering::Greater, v.cmp_int(&-0_i8));
            assert_eq!(Ordering::Greater, v.cmp_int(&-0_i16));
            assert_eq!(Ordering::Greater, v.cmp_int(&-0_i32));
            assert_eq!(Ordering::Greater, v.cmp_int(&-0_i64));
            assert_eq!(Ordering::Greater, v.cmp_int(&-0_i128));
            assert_eq!(Ordering::Greater, v.cmp_int(&-0_isize));

            assert_eq!(Ordering::Greater, v.cmp_int(&0_u8));
            assert_eq!(Ordering::Greater, v.cmp_int(&0_u16));
            assert_eq!(Ordering::Greater, v.cmp_int(&0_u32));
            assert_eq!(Ordering::Greater, v.cmp_int(&0_u64));
            assert_eq!(Ordering::Greater, v.cmp_int(&0_u128));
            assert_eq!(Ordering::Greater, v.cmp_int(&0_usize));
        )+
    }};}

    greater!(1_i8, 1_i16, 1_i32, 1_i64, 1_i128, 1_isize, 1_u8, 1_u16, 1_u32, 1_u64, 1_u128, 1_usize,);
}

#[test]
fn test_ordering_equal() {
    macro_rules! equal_to_negative { ($($v: expr,)+) => {{
        $(
            let v = $v;
            assert_eq!(Ordering::Equal, v.cmp_int(&-1_i8));
            assert_eq!(Ordering::Equal, v.cmp_int(&-1_i16));
            assert_eq!(Ordering::Equal, v.cmp_int(&-1_i32));
            assert_eq!(Ordering::Equal, v.cmp_int(&-1_i64));
            assert_eq!(Ordering::Equal, v.cmp_int(&-1_i128));
            assert_eq!(Ordering::Equal, v.cmp_int(&-1_isize));
        )+
    }};}
    equal_to_negative!(-1_i8, -1_i16, -1_i32, -1_i64, -1_i128, -1_isize,);

    macro_rules! equal_to_zeros { ($($v: expr,)+) => {{
        $(
            let v = $v;

            assert_eq!(Ordering::Equal, v.cmp_int(&-0_i8));
            assert_eq!(Ordering::Equal, v.cmp_int(&-0_i16));
            assert_eq!(Ordering::Equal, v.cmp_int(&-0_i32));
            assert_eq!(Ordering::Equal, v.cmp_int(&-0_i64));
            assert_eq!(Ordering::Equal, v.cmp_int(&-0_i128));
            assert_eq!(Ordering::Equal, v.cmp_int(&-0_isize));

            assert_eq!(Ordering::Equal, v.cmp_int(&0_u8));
            assert_eq!(Ordering::Equal, v.cmp_int(&0_u16));
            assert_eq!(Ordering::Equal, v.cmp_int(&0_u32));
            assert_eq!(Ordering::Equal, v.cmp_int(&0_u64));
            assert_eq!(Ordering::Equal, v.cmp_int(&0_u128));
            assert_eq!(Ordering::Equal, v.cmp_int(&0_usize));
        )+
    }};}
    equal_to_zeros!(-0_i8, -0_i16, -0_i32, -0_i64, -0_i128, -0_isize, 0_u8, 0_u16, 0_u32, 0_u64, 0_u128, 0_usize,);

    macro_rules! equal_to_positive { ($($v: expr,)+) => {{
        $(
            let v = $v;

            assert_eq!(Ordering::Equal, v.cmp_int(&1_i8));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_i16));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_i32));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_i64));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_i128));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_isize));

            assert_eq!(Ordering::Equal, v.cmp_int(&1_u8));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_u16));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_u32));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_u64));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_u128));
            assert_eq!(Ordering::Equal, v.cmp_int(&1_usize));
        )+
    }};}
    equal_to_positive!(1_i8, 1_i16, 1_i32, 1_i64, 1_i128, 1_isize, 1_u8, 1_u16, 1_u32, 1_u64, 1_u128, 1_usize,);
}

#[test]
fn test_ordering_less() {
    macro_rules! less { ($($v: expr,)+) => {{
        $(
            let v = $v;

            assert_eq!(Ordering::Less, v.cmp_int(&-1_i8));
            assert_eq!(Ordering::Less, v.cmp_int(&-1_i16));
            assert_eq!(Ordering::Less, v.cmp_int(&-1_i32));
            assert_eq!(Ordering::Less, v.cmp_int(&-1_i64));
            assert_eq!(Ordering::Less, v.cmp_int(&-1_i128));
            assert_eq!(Ordering::Less, v.cmp_int(&-1_isize));

            assert_eq!(Ordering::Less, v.cmp_int(&-0_i8));
            assert_eq!(Ordering::Less, v.cmp_int(&-0_i16));
            assert_eq!(Ordering::Less, v.cmp_int(&-0_i32));
            assert_eq!(Ordering::Less, v.cmp_int(&-0_i64));
            assert_eq!(Ordering::Less, v.cmp_int(&-0_i128));
            assert_eq!(Ordering::Less, v.cmp_int(&-0_isize));

            assert_eq!(Ordering::Less, v.cmp_int(&0_u8));
            assert_eq!(Ordering::Less, v.cmp_int(&0_u16));
            assert_eq!(Ordering::Less, v.cmp_int(&0_u32));
            assert_eq!(Ordering::Less, v.cmp_int(&0_u64));
            assert_eq!(Ordering::Less, v.cmp_int(&0_u128));
            assert_eq!(Ordering::Less, v.cmp_int(&0_usize));
        )+
    }};}

    less!(-2_i8, -2_i16, -2_i32, -2_i64, -2_i128, -2_isize,);
}
