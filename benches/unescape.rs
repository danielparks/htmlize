#![feature(test)]

extern crate test;

use html_entities::*;
use test::Bencher;

macro_rules! bench_func {
    ($name:ident, $func:ident, $sample:expr) => {
        #[bench]
        fn $name(bench: &mut Bencher) {
            let sample = $sample;
            bench.iter(|| { $func(sample) });
            bench.bytes = sample.len() as u64;
        }
    }
}

bench_func!(unescape_none, unescape, "sdfasfdasfsdf");
bench_func!(unescape_single, unescape, "&amp;");
bench_func!(unescape_single_prefix, unescape, "sdfasfdasfsdf&amp;");
