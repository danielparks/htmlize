/// Generates code from entities.json when feature “unescape“ is enabled.
///
/// The canonical source is https://html.spec.whatwg.org/entities.json (see
/// https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references).
///
/// The entities.json file looks like:
///
///     {
///         "&AElig": { "codepoints": [198], "characters": "\u00C6" },
///         . . .
///     }
#[cfg(feature = "unescape")]
use std::{
    cmp::{max, min},
    env, fs,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    #[cfg(feature = "unescape")]
    generate_entities_rs();
}

#[cfg(feature = "unescape")]
fn generate_entities_rs() {
    let entities = load_entities("entities.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("entities.rs");
    let mut out = BufWriter::new(File::create(out_path).unwrap());

    macro_rules! w {
        ($msg:literal $(, $args:expr)*) => {
            writeln!(out, $msg $(, $args)*).unwrap();
        }
    }

    w!("/// A map of all valid HTML entities to their expansions (requires `unescape`");
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
    for (name, glyph) in &entities {
        let codepoints: Vec<String> = glyph
            .chars()
            .map(|c| format!("U+{:06X}", u32::from(c)))
            .collect();

        // `{:28}` would pad the output inside the backticks.
        let name = format!("`{}`", name);

        // Suppress a few inconvenient glyphs. Newline adds an extra line, and
        // tab causes a clippy warning.
        let glyph = match glyph.as_str() {
            "\n" | "\t" => "",
            v => v,
        };

        w!("/// {:30} | {:18} | {}", name, codepoints.join(", "), glyph);
    }

    let mut map_builder = phf_codegen::Map::<&[u8]>::new();
    let mut max_len: usize = 0;
    let mut min_len: usize = usize::max_value();
    for (name, value) in &entities {
        map_builder.entry(name.as_bytes(), &format!("&{:?}", value.as_bytes()));
        max_len = max(max_len, name.len());
        min_len = min(min_len, name.len());
    }

    w!(
        "pub static ENTITIES: phf::Map<&[u8], &[u8]> = {};",
        map_builder.build()
    );

    w!("");
    w!("/// Length of longest entity including & and possibly ; (requires `unescape`");
    w!("/// feature)");
    w!("pub const ENTITY_MAX_LENGTH: usize = {};", max_len);

    w!("");
    w!("/// Length of shortest entity including & and possibly ; (requires `unescape`");
    w!("/// feature)");
    w!("pub const ENTITY_MIN_LENGTH: usize = {};", min_len);
}

#[cfg(feature = "unescape")]
fn load_entities<P: AsRef<Path>>(path: P) -> Vec<(String, String)> {
    let input = fs::read(path.as_ref()).unwrap();
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
