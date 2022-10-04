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
use serde_json::{Map, Value};
use std::cmp::{max, min};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let entities = load_entities("entities.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("entities.rs");
    let mut out = File::create(&out_path).unwrap();

    macro_rules! w {
        ($msg:literal $(, $args:expr)*) => {
            writeln!(out, $msg $(, $args)*).unwrap();
        }
    }

    w!("use phf::phf_map;");
    w!("");
    w!(r#"/// All valid HTML entities and their expansions as `(b"&copy;", b"©")` tuples."#);
    w!("///");
    w!("/// See the [WHATWG HTML spec](https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references)");
    w!("/// the canonical list of entities with their codepoints and glyphs. The");
    w!("/// [entities.json](https://html.spec.whatwg.org/entities.json) file linked");
    w!("/// there is used to generate this constant.");
    w!("///");
    w!("/// Entity                         | Codepoints         | Glyph");
    w!("/// -------------------------------|--------------------|------");
    for (name, value) in &entities {
        let mut codepoints: Vec<String> = Vec::new();
        for c in value.to_string().chars() {
            let ord: u32 = c.into();
            codepoints.push(format!("U+{:06.X}", ord));
        }

        let name = format!("`{}`", name);

        // Suppress a few weird values. They wouldn’t actually hurt anything,
        // but newline adds an extra line, and tab causes a clippy warning.
        let value = match value.as_str() {
            "\n" | "\t" => "",
            v => v,
        };

        w!("/// {:30} | {:18} | {}", name, codepoints.join(", "), value);
    }

    w!(
        "{}",
        "pub static ENTITIES: phf::Map<&[u8], &[u8]> = phf_map! {"
    );

    let mut max_len: usize = 0;
    let mut min_len: usize = usize::max_value();
    for (name, value) in &entities {
        max_len = max(max_len, name.len());
        min_len = min(min_len, name.len());

        w!("    b{:?} => &{:?}, // {}", name, value.as_bytes(), value);
    }

    w!("{}", "};");

    w!("");
    w!("/// Length of longest entity including & and possibly ;");
    w!("pub const ENTITY_MAX_LENGTH: usize = {};", max_len);

    w!("");
    w!("/// Length of shortest entity including & and possibly ;");
    w!("pub const ENTITY_MIN_LENGTH: usize = {};", min_len);
}

pub fn load_entities<P: AsRef<Path>>(path: P) -> Vec<(String, String)> {
    let path = path.as_ref();
    let input = fs::read(path).unwrap();
    let input: Map<String, Value> = serde_json::from_slice(&input).unwrap();

    let mut entities = Vec::new();
    for (name, info) in input {
        entities.push((name, String::from(info["characters"].as_str().unwrap())))
    }

    entities
}
