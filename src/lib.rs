#![feature(decl_macro, macro_at_most_once_rep)]

//!
//! This crate exposes a macro `slab` which creates a new [`Slab`].  
//! It returns a tuple of the slab and the created keys as a fixed size array.
//!
//! # Examples
//!
//! ## Basic
//!
//! ```
//! #![feature(decl_macro, macro_at_most_once_rep)]
//! use slablit::slab;
//!
//! let (slab, [first_id, second_id, third_id]) = slab!["foo", "bar", "baz"];
//! ```
//!
//! ## With Trailing Comma
//!
//! ```
//! #![feature(decl_macro, macro_at_most_once_rep)]
//!
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

#[allow(unused_macros)]
#[doc(hidden)]
macro replace_expr($_t:tt $sub:expr) {
    $sub
}

#[allow(unused_macros)]
#[doc(hidden)]
macro count_tts {
    ($($tts:tt)*) => {<[()]>::len(&[$(crate::replace_expr!($tts ())),*])};
}

pub macro slab {
    ($( $x:expr ),* $(,)?) => {{
        let mut temp_slab = slab::Slab::with_capacity(crate::count_tts!($($x)*));
        let keys = [$(temp_slab.insert($x), )*];

        (temp_slab, keys)
    }};
}

#[cfg(test)]
mod test {
    use super::slab;

    #[test]
    fn slab_macro() {
        let (slab, [first, second, third]) = slab![10, 20, 30];

        assert_eq!(Some(&10), slab.get(first));
        assert_eq!(Some(&20), slab.get(second));
        assert_eq!(Some(&30), slab.get(third));
    }

    #[test]
    fn slab_macro_with_trailing_comma() {
        let (slab, [first, second, third]) = slab![10, 20, 30,];

        assert_eq!(Some(&10), slab.get(first));
        assert_eq!(Some(&20), slab.get(second));
        assert_eq!(Some(&30), slab.get(third));
    }
}
