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

    w!(r#"/// All valid HTML entities and their expansions as `(b"&copy;", b"©")` tuples."#);
    w!("///");
    w!("/// See the [WHATWG HTML spec](https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references)");
    w!("/// the canonical list of entities with their codepoints and glyphs. The");
    w!("/// [entries.json](https://html.spec.whatwg.org/entities.json) file linked");
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

        // \n is a possible value. As long as the “glyph” is last, it’s fine.
        w!("/// {:30} | {:18} | {}", name, codepoints.join(", "), value);
    }

    w!("pub const ENTITIES: [(&[u8], &[u8]); {}] = [", entities.len());

    for (name, value) in &entities {
        w!("    (b{:?}, &{:?}), // {}", name, value.as_bytes(), value);
    }

    w!("];");
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
