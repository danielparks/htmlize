use htmlize::*;
use iai::black_box;
use paste::paste;

macro_rules! iai_benchmarks {
    ( $( ($name:ident, $input:expr), )+ ) => {
        paste! {
            $(
                fn [<iai_unescape_ $name>]() -> String {
                    unescape(black_box($input))
                }

                fn [<iai_unescape_attribute_ $name>]() -> String {
                    unescape_attribute(black_box($input))
                }
            )+

            iai::main!(
                $(
                    [<iai_unescape_ $name>],
                    [<iai_unescape_attribute_ $name>],
                )+
            );
        }
    }
}

// FIXME: we’re benchmarking making the sample too.
fn make_sample(count: usize, entity: &str, padding: &str) -> String {
    let mut s = padding.repeat(count);
    s.extend(entity.chars());
    s.repeat(count)
}

iai_benchmarks! {
    (none, "sdfasfdasfsdf"),
    (single_prefix, "sdfasfdasfsdf&amp;"),
    (long_invalid, "&abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd"),
    (all_entities, include_str!("../tests/corpus/all-entities-source.txt")),
    (html_document, include_str!("../tests/corpus/html-escaped.txt")),
    (sample_32_bare, make_sample(32, "&lt", "a")),
    (sample_32, make_sample(32, "&lt;", "a")),
    (sample_64_bare, make_sample(64, "&lt", "a")),
    (sample_64, make_sample(64, "&lt;", "a")),
    (sample_128_bare, make_sample(128, "&lt", "a")),
    (sample_128, make_sample(128, "&lt;", "a")),
}
