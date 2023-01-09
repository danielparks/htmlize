# Encode and decode HTML entities

[![docs.rs](https://img.shields.io/docsrs/htmlize)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/htmlize)][crates.io]

If you only need to escape text for embedding into HTML then installing is as
simple as running:

```sh
cargo add htmlize
```

If you want to unescape entities back into raw text, see [Unescaping entities
into text](#unescaping-entities-into-text) below.

## Escaping text into entities

The `escape` functions should cover most cases where you need to safely embed a
string in HTML. Generally, if the text goes in an attribute, use
[`escape_attribute()`], otherwise use [`escape_text()`].

|                         | `&` | `<` | `>` | `"` | `'` |
|-------------------------|:---:|:---:|:---:|:---:|:---:|
| [`escape_text()`]       |  ✓  |  ✓  |  ✓  |     |     |
| [`escape_attribute()`]  |  ✓  |  ✓  |  ✓  |  ✓  |     |
| [`escape_all_quotes()`] |  ✓  |  ✓  |  ✓  |  ✓  |  ✓  |

You should almost never need [`escape_all_quotes()`], but it is included because
sometimes it’s convenient to wrap attribute values in single quotes.

### `escape_text(string) -> String` ([reference][`escape_text()`])

Escape a string so that it can be embedded in the main text. This does not
escape quotes at all.

### `escape_attribute(string) -> String` ([reference][`escape_attribute()`])

Escape a string so that it can be embedded in an attribute. Always use double
quotes around attributes.

### `escape_all_quotes(string) -> String` ([reference][`escape_all_quotes()`])

Escape both single and double quotes in a string along with other standard
characters. In general you should not need to use this.

## Unescaping entities into text

This requires the `unescape` feature. To configure it:

```sh
cargo add htmlize --features unescape
```

### `unescape(string) -> String` ([reference][`unescape()`])

This follows the [official WHATWG algorithm] for expanding entities in general.

Strictly speaking, this does not correctly handle text from the value of
attributes. It’s probably fine for most uses, but if you know that the input
string came from the value of an attribute, use [`unescape_attribute()`]
instead. See the [`unescape_in()` reference documentation][`unescape_in()`] for
more information.

### `unescape_attribute(string) -> String` ([reference][`unescape_attribute()`])

This follows the [official WHATWG algorithm] for expanding entities found in the
value of an attribute.

The only difference is in how this handles named entities without a trailing
semicolon. See the [`unescape_in()` reference documentation][`unescape_in()`]
for more information.

### `unescape_in(string, Htmlize::Context) -> String` ([reference][`unescape_in()`])

This follows the [official WHATWG algorithm] for expanding entities based on
the context where they are found. See the [reference
documentation][`unescape_in()`] for more information.

## Features

  * `unescape`: build `ENTITIES` map and provide `unescape()` function. Enabling
    this will add a dependency on [phf] and may slow builds by a few seconds.

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[docs.rs]: https://docs.rs/htmlize/latest/htmlize/
[crates.io]: https://crates.io/crates/htmlize
[`escape_text()`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_text.html
[`escape_attribute()`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_attribute.html
[`escape_all_quotes()`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_all_quotes.html
[`unescape()`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.unescape.html
[`unescape_attribute()`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.unescape_attribute.html
[`unescape_in()`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.unescape_in.html
[official WHATWG algorithm]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
[phf]: https://crates.io/crates/phf
