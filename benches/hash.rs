//! Benchmark hashing functions with [`criterion`].

#![allow(clippy::missing_docs_in_private_items, missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use htmlize::unescape::internal::{Expander, Hashify, PhfMap, QuickPhf};
use std::collections::HashMap;

// Include function to match entities at the start of an iterator. Used in
// `match_entity()`.
//
// fn entity_matcher<'a, I>(iter: &mut I) -> Option<(bool, &'static [u8])>
// where
//     I: Iterator<Item = &'a u8> + Clone,
include!(concat!(env!("OUT_DIR"), "/matcher.rs"));

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash");
    let input = &b"&timesbar;"[..];

    let mut map = HashMap::new();
    for (&key, &value) in &htmlize::ENTITIES {
        map.insert(key, value);
    }

    group.bench_with_input("hashmap", input, |b, input| {
        b.iter(|| map.get(&input));
    });

    group.bench_with_input("hashify", input, |b, input| {
        b.iter(|| Hashify::expand(input));
    });

    group.bench_with_input("phf", input, |b, input| {
        b.iter(|| PhfMap::expand(input));
    });

    group.bench_with_input("quickphf", input, |b, input| {
        b.iter(|| QuickPhf::expand(input));
    });

    group.bench_with_input("matchgen", input, |b, input| {
        b.iter(|| entity_matcher(input));
    });
    group.finish();
}

criterion_group!(hash_group, benchmarks);
criterion_main!(hash_group);
