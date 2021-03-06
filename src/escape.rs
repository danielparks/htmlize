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

macro_rules! escape {
    ($raw:expr, $($ch:literal),+) => {{
        let raw = $raw.as_ref();
        let mut output: Vec<u8> = Vec::with_capacity(raw.len());

        for c in raw {
            match c {
                $($ch)|+ => output.extend_from_slice(map_u8(*c)),
                _ => output.push(*c),
            }
        }

        String::from_utf8(output).unwrap()
    }}
}

/// Escape a string used in a text node, i.e. regular text.
///
/// **Do not use this in attributes or comments.**
pub fn escape_text<S: AsRef<[u8]>>(raw: S) -> String {
    escape!(raw, b'&', b'<', b'>')
}

/// Escape a string used in a quoted attribute.
///
/// **Do not use this in comments.**
pub fn escape_attribute<S: AsRef<[u8]>>(raw: S) -> String {
    escape!(raw, b'&', b'<', b'>', b'"')
}

/// Escape a string including both single and double quotes.
///
/// **Do not use this in comments.** Generally, it is safe to leave single
/// quotes (apostrophes) unescaped.
pub fn escape_all_quotes<S: AsRef<[u8]>>(raw: S) -> String {
    escape!(raw, b'&', b'<', b'>', b'"', b'\'')
}


#[cfg(test)]
mod tests {
    use super::*;
    use assertify::testify;

    const BASIC_CORPUS: [(&str, &str); 4] = [
        ("", ""),
        ("clean", "clean"),
        ("< >", "&lt; &gt;"),
        ("&amp;", "&amp;amp;"),
    ];

    test_multiple!(escape_text_basic, escape_text, BASIC_CORPUS);
    test_multiple!(escape_attribute_basic, escape_attribute, BASIC_CORPUS);
    test_multiple!(escape_all_quotes_basic, escape_all_quotes, BASIC_CORPUS);

    testify!(escape_text_quotes,
        escape_text("He said, \"That's mine.\"")
            == "He said, \"That's mine.\"");

    testify!(escape_attribute_quotes,
        escape_attribute("He said, \"That's mine.\"")
            == "He said, &quot;That's mine.&quot;");

    testify!(escape_all_quotes_quotes,
        escape_all_quotes("He said, \"That's mine.\"")
            == "He said, &quot;That&apos;s mine.&quot;");

    const HTML_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
    const HTML_DIRTY_ESCAPED: &str = include_str!("../tests/corpus/html-escaped.txt");
    const HTML_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

    testify!(escape_text_dirty_html, escape_text(HTML_DIRTY) == HTML_DIRTY_ESCAPED);
    testify!(escape_text_clean_html, escape_text(HTML_CLEAN) == HTML_CLEAN);
}
