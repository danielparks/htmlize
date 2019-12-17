#[macro_export]
macro_rules! test_multiple {
    ($name:ident, $func:ident, $tests:expr) => {
        #[test]
        fn $name() {
            for (input, expected) in &$tests {
                ::assertify::assertify!($func(&input) == *expected);
            }
        }
    }
}
