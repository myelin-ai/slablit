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

        $(
            temp_slab.insert($x);
        )*

        temp_slab
    }};
}

#[cfg(test)]
mod test {
    use super::slab;
    use slab::Slab;

    #[test]
    fn test_slab_macro() {
        let slab: Slab<usize> = slab![10, 20, 30];

        assert_eq!(
            vec![10, 20, 30],
            slab.into_iter()
                .map(|(_, value)| *value)
                .collect::<Vec<usize>>()
        );
    }

    #[test]
    fn test_slab_macro_with_trailing_comma() {
        let slab: Slab<usize> = slab![10, 20, 30, ];

        assert_eq!(
            vec![10, 20, 30],
            slab.into_iter()
                .map(|(_, value)| *value)
                .collect::<Vec<usize>>()
        );
    }
}
