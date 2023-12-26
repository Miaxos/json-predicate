# JSON Predicate

[![release](https://github.com/Miaxos/json-predicate/actions/workflows/release.yml/badge.svg)](https://github.com/Miaxos/json-predicate/actions/workflows/release.yml)
[![Crates.io version](https://img.shields.io/crates/v/json-predicate.svg)](https://crates.io/crates/json-predicate)
[![dependency status](https://deps.rs/repo/github/miaxos/json-predicate/status.svg)](https://deps.rs/repo/github/miaxos/json-predicate)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/json-predicate)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/miaxos/json-predicate/compare)

Partial implementation of [Snell Json Predicate Draft
07](https://tools.ietf.org/html/draft-snell-json-test-07).

Even if this Draft is Expired, it's still a pretty good draft of predicate over
JSON.

## Features
https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

### First order predicate

- [x] "contains"
- [x] "contains-"
- [x] "defined"
- [x] "undefined"
- [x] "starts"
- [x] "ends"
- [x] "type"
- [x] "in"
- [x] "matches"
- [x] "test"
- [x] "less"
- [x] "more"

### Second order predicate

- [x] "and"
- [x] "not"
- [x] "or"

## References

- [Specification draft-snell-json-test-07](https://tools.ietf.org/html/draft-snell-json-test-07)

## License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT) at your option.
