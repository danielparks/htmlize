#[macro_export]
macro_rules! bench_func {
    ($name:ident, $func:ident, $sample:expr) => {
        #[bench]
        fn $name(bench: &mut test::Bencher) {
            let sample = $sample;
            bench.iter(|| $func(sample));
            bench.bytes = sample.len() as u64;
        }
    };
}
