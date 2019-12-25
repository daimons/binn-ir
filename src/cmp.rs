// License: see LICENSE file at root directory of `master` branch

//! # Helper for comparing things

use core::{
    cmp::Ordering,
    mem,
};

/// # Helps compare 2 things
pub trait CmpTo<T: Ord>: Ord {

    /// # Comapres to target
    fn cmp_to(&self, target: &T) -> Ordering;

}

impl CmpTo<u32> for i32 {

    fn cmp_to(&self, target: &u32) -> Ordering {
        #[allow(clippy::op_ref)]
        match self < &0 {
            true => Ordering::Less,
            false => (*self as u32).cmp(target),
        }
    }

}

macro_rules! impl_cmp_to_for_same_sign {
    ($($src: ty, $target: ty, $test_fn_name: tt,)+) => {
        $(
            #[test]
            #[allow(unused_comparisons)]
            fn $test_fn_name() {
                let min_src = <$src>::min_value();
                let min_target = <$target>::min_value();
                assert!((min_src < 0 && min_target < 0) || (min_src >= 0 && min_target >= 0));
            }

            impl CmpTo<$target> for $src {

                fn cmp_to(&self, target: &$target) -> Ordering {
                    match mem::size_of::<Self>() >= mem::size_of::<$target>() {
                        true => self.cmp(&(*target as Self)),
                        false => (*self as $target).cmp(target),
                    }
                }

            }
        )+
    }
}

impl_cmp_to_for_same_sign!(
    usize, u32, test_impl_of_usize_u32,
    u32, usize, test_impl_of_u32_usize,
);

macro_rules! impl_cmp_to_for_one_type {
    ($($ty: ty,)+) => {
        $(
            impl CmpTo<$ty> for $ty {

                fn cmp_to(&self, target: &$ty) -> Ordering {
                    self.cmp(target)
                }

            }
        )+
    }
}

impl_cmp_to_for_one_type!(usize, u32,);
