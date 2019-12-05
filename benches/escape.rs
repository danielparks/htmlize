#![feature(test)]

extern crate test;

use html_entities::*;
use test::Bencher;

const SMALL_DIRTY: &str = "<a href=\"http://example.com/\">link</a> & [link]";
const SMALL_CLEAN: &str = ".a href=.http://example.com/..link./a. . [link]";
const BIG_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
const BIG_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

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

bench_func!(escape_text_small_dirty, escape_text, SMALL_DIRTY);
bench_func!(escape_text_small_clean, escape_text, SMALL_CLEAN);
bench_func!(escape_text_big_dirty, escape_text, BIG_DIRTY);
bench_func!(escape_text_big_clean, escape_text, BIG_CLEAN);
