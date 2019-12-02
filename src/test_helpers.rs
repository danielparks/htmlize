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
