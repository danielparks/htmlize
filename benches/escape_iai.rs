use htmlize::*;
use iai::black_box;
use paste::paste;

macro_rules! iai_benchmarks {
    ( $( ($name:ident, $input:expr), )+ ) => {
        paste! {
            $(
                fn [<iai_escape_text_ $name>]() -> String {
                    escape_text(black_box($input))
                }
            )+

            iai::main!(
                $(
                    [<iai_escape_text_ $name>],
                )+
            );
        }
    }
}

iai_benchmarks! {
    (small_clean, ".a href=.http://example.com/..link./a. . [link]"),
    (big_clean, include_str!("../tests/corpus/html-cleaned.txt")),
    (small_dirty, "<a href=\"http://example.com/\">link</a> & [link]"),
    (big_dirty, include_str!("../tests/corpus/html-raw.txt")),
}
