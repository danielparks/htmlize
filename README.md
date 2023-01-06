# Encode and decode HTML entities

[![docs.rs](https://img.shields.io/docsrs/htmlize)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/htmlize)][crates.io]

## Escaping text into entities

The `escape` functions should cover most cases where you need to safely embed a
string in HTML. Generally, if the text goes in an attribute, use
[`escape_attribute`], otherwise use [`escape_text`].

The differences between the functions are more exactly summed up below.

Character | Entity   | `escape_text` | `escape_attribute` | `escape_all_quotes`
----------|----------|---------------|--------------------|--------------------
`&`       | `&amp;`  | ✔             | ✔                  | ✔
`<`       | `&lt;`   | ✔             | ✔                  | ✔
`>`       | `&gt;`   | ✔             | ✔                  | ✔
`"`       | `&quot;` |               | ✔                  | ✔
`'`       | `&apos;` |               |                    | ✔

**Note:** These are not sufficient to escape strings embedded in comments.

### `escape_text(string) -> String`

Escape a string so that it can be embedded in the main text. This does not
escape quotes at all.

[API docs.](https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_text.html)

### `escape_attribute(string) -> String`

Escape a string so that it can be embedded in an attribute. Always use double
quotes around attributes.

[API docs.](https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_attribute.html)

### `escape_all_quotes(string) -> String`

Escape a string including both single and double quotes. In general you should
not need to use this.

[API docs.](https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_all_quotes.html)

## Unescaping entities into text

This requires the `unescape` feature.

### `unescape(string) -> String`

This follows the [official WHATWG algorithm] for expanding entities. The only
exception should be that entities are supposed to be expanded slightly
differently when they are in the main body of the text as opposed to within an
attribute.

[API docs.](https://docs.rs/htmlize/0.5.1/htmlize/fn.unescape.html)

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
[`escape_attribute`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_attribute.html
[`escape_text`]: https://docs.rs/htmlize/0.5.1/htmlize/fn.escape_text.html
[official WHATWG algorithm]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
[phf]: https://crates.io/crates/phf
