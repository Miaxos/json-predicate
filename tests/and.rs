// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::AndBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{Predicate, PredicateImpl, SecondOrder};

mod utils;
use serde::Deserialize;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate =
        SecondOrder::from(AndBuilder::default().apply([]).build().unwrap()).into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn return_true_for_and_case_with_shallow_endpoint() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "and",
        "apply": [
          {
            "op": "defined",
            "path": "/stringA"
          },
          {
            "op": "defined",
            "path": "/stringABC"
          }
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn return_true_for_and_case_with_deep_endpoint() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "and",
        "apply": [
          {
            "op": "defined",
            "path": "/objA/stringX"
          },
          {
            "op": "defined",
            "path": "/objA/stringXYZ"
          }
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_true_for_and_case_with_t_t_and_compound_paths() {
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
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_true_for_and_case_with_t_t_t() {
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
          },
          {
            "op": "defined",
            "path": "/null2"
          }
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_err_for_and_case_with_t_f() {
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
            "path": "/not_read"
          }
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_debug_snapshot!(result.unwrap_err(), @r###"
    PredicateError {
        kind: JsonPointerIssue {
            issue: NoSuchKey(
                "not_read",
            ),
        },
    }
    "###);
}

#[test]
pub fn returns_true_for_and_case_empty_array() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "and",
        "path": "/objA",
        "apply": [],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}
