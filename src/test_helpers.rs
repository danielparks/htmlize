#[cfg(test)]

#[macro_export]
macro_rules! test_multiple {
    ($name:ident, $func:ident, $tests:expr) => {
        #[test]
        fn $name() {
            for (input, expected) in &$tests {
                let actual = $func(&input);
                assert_eq!(actual, *expected,
                    "{}({:?}) == {:?}",
                    stringify!($func), input, expected);
            }
        }
    }
}

#[macro_export]
macro_rules! test_eq {
    ($name:ident, $func:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let actual = $func($input);
            if actual != $expected {
                panic!("failed: {}({:?})\n  expected: {:?}\n  actual:   {:?}\n",
                    stringify!($func), $input, $expected, actual);
            }
        }
    }
}
