use std::borrow::Cow;

macro_rules! find_u8_body {
    ($slice:expr, $ch1:literal $(,)?) => {
        memchr::memchr($ch1, $slice)
    };
    ($slice:expr, $ch1:literal, $ch2:literal $(,)?) => {
        memchr::memchr2($ch1, $ch2, $slice)
    };
    ($slice:expr, $ch1:literal, $ch2:literal, $ch3:literal $(,)?) => {
        memchr::memchr3($ch1, $ch2, $ch3, $slice)
    };
    ($slice:expr, $ch1:literal, $ch2:literal, $ch3:literal, $ch4:literal, $ch5:literal $(,)?) => {{
        if let Some(i) = memchr::memchr3($ch1, $ch2, $ch3, $slice) {
            if let Some(j) = memchr::memchr2($ch4, $ch5, &$slice[..i]) {
                // j has to be less than i
                Some(j)
            } else {
                Some(i)
            }
        } else {
            memchr::memchr2($ch4, $ch5, $slice)
        }
    }};
    ($slice:expr, $($ch:literal),+) => {
        $slice.iter().position(|c| matches!(c, $($ch)|+))
    };
}

macro_rules! escape_fn {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident {
            $($ch:literal => $entity:literal,)+
        }
    ) => {
        $(#[$meta])*
        $vis fn $name<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
            let input = input.into();
            let raw = input.as_bytes();

            #[inline]
            fn find_u8(haystack: &[u8]) -> Option<usize> {
                find_u8_body!(haystack, $($ch),+)
            }

            #[inline]
            fn map_u8(c: u8) -> &'static [u8] {
                match c {
                    $( $ch => $entity, )+
                    // This should never happen, but using unreachable!()
                    // actually makes other parts of the function slower.
                    _ => b"",
                }
            }

            if let Some(i) = find_u8(raw) {
                let mut output: Vec<u8> = Vec::with_capacity(raw.len() * 2);
                output.extend_from_slice(&raw[..i]);
                output.extend_from_slice(map_u8(raw[i]));
                let mut remainder = &raw[i+1..];

                while let Some(i) = find_u8(remainder) {
                    output.extend_from_slice(&remainder[..i]);
                    output.extend_from_slice(map_u8(remainder[i]));
                    remainder = &remainder[i+1..];
                }

                output.extend_from_slice(&remainder);

                return String::from_utf8(output).unwrap().into();
            }

            input
        }
    }
}

escape_fn! {
    /// Escape a string used in a text node, i.e. regular text.
    ///
    /// **Do not use this in attributes.**
    ///
    /// ```rust
    /// use htmlize::escape_text;
    ///
    /// assert_eq!(
    ///     escape_text(r#"Björk & Борис O'Brien <3, "love > hate""#),
    ///     r#"Björk &amp; Борис O'Brien &lt;3, "love &gt; hate""#
    /// );
    /// ```
    pub fn escape_text {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
    }
}

escape_fn! {
    /// Escape a string to be used in a quoted attribute.
    ///
    /// ```rust
    /// use htmlize::escape_attribute;
    ///
    /// assert_eq!(
    ///     escape_attribute(r#"Björk & Борис O'Brien <3, "love > hate""#),
    ///     "Björk &amp; Борис O'Brien &lt;3, &quot;love &gt; hate&quot;"
    /// );
    /// ```
    pub fn escape_attribute {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
        b'"' => b"&quot;", // Attributes
    }
}

escape_fn! {
    /// Escape a string including both single and double quotes.
    ///
    /// Generally, it is safe to leave single quotes (apostrophes) unescaped, so you
    /// should use [`escape_text()`] or [`escape_attribute()`].
    ///
    /// ```rust
    /// use htmlize::escape_all_quotes;
    ///
    /// assert_eq!(
    ///     escape_all_quotes(r#"Björk & Борис O'Brien <3, "love > hate""#),
    ///     "Björk &amp; Борис O&apos;Brien &lt;3, &quot;love &gt; hate&quot;"
    /// );
    /// ```
    pub fn escape_all_quotes {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
        b'"' => b"&quot;",  // Attributes
        b'\'' => b"&apos;", // Single quoted attributes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_CORPUS: [(&str, &str); 7] = [
        ("", ""),
        ("clean", "clean"),
        ("< >", "&lt; &gt;"),
        ("&amp;", "&amp;amp;"),
        ("prefix&", "prefix&amp;"),
        ("☺️&☺️", "☺️&amp;☺️"),
        (
            "Björk and Борис OBrien ❤️, “love beats hate”",
            "Björk and Борис OBrien ❤️, “love beats hate”",
        ),
    ];

    test_multiple!(escape_text_basic, escape_text, BASIC_CORPUS);
    test_multiple!(escape_attribute_basic, escape_attribute, BASIC_CORPUS);
    test_multiple!(escape_all_quotes_basic, escape_all_quotes, BASIC_CORPUS);

    test!(
        escape_text_quotes,
        escape_text("He said, \"That's mine.\"") == "He said, \"That's mine.\""
    );

    test!(
        escape_attribute_quotes,
        escape_attribute("He said, \"That's mine.\"")
            == "He said, &quot;That's mine.&quot;"
    );

    test!(
        escape_all_quotes_quotes,
        escape_all_quotes("He said, \"That's mine.\"")
            == "He said, &quot;That&apos;s mine.&quot;"
    );

    const HTML_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
    const HTML_DIRTY_ESCAPED: &str =
        include_str!("../tests/corpus/html-escaped.txt");
    const HTML_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

    test!(
        escape_text_dirty_html,
        escape_text(HTML_DIRTY) == HTML_DIRTY_ESCAPED
    );
    test!(
        escape_text_clean_html,
        escape_text(HTML_CLEAN) == HTML_CLEAN
    );
}
