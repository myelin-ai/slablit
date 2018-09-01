#![feature(decl_macro, macro_at_most_once_rep)]

#[allow(unused_macros)]
macro replace_expr($_t:tt $sub:expr) {
    $sub
}

#[allow(unused_macros)]
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
    fn test_slab_macro() {
        let (slab, [first, second, third]) = slab![10, 20, 30];

        assert_eq!(Some(&10), slab.get(first));
        assert_eq!(Some(&20), slab.get(second));
        assert_eq!(Some(&30), slab.get(third));
    }

    #[test]
    fn test_slab_macro_with_trailing_comma() {
        let (slab, [first, second, third]) = slab![10, 20, 30,];

        assert_eq!(Some(&10), slab.get(first));
        assert_eq!(Some(&20), slab.get(second));
        assert_eq!(Some(&30), slab.get(third));
    }
}
