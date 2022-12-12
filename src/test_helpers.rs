#[cfg(test)]
#[macro_export]
macro_rules! test_multiple {
    ($name:ident, $func:ident, $tests:expr) => {
        #[test]
        fn $name() {
            for (input, expected) in &$tests {
                ::assert2::assert!($func(&input) == *expected);
            }
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! test {
    ($name:ident, $($test:tt)+) => {
        #[test]
        fn $name() {
            ::assert2::assert!($($test)+);
        }
    };
}
