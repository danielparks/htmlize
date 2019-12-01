#[inline]
fn map_u8(c: u8) -> &'static [u8] {
    match c {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
        b'"' => b"&quot;", // Attributes
        b'\'' => b"&apos;", // Paranoid
        b'/' => b"&#x2F;", // Paranoid (very paranoid)
        _ => panic!("map_u8 called on invalid character {}", char::from(c)),
    }
}

macro_rules! encoder {
    ($name:ident $(, $ch:literal)+) => {
        pub fn $name<S>(raw: S) -> String
            where S: AsRef<[u8]>
        {
            let raw = raw.as_ref();
            let mut output:Vec<u8> = Vec::with_capacity(raw.len());

            for c in raw {
                match c {
                    $($ch => output.extend_from_slice(map_u8(*c)),)+
                    _ => output.push(*c),
                }
            }

            String::from_utf8(output).unwrap()
        }
    }
}

encoder!(encode_text, b'&', b'<', b'>');
encoder!(encode_attribute, b'&', b'<', b'>', b'"');

// https://cheatsheetseries.owasp.org/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.html#rule-1---html-escape-before-inserting-untrusted-data-into-html-element-content
encoder!(encode_paranoid, b'&', b'<', b'>', b'"', b'\'', b'/');

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
        ($name:ident, $func:ident, $corpus:tt) => {
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

    test_corpus!(encode_text_short_strings, encode_text, [
        ("", ""),
        ("clean", "clean"),
        ("< >", "&lt; &gt;"),
        ("&amp;", "&amp;amp;"),
        ("He said, \"That's mine.\"", "He said, \"That's mine.\""),
    ]);

    test_corpus!(encode_attribute_short_strings, encode_attribute, [
        ("", ""),
        ("clean", "clean"),
        ("< >", "&lt; &gt;"),
        ("&amp;", "&amp;amp;"),
        ("He said, \"That's mine.\"", "He said, &quot;That's mine.&quot;"),
    ]);

    test_corpus!(encode_paranoid_short_strings, encode_paranoid, [
        ("", ""),
        ("clean", "clean"),
        ("< >", "&lt; &gt;"),
        ("&amp;", "&amp;amp;"),
        ("He said, \"That's mine.\"", "He said, &quot;That&apos;s mine.&quot;"),
        ("<div class=foo/>", "&lt;div class=foo&#x2F;&gt;"),
    ]);

    const BIG_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
    const BIG_DIRTY_ENCODED: &str = include_str!("../tests/corpus/html-encoded.txt");
    const BIG_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

    test_corpus!(encode_text_html, encode_text, [
        (BIG_DIRTY, BIG_DIRTY_ENCODED),
        (BIG_CLEAN, BIG_CLEAN),
    ]);
}
