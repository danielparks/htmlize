#![feature(test)]

extern crate test;

use htmlize::*;

#[macro_use]
mod helpers;

bench_func!(unescape_none, unescape, "sdfasfdasfsdf");
bench_func!(unescape_single, unescape, "&amp;");
bench_func!(unescape_single_prefix, unescape, "sdfasfdasfsdf&amp;");
bench_func!(
    unescape_long_invalid,
    unescape,
    "&abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd"
);

const ALL_SOURCE: &str =
    include_str!("../tests/corpus/all-entities-source.txt");
bench_func!(unescape_all_entities, unescape, ALL_SOURCE);

const HTML_ESCAPED: &str = include_str!("../tests/corpus/html-escaped.txt");
bench_func!(unescape_html_document, unescape, HTML_ESCAPED);

fn make_sample(count: usize, entity: &str, padding: &str) -> String {
    let mut s = padding.repeat(count);
    s.extend(entity.chars());
    s.repeat(count)
}

// ..._s is for semicolon. Closing the entity helps performance.
bench_func!(unescape_128_lt, unescape, make_sample(128, "&lt", "a"));
bench_func!(unescape_128_lt_s, unescape, make_sample(128, "&lt;", "a"));
bench_func!(unescape_64_lt, unescape, make_sample(64, "&lt", "a"));
bench_func!(unescape_64_lt_s, unescape, make_sample(64, "&lt;", "a"));
bench_func!(unescape_32_lt, unescape, make_sample(32, "&lt", "a"));
bench_func!(unescape_32_lt_s, unescape, make_sample(32, "&lt;", "a"));
