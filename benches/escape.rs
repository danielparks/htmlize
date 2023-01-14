use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use htmlize::*;
use std::convert::TryInto;
use std::time::Duration;

mod util;

macro_rules! bench {
    ( $group:expr, $function:ident, $size_name:expr, $input:expr ) => {{
        let input = $input;
        $group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        $group.bench_with_input(
            BenchmarkId::new(stringify!($function), $size_name),
            input,
            |b, input| b.iter(|| $function(&*input)),
        );
    }};
}

fn benchmarks(c: &mut Criterion) {
    let groups = [
        (
            "clean",
            [
                ("small", util::inputs::CLEAN_SMALL),
                ("medium", util::inputs::CLEAN_MEDIUM),
                ("big", util::inputs::CLEAN_BIG),
            ],
        ),
        (
            "dirty",
            [
                ("small", util::inputs::DIRTY_SMALL),
                ("medium", util::inputs::DIRTY_MEDIUM),
                ("big", util::inputs::DIRTY_BIG),
            ],
        ),
    ];

    for (group_name, inputs) in groups {
        let mut group = c.benchmark_group(group_name);
        group
            .significance_level(0.01)
            .confidence_level(0.99)
            .sample_size(500)
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(5));

        for (size_name, input) in inputs {
            bench!(group, escape_text, size_name, input);
            bench!(group, escape_all_quotes, size_name, input);
        }

        group.finish();
    }
}

criterion_group!(escape_group, benchmarks);
criterion_main!(escape_group);
