pub const SMALL_CLEAN: &str = "Björk and Борис OBrien ❤️, “love beats hate”";
pub const MEDIUM_CLEAN: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.";
pub const BIG_CLEAN: &str = include_str!("../../tests/corpus/html-cleaned.txt");

pub const SMALL_DIRTY: &str = r#"Björk & Борис O'Brien <3, "love > hate""#;
pub const MEDIUM_DIRTY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa&aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa>";
pub const BIG_DIRTY: &str = include_str!("../../tests/corpus/html-raw.txt");
