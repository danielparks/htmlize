#![feature(test)]

extern crate test;

use html_entities::*;

const SMALL_DIRTY: &str = "<a href=\"http://example.com/\">link</a> & [link]";
const SMALL_CLEAN: &str = ".a href=.http://example.com/..link./a. . [link]";
const BIG_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
const BIG_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

bench_func!(escape_text_small_dirty, escape_text, SMALL_DIRTY);
bench_func!(escape_text_small_clean, escape_text, SMALL_CLEAN);
bench_func!(escape_text_big_dirty, escape_text, BIG_DIRTY);
bench_func!(escape_text_big_clean, escape_text, BIG_CLEAN);
