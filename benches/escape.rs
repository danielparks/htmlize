use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use htmlize::*;
use std::convert::TryInto;
use std::time::Duration;

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("escape");
    group
        .noise_threshold(0.10)
        .significance_level(0.01)
        .confidence_level(0.99)
        .sample_size(500)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5));

    [
        (
            "small_clean",
            ".a href=.http://example.com/..link./a. . [link]",
        ),
        (
            "big_clean",
            include_str!("../tests/corpus/html-cleaned.txt"),
        ),
        (
            "small_dirty",
            "<a href=\"http://example.com/\">link</a> & [link]",
        ),
        ("big_dirty", include_str!("../tests/corpus/html-raw.txt")),
    ]
    .iter()
    .for_each(|(name, input)| {
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("escape_text", name),
            input,
            |b, input| b.iter(|| escape_text(&**input)),
        );
    });

    group.finish();
}

criterion_group!(escape_group, benchmarks);
criterion_main!(escape_group);
