use bencher::*;
use htmlentity::*;

const SMALL_DIRTY: &str = "<a href=\"http://example.com/\">link</a> & [link]";
const SMALL_CLEAN: &str = ".a href=.http://example.com/..link./a. . [link]";
const BIG_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
const BIG_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

macro_rules! bench_func {
    ($name:ident, $func:ident, $sample:expr) => {
        fn $name(bench: &mut Bencher) {
            let sample = $sample;
            bench.iter(|| { $func(sample) });
            bench.bytes = sample.len() as u64;
        }
    }
}

bench_func!(encode_text_small_dirty, encode_text, SMALL_DIRTY);
bench_func!(encode_text_small_clean, encode_text, SMALL_CLEAN);
bench_func!(encode_text_big_dirty, encode_text, BIG_DIRTY);
bench_func!(encode_text_big_clean, encode_text, BIG_CLEAN);

bench_func!(encode_text_small_dirty_old, old_encode_text, SMALL_DIRTY);
bench_func!(encode_text_small_clean_old, old_encode_text, SMALL_CLEAN);
bench_func!(encode_text_big_dirty_old, old_encode_text, BIG_DIRTY);
bench_func!(encode_text_big_clean_old, old_encode_text, BIG_CLEAN);

benchmark_group!(benches,
    encode_text_small_dirty, encode_text_small_clean,
    encode_text_big_dirty, encode_text_big_clean,
    encode_text_small_dirty_old, encode_text_small_clean_old,
    encode_text_big_dirty_old, encode_text_big_clean_old);
benchmark_main!(benches);
