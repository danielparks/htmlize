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

    w!("/// All valid HTML entities and their equivalents as \
        `(\"&entity;\", \"c\")` tuples.");
    w!("///");
    w!("/// See the [WHATWG HTML spec](https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references)");
    w!("/// for a table of all entities with their codepoints and glyphs.");
    w!("pub const ENTITIES: [(&str, &str); {}] = [", entities.len());

    for (name, value) in &entities {
        w!("    ({:?}, {:?}),", name, value);
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
