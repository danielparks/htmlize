/// Generates code from entities.json
///
/// The canonical source for entities.json is
/// https://html.spec.whatwg.org/entities.json (see
/// https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references).
///
/// The entities.json file looks like:
///
/// {
///    "&AElig": { "codepoints": [198], "characters": "\u00C6" },
///    . . .
/// }
use std::cmp::{max, min};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let entities = load_entities("entities.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("entities.rs");
    let mut out = BufWriter::new(File::create(out_path).unwrap());

    macro_rules! w {
        ($msg:literal $(, $args:expr)*) => {
            writeln!(out, $msg $(, $args)*).unwrap();
        }
    }

    w!("/// A map of all valid HTML entities to their expansions.");
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
        let mut codepoints: Vec<String> = Vec::new();
        for c in glyph.to_string().chars() {
            let ord: u32 = c.into();
            codepoints.push(format!("U+{:06X}", ord));
        }

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
    w!("/// Length of longest entity including & and possibly ;");
    w!("pub const ENTITY_MAX_LENGTH: usize = {};", max_len);

    w!("");
    w!("/// Length of shortest entity including & and possibly ;");
    w!("pub const ENTITY_MIN_LENGTH: usize = {};", min_len);
}

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
