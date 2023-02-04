# Change log

All notable changes to this project will be documented in this file.

## main branch

### Breaking changes

* Hid `unescape()` behind `unescape` feature. This allows users to avoid the
  dependency on [phf][] and the build dependency on [serde_json][], which cuts
  build times on my machine by more than 90% (from 6.2 seconds to 0.5 seconds).
* Switched both escape and unescape functions to use `Cow<'a, str>` for input
  and output. This allows for significant performance improvements when the
  input can be returned unchanged (see performance improvements below for more).
* Updated minimum supported Rust version (MSRV) to 1.60.

### Improvements

* Significantly optimized both escape and unescape functions. Many of the
  improvements to the escape functions are similar to the ones outlined in Lise
  Henry’s [excellent blog post on optimizing HTML entity
  escaping][optimize-blog] (see also: [its Reddit discussion][optimize-reddit]),
  though most notably I’m using [memchr][] directly rather than [regex][].
* Added `unescape_attribute()` to handle the special rules for dealing with
  entities in the value of an HTML attribute. Also adds `unescape_in()`, which
  takes a context parameter that can either be `Context::Attribute` or
  `Context::General` (for everything else).
* Added `escape_..._bytes()` functions to work on `[u8]` rather than `str`.
* Switched to the [phf_codegen][] crate instead of using the `phf_map!` macro.
  On my machine, this cuts build time by about 25% (~2 seconds).
* Pre-allocated the output buffer for `unescape()`, which generally improves
  performance slightly.
* Clarified documentation of `ENTITIES` to indicate that it’s a `Map`, not just
  a collection of tuples.

### Bug fixes

* `unescape()` incorrectly outputted the replacement character (U+FFFD “�”) for
  certain numeric entities:

    * [Noncharacters]
    * [Control] characters
    * `0x0D` (carriage return)

  A close reading of the [spec] and some browser testing shows that behavior to
  be incorrect. Those characters are now outputted as themselves.

* `unescape()` incorrectly outputted long numeric entities as the literal text
  of the entity.

  A close reading of the [spec] and some browser testing shows that behavior to
  be incorrect. Those long entities are now outputted as the replacement
  character (U+FFFD “�”).

[phf]: https://crates.io/crates/phf
[phf_codegen]: https://crates.io/crates/phf_codegen
[serde_json]: https://crates.io/crates/serde_json
[optimize-blog]: https://lise-henry.github.io/articles/optimising_strings.html
[optimize-reddit]: https://www.reddit.com/r/rust/comments/55wpxh/optimising_string_processing_in_rust/
[memchr]: https://docs.rs/memchr
[regex]: https://docs.rs/regex
[Noncharacters]: https://infra.spec.whatwg.org/#noncharacter
[Control]: https://infra.spec.whatwg.org/#control
[spec]: https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-end-state

## Release 0.5.1 (2022-12-13)

### Bug fixes

* Switched from [assertify][] (deprecated) to [assert2][] for testing.
* Fixed typo of in docs: “entries.json” should have been “entities.json”.
* Fixed formatting and lints.
* Added this change log.

[assertify]: https://crates.io/crates/assertify
[assert2]: https://crates.io/crates/assert2
