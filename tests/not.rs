// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::NotBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::{Predicate, PredicateImpl, SecondOrder};

mod utils;
use serde::Deserialize;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate =
        SecondOrder::from(NotBuilder::default().apply([]).build().unwrap()).into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn return_true_for_not_case_f_f_with_shallow_endpoint() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "not",
        "apply": [
          {
            "op": "defined",
            "path": "/not_real_thing"
          },
          {
            "op": "defined",
            "path": "/not_real_1"
          }
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn return_true_for_not_case_f_f_with_deep_endpoint() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "not",
        "apply": [
          {
            "op": "defined",
            "path": "/objA/not_real_thing"
          },
          {
            "op": "defined",
            "path": "/objA/not_real_1"
          }
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn return_false_for_not_case_t_f_and_compound_paths() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "not",
        "path": "/objA",
        "apply": [
          {
            "op": "defined",
            "path": "/not_real_thing"
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
    assert!(!result.unwrap());
}

#[test]
pub fn return_false_for_or_case_t_f_t() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "not",
        "path": "/objA",
        "apply": [
          {
            "op": "defined",
            "path": "/stringX"
          },
          {
            "op": "defined",
            "path": "/not_real"
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
    assert!(!result.unwrap());
}

#[test]
pub fn return_true_for_not_case_f_f() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "not",
        "path": "/objA",
        "apply": [
          {
            "op": "defined",
            "path": "/not_real_1"
          },
          {
            "op": "defined",
            "path": "/not_real_2"
          },
        ],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_not_case_empty_array() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "not",
        "path": "/objA",
        "apply": [],
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}
