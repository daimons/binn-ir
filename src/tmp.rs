// License: see LICENSE file at root directory of `master` branch

//! Temporary kit

/// # Compares integers
///
/// - Version: 0.0.6 (July 7th, 2018)
// #[macro_export]
macro_rules! cmp_integers {
    ($a: expr, $b: expr) => {{
        match ::std::mem::size_of_val(&$a).max(::std::mem::size_of_val(&$b)) {
            1 => cmp_integers!($a, $b, u8, i8),
            2 => cmp_integers!($a, $b, u16, i16),
            4 => cmp_integers!($a, $b, u32, i32),
            8 => cmp_integers!($a, $b, u64, i64),
            16 => cmp_integers!($a, $b, u128, i128),
            _ => cmp_integers!($a, $b, usize, isize),
        }
    }};
    ($a: expr, $b: expr, $unsigned: ty, $signed: ty) => {{
        #[allow(unused_comparisons)]
        match ($a >= 0, $b >= 0) {
            (true, true) => ($a as $unsigned).cmp(&($b as $unsigned)),
            (true, false) => ::std::cmp::Ordering::Greater,
            (false, true) => ::std::cmp::Ordering::Less,
            (false, false) => ($a as $signed).cmp(&($b as $signed)),
        }
    }};
}

#[test]
fn test_cmp_integers() {
    assert!(::std::cmp::Ordering::Greater == cmp_integers!(111_u8, -1_i8));
    assert!(::std::cmp::Ordering::Greater == cmp_integers!(111_usize, -1_i8));
    assert!(::std::cmp::Ordering::Greater == cmp_integers!(::std::u128::MAX, -1_i8));

    assert!(::std::cmp::Ordering::Less == cmp_integers!(-1_i8, 2_u32));
    assert!(::std::cmp::Ordering::Less == cmp_integers!(-1_i8, 2_usize));

    assert!(::std::cmp::Ordering::Equal == cmp_integers!(-0_i32, 0));
    assert!(::std::cmp::Ordering::Equal == cmp_integers!(-0_i8, 0_u64));
    assert!(::std::cmp::Ordering::Equal == cmp_integers!(1_i8, 1_u64));
    assert!(::std::cmp::Ordering::Equal == cmp_integers!(1_i8, 1_usize));
    assert!(::std::cmp::Ordering::Equal == cmp_integers!(111_i64, 111_u8));
    assert!(::std::cmp::Ordering::Equal == cmp_integers!(-1_i8, -1_i32));
    assert!(::std::cmp::Ordering::Equal == cmp_integers!(-999_i16, -999_i64));
}
