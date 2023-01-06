# Change log

All notable changes to this project will be documented in this file.

## main branch

* Switch to the [phf_codegen][] crate instead of using the `phf_map!` macro.
  On my machine, this cuts build time by about 25% (~2 seconds).
* Clarified documentation of `ENTITIES` to indicate that it’s a `Map`, not just
  a collection of tuples.

[phf_codegen]: https://crates.io/crates/phf_codegen

## Release 0.5.1 (2022-12-13)

### Bugfixes

* Switched from [assertify][] (deprecated) to [assert2][] for testing.
* Fixed typo of in docs: “entries.json” should have been “entities.json”.
* Fixed formatting and lints.
* Added this change log.

[assertify]: https://crates.io/crates/assertify
[assert2]: https://crates.io/crates/assert2
