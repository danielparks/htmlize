use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use htmlize::*;
use std::convert::TryInto;
use std::time::Duration;

fn make_sample(count: usize, entity: &str, padding: &str) -> String {
    let mut s = padding.repeat(count);
    s.extend(entity.chars());
    s.repeat(count)
}

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("unescape");
    group
        .noise_threshold(0.10)
        .significance_level(0.01)
        .confidence_level(0.99)
        .sample_size(300)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10));

    [
        ("none", "sdfasfdasfsdf"),
        ("single", "&amp;"),
        ("single_prefix", "sdfasfdasfsdf&amp;"),
        ("long_invalid", "&abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd"),
        ("all_entities", include_str!("../tests/corpus/all-entities-source.txt")),
        ("html_document", include_str!("../tests/corpus/html-escaped.txt")),
    ]
    .iter()
    .for_each(|(name, input)| {
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("unescape", name),
            input,
            |b, input| b.iter(|| unescape(&**input)),
        );
        group.bench_with_input(
            BenchmarkId::new("unescape_attribute", name),
            input,
            |b, input| b.iter(|| unescape_attribute(&**input)),
        );
    });

    for size in [32, 64, 128] {
        // Bare entity without semicolon; should have worse performance.
        let name = format!("sample_{size}_bare");
        let input = make_sample(size, "&lt", "a");
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("unescape", &name),
            &input,
            |b, input| b.iter(|| unescape(&**input)),
        );
        group.bench_with_input(
            BenchmarkId::new("unescape_attribute", &name),
            &input,
            |b, input| b.iter(|| unescape_attribute(&**input)),
        );

        let name = format!("sample_{size}");
        let input = make_sample(size, "&lt;", "a");
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("unescape", &name),
            &input,
            |b, input| b.iter(|| unescape(&**input)),
        );
        group.bench_with_input(
            BenchmarkId::new("unescape_attribute", &name),
            &input,
            |b, input| b.iter(|| unescape_attribute(&**input)),
        );
    }

    group.finish();
}

criterion_group!(unescape_group, benchmarks);
criterion_main!(unescape_group);
