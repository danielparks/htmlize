use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use htmlize::*;
use std::convert::TryInto;
use std::time::Duration;

mod util;

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
        ("small_clean", util::inputs::SMALL_CLEAN),
        ("medium_clean", util::inputs::MEDIUM_CLEAN),
        ("big_clean", util::inputs::BIG_CLEAN),
        ("small_dirty", util::inputs::SMALL_DIRTY),
        ("medium_dirty", util::inputs::MEDIUM_DIRTY),
        ("big_dirty", util::inputs::BIG_DIRTY),
    ]
    .iter()
    .for_each(|(name, input)| {
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("escape_text", name),
            input,
            |b, input| b.iter(|| escape_text(&**input)),
        );
        group.bench_with_input(
            BenchmarkId::new("escape_all_quotes", name),
            input,
            |b, input| b.iter(|| escape_all_quotes(&**input)),
        );
    });

    group.finish();
}

criterion_group!(escape_group, benchmarks);
criterion_main!(escape_group);
