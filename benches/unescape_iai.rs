use htmlize::*;
use iai::black_box;
use paste::paste;

mod util;

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
iai_benchmarks! {
    (sample_128, util::inputs::make_sample(128, "&lt;", "a")),
    (sample_128_bare, util::inputs::make_sample(128, "&lta", "a")),
    (sample_128_none, util::inputs::make_sample(128, "_lta", "a")),
    (sample_128_invalid, util::inputs::make_sample(128, "&xxa", "a")),
}
