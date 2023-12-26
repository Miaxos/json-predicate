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

## Introduction

This specification defines JSON Predicates, a JSON-based
[RFC4627](https://datatracker.ietf.org/doc/html/rfc4627)
syntax for the description and serialization of logical boolean
predicate operations intended to be used in conjunction with other
JSON-based mechanisms, such as JSON Patch [RFC6902](https://datatracker.ietf.org/doc/html/rfc6902),
as a means of incorporating conditional processing.

JSON Predicates can be used, for instance, to extend a JSON Patch
[RFC6902](https://datatracker.ietf.org/doc/html/rfc6902) document to provide 
for a broader range of conditional processing options not currently supported 
by JSON Patch.

## Example

Given this JSON:
```json
{
    "a": {
        "b": {
            "c": "ABC!XYZ"
        }
    }
}
```

We could have a predicate like this: 
```json
{
    "op": "and",
    "path": "/a/b/c",
    "apply": [
        {
            "op": "type",
            "value": "string"
        },
        {
            "op": "contains",
            "value": "ABC"
        }
    ]
}
```

which would evaluate as `true` if evaluated against the previous JSON.

## Rust Example

```rust
let predicate = Predicate::deserialize(serde_json::json!({
    "op": "and",
    "path": "/objA",
    "apply": [
      {
        "op": "defined",
        "path": "/stringX"
      },
      {
        "op": "defined",
        "path": "/stringXYZ"
      }
    ],
}))?;

let evaluted_predicate: bool = predicate
    .test(&ENTRY, PredicateContext::default());
```

## JSON Patch

The JSON Patch methods described in [draft-snell-json-test-07](https://datatracker.ietf.org/doc/html/draft-snell-json-test-07)
are not implemented yet.

## Features

### First order predicate

- [x] "contains"
- [x] "contains-"
- [x] "defined"
- [x] "undefined"
- [x] "starts"
- [x] "starts-"
- [x] "ends"
- [x] "ends-"
- [x] "type"
- [x] "in"
- [x] "in-"
- [x] "matches"
- [x] "matches-"
- [x] "test"
- [x] "test-"
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
