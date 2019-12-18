# Encode and decode HTML entities

**These are not sufficient to escape strings embedded in comments.**

Character | Entity   | `escape_text` | `escape_attribute` | `escape_all_quotes`
----------|----------|---------------|--------------------|--------------------
`&`       | `&amp;`  | ✔             | ✔                  | ✔
`<`       | `&lt;`   | ✔             | ✔                  | ✔
`>`       | `&gt;`   | ✔             | ✔                  | ✔
`"`       | `&quot;` |               | ✔                  | ✔
`'`       | `&apos;` |               |                    | ✔

### `escape_text(string) -> String`

Escape a string so that it can be embedded in the main text. This does not
escape quotes at all.

[API docs.](https://docs.rs/htmlize/0.5.0/htmlize/fn.escape_text.html)

### `escape_attribute(string) -> String`

Escape a string so that it can be embedded in an attribute. Always use double
quotes around attributes.

[API docs.](https://docs.rs/htmlize/0.5.0/htmlize/fn.escape_attribute.html)

### `escape_all_quotes(string) -> String`

Escape a string including both single and double quotes. In general you should
not need to use this.

[API docs.](https://docs.rs/htmlize/0.5.0/htmlize/fn.escape_all_quotes.html)

### `unescape(string) -> String`

This follows the [official WHATWG algorithm] for expanding entities. The only
exception should be that entities are supposed to be expanded slightly
differently when they are in the main body of the text as opposed to within an
attribute.

[API docs.](https://docs.rs/htmlize/0.5.0/htmlize/fn.unescape.html)

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[official WHATWG algorithm]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
