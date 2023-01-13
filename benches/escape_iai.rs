use htmlize::*;
use iai::black_box;
use paste::paste;

mod util;

macro_rules! iai_benchmarks {
    ( $( ($name:ident, $input:expr), )+ ) => {
        paste! {
            $(
                fn [<iai_escape_text_ $name>]() -> String {
                    escape_text(black_box($input))
                }
            )+

            $(
                fn [<iai_escape_all_quotes_ $name>]() -> String {
                    escape_all_quotes(black_box($input))
                }
            )+

            iai::main!(
                $(
                    [<iai_escape_text_ $name>],
                )+
                $(
                    [<iai_escape_all_quotes_ $name>],
                )+
            );
        }
    }
}

iai_benchmarks! {
    (small_clean, util::inputs::SMALL_CLEAN),
    (medium_clean, util::inputs::MEDIUM_CLEAN),
    (big_clean, util::inputs::BIG_CLEAN),
    (small_dirty, util::inputs::SMALL_DIRTY),
    (medium_dirty, util::inputs::MEDIUM_DIRTY),
    (big_dirty, util::inputs::BIG_DIRTY),
}
