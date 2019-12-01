#[inline]
fn map_u8(c: u8) -> &'static [u8] {
    match c {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
        b'"' => b"&quot;", // Attributes
        b'\'' => b"&apos;", // Single quoted attributes
        _ => panic!("map_u8 called on invalid character {}", char::from(c)),
    }
}

macro_rules! encoder {
    ($raw:expr, $($ch:literal),+) => {{
        let raw = $raw.as_ref();
        let mut output:Vec<u8> = Vec::with_capacity(raw.len());

        for c in raw {
            match c {
                $($ch => output.extend_from_slice(map_u8(*c)),)+
                _ => output.push(*c),
            }
        }

        String::from_utf8(output).unwrap()
    }}
}

/// Escape a string used in a text node, i.e. regular text.
///
/// **Do not use this in attributes or comments.**
pub fn encode_text<S: AsRef<[u8]>>(raw: S) -> String {
    encoder!(raw, b'&', b'<', b'>')
}

/// Escape a string used in quoted attribute.
///
/// **Do not use this in of unquoted or single-quoted attributes, or in comments.**
pub fn encode_attribute<S: AsRef<[u8]>>(raw: S) -> String {
    encoder!(raw, b'&', b'<', b'>', b'"')
}

/// Escape a string including both apostrophes and double quotes.
///
/// **Do not use this outside of quoted attributes or in comments.** You should
/// avoid using this unless you need to use single-quoted attributes. Generally,
/// it is safe to leave apostrophes unescaped.
pub fn encode_quotes<S: AsRef<[u8]>>(raw: S) -> String {
    encoder!(raw, b'&', b'<', b'>', b'"', b'\'')
}

pub fn old_encode_text<S>(raw: S) -> String
    where S: AsRef<[u8]>
{
    let raw = raw.as_ref();
    let mut output:Vec<u8> = Vec::with_capacity(raw.len());

    for c in raw {
        match c {
            b'&' => output.extend_from_slice(b"&amp;"),
            b'<' => output.extend_from_slice(b"&lt;"),
            b'>' => output.extend_from_slice(b"&gt;"),
            _ => output.push(*c),
        }
    }

    String::from_utf8(output).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    // FIXME corpus isn’t the right word.
    macro_rules! test_corpus {
        ($name:ident, $func:ident, $corpus:expr) => {
            paste::item! {
                #[test]
                fn $name() {
                    let corpus = $corpus;
                    for (input, expected) in &corpus {
                        let actual = $func(&input);
                        assert_eq!(actual, *expected,
                            "actual ≠ expected (left ≠ right)");
                    }
                }
            }
        }
    }

    const BASIC_CORPUS: [(&str, &str); 4] = [
        ("", ""),
        ("clean", "clean"),
        ("< >", "&lt; &gt;"),
        ("&amp;", "&amp;amp;"),
    ];

    test_corpus!(encode_text_basic, encode_text, BASIC_CORPUS);
    test_corpus!(encode_attribute_basic, encode_attribute, BASIC_CORPUS);
    test_corpus!(encode_quotes_basic, encode_quotes, BASIC_CORPUS);

    test_corpus!(encode_text_quotes, encode_text, [
        ("He said, \"That's mine.\"", "He said, \"That's mine.\""),
    ]);

    test_corpus!(encode_attribute_quotes, encode_attribute, [
        ("He said, \"That's mine.\"", "He said, &quot;That's mine.&quot;"),
    ]);

    test_corpus!(encode_quotes_quotes, encode_quotes, [
        ("He said, \"That's mine.\"", "He said, &quot;That&apos;s mine.&quot;"),
    ]);

    const HTML_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
    const HTML_DIRTY_ENCODED: &str = include_str!("../tests/corpus/html-encoded.txt");
    const HTML_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

    test_corpus!(encode_text_html, encode_text, [
        (HTML_DIRTY, HTML_DIRTY_ENCODED),
        (HTML_CLEAN, HTML_CLEAN),
    ]);
}
