#![feature(test)]

extern crate test;

use htmlize::*;

#[macro_use]
mod helpers;

bench_func!(unescape_none, unescape, "sdfasfdasfsdf");
bench_func!(unescape_single, unescape, "&amp;");
bench_func!(unescape_single_prefix, unescape, "sdfasfdasfsdf&amp;");
bench_func!(unescape_long, unescape,
    "&abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd");
