#![feature(test)]

extern crate test;

use html_entities::bench_func;
use html_entities::matcher::m;

bench_func!(bench_matcher_abc, m, "abc");
bench_func!(bench_matcher_aab, m, "aab");
bench_func!(bench_matcher_aaa, m, "aaa");
bench_func!(bench_matcher_abaaa, m, "abaaa");
bench_func!(bench_matcher_aaaa, m, "aaaa");
bench_func!(bench_matcher_baaa, m, "baaa");
bench_func!(bench_matcher_bcaaa, m, "bcaaa");
bench_func!(bench_matcher_bcaaaa, m, "bcaaaa");
bench_func!(bench_matcher_bcaaaab, m, "bcaaaab");
bench_func!(bench_matcher_baaaaaab, m, "baaaaaab");
bench_func!(bench_matcher_baaasaaab, m, "baaasaaab");
bench_func!(bench_matcher_abbc, m, "abbc");
bench_func!(bench_matcher_aabb, m, "aabb");
