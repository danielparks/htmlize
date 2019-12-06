#[cfg(test)]

#[macro_export]
macro_rules! check {
    ($func:ident($($arg:expr),*) == $expected:expr) => {
        let actual = $func($($arg),*);
        if actual != *$expected {
            panic!(
                "check failed: {}\n  \
                  expected: {:?}\n  \
                  actual:   {:?}\n",
                stringify!($func($($arg),*)), $expected, actual);
        }
    }
}

#[macro_export]
macro_rules! test {
    ($name:ident, $func:ident($($arg:expr),*) == $expected:expr) => {
        #[test]
        fn $name() {
            check!($func($($arg),*) == $expected);
        }
    }
}

#[macro_export]
macro_rules! test_multiple {
    ($name:ident, $func:ident, $tests:expr) => {
        #[test]
        fn $name() {
            for (input, expected) in &$tests {
                check!($func(&input) == expected);
            }
        }
    }
}

#[macro_export]
macro_rules! test_eq {
    ($name:ident, $func:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            check!($func($input) == $expected);
        }
    }
}
