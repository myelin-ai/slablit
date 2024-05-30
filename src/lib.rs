//! This crate exposes a macro `slab` which creates a new [`Slab`].  
//! It returns a tuple of the slab and the created keys as a fixed size array.
//!
//! # Examples
//!
//! ## Basic
//!
//! ```
//! use slablit::slab;
//!
//! let (slab, [first_id, second_id, third_id]) = slab!["foo", "bar", "baz"];
//! ```
//!
//! ## With Trailing Comma
//!
//! ```
//! use slablit::slab;
//!
//! let (slab, _) = slab![
//!     "foo",
//!     "bar",
//!     "baz",
//! ];
//!
//! ```
//!
//! [`Slab`]: https://docs.rs/slab/latest/slab/struct.Slab.html
//!

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_count_tts {
    ($($tts:tt)*) => {<[()]>::len(&[$($crate::__internal_replace_expr!($tts ())),*])};
}

#[macro_export]
macro_rules! slab {
    ($( $x:expr ),* $(,)?) => {{
        const COUNT: usize = $crate::__internal_count_tts!($($x)*);
        #[allow(unused_mut)]
        let mut temp_slab = slab::Slab::with_capacity(COUNT);
        let keys: [usize; COUNT] = [$(temp_slab.insert($x), )*];

        (temp_slab, keys)
    }};
}

#[cfg(doctest)]
#[doc = include_str!("../readme.md")]
pub mod doctest_readme {}

#[cfg(test)]
mod test {
    use super::slab;

    #[test]
    fn slab_macro_can_create_empty_slab() {
        let (slab, []): (slab::Slab<i32>, _) = slab![];
        assert!(slab.is_empty());
    }

    #[test]
    fn slab_macro_works() {
        let (slab, [first, second, third]) = slab![10, 20, 30];

        assert_eq!(Some(&10), slab.get(first));
        assert_eq!(Some(&20), slab.get(second));
        assert_eq!(Some(&30), slab.get(third));
    }

    #[test]
    fn slab_macro_with_trailing_comma_works() {
        let (slab, [first, second, third]) = slab![10, 20, 30,];

        assert_eq!(Some(&10), slab.get(first));
        assert_eq!(Some(&20), slab.get(second));
        assert_eq!(Some(&30), slab.get(third));
    }
}
