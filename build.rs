// Generates code from entities.json depending on enabled features.
//
// The canonical source is https://html.spec.whatwg.org/entities.json (see
// https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references).
//
// The entities.json file looks like:
//
//     {
//         "&AElig": { "codepoints": [198], "characters": "\u00C6" },
//         . . .
//     }

fn main() {
    #[cfg(any(feature = "unescape_fast", feature = "entities"))]
    let entities = load_entities("entities.json");

    #[cfg(feature = "unescape_fast")]
    generate_matcher_rs(&entities);

    #[cfg(feature = "entities")]
    generate_entities_rs(&entities);
}

#[cfg(feature = "entities")]
fn generate_entities_rs(entities: &[(String, String)]) {
    use std::cmp::{max, min};
    use std::env;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("entities.rs");
    let mut out = BufWriter::new(File::create(out_path).unwrap());

    macro_rules! w {
        ($msg:literal $(, $args:expr)*) => {
            writeln!(out, $msg $(, $args)*).unwrap();
        }
    }

    w!("/// A map of all valid HTML entities to their expansions (requires `entities`");
    w!("/// feature).");
    w!("///");
    w!("/// The keys of the map are full entity byte strings, e.g. `b\"&copy;\"`, and the");
    w!("/// values are their expansions, e.g. `b\"©\"`.");
    w!("///");
    w!("/// See the [WHATWG HTML spec][spec] for the canonical list of entities with");
    w!("/// their codepoints and glyphs. The [entities.json][] file linked there is");
    w!("/// used to generate this constant.");
    w!("///");
    w!("/// [spec]: https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references");
    w!("/// [entities.json]: https://html.spec.whatwg.org/entities.json");
    w!("///");
    w!("/// Entity                         | Codepoints         | Glyph");
    w!("/// -------------------------------|--------------------|------");

    let mut map_builder = phf_codegen::Map::<&[u8]>::new();
    let mut max_len: usize = 0;
    let mut min_len: usize = usize::max_value();
    for (name, glyph) in entities {
        map_builder.entry(name.as_bytes(), &format!("&{:?}", glyph.as_bytes()));
        max_len = max(max_len, name.len());
        min_len = min(min_len, name.len());

        let codepoints: Vec<String> = glyph
            .chars()
            .map(|c| format!("U+{:06X}", u32::from(c)))
            .collect();

        // `{:28}` would pad the output inside the backticks.
        let name = format!("`{name}`");

        // Suppress a few inconvenient glyphs. Newline adds an extra line, and
        // tab causes a clippy warning.
        let glyph = match glyph.as_str() {
            "\n" | "\t" => "",
            v => v,
        };

        w!("/// {name:30} | {:18} | {glyph}", codepoints.join(", "));
    }

    w!(
        "pub static ENTITIES: phf::Map<&[u8], &[u8]> = {};",
        map_builder.build()
    );

    w!("");
    w!("/// Length of longest entity including ‘&’ and possibly ‘;’ (requires");
    w!("/// `entities` feature)");
    w!("pub const ENTITY_MAX_LENGTH: usize = {};", max_len);

    w!("");
    w!("/// Length of shortest entity including ‘&’ and possibly ‘;’ (requires");
    w!("/// `entities` feature)");
    w!("pub const ENTITY_MIN_LENGTH: usize = {};", min_len);
}

#[cfg(feature = "unescape_fast")]
fn generate_matcher_rs(entities: &[(String, String)]) {
    use iter_matcher::IterMatcher;
    use std::env;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("matcher.rs");
    let mut out = BufWriter::new(File::create(out_path).unwrap());

    writeln!(out, "/// Used in `match_entity()`.").unwrap();
    let mut matcher =
        IterMatcher::new("fn entity_matcher", "(bool, &'static [u8])");
    entities.iter().for_each(|(name, glyph)| {
        matcher.add(
            name.as_bytes(),
            format!("({:?}, &{:?})", name.ends_with(';'), glyph.as_bytes()),
        );
    });
    matcher.disable_clippy(true);
    matcher.render(&mut out).unwrap();
    writeln!(out).unwrap();
}

#[cfg(any(feature = "unescape_fast", feature = "entities"))]
fn load_entities<P: AsRef<std::path::Path>>(path: P) -> Vec<(String, String)> {
    let input = std::fs::read(path.as_ref()).unwrap();
    let input: serde_json::Map<String, serde_json::Value> =
        serde_json::from_slice(&input).unwrap();

    input
        .iter()
        .map(|(name, info)| {
            (
                name.to_owned(),
                info["characters"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}
