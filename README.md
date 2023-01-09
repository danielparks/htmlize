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

You should almost never need [`escape_all_quotes()`], but is included because
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

This follows the [official WHATWG algorithm] for expanding entities. The only
exception should be that entities are supposed to be expanded slightly
differently when they are in the main body of the text as opposed to within an
attribute.

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
[official WHATWG algorithm]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
[phf]: https://crates.io/crates/phf
