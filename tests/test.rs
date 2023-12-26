// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::TestBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .ignore_case(false)
            .value("ABC")
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_true_for_matching_string_value() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("ABC")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_for_stirng_mismatched_only_by_case_without_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("aBc")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_for_string_mismatched_only_by_case_with_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("aBc")
            .ignore_case(true)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_matching_shallow_object() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objX/objY").unwrap())
            .value(serde_json::json!({ "num2": 2 }))
            .ignore_case(true)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_matching_deep_object() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objX").unwrap())
            .value(serde_json::json!({ "num1": 1, "stringAbc": "Abc", "objY": { "num2": 2 } }))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_matching_number_value() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value(2)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_matching_array() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/arrayA").unwrap())
            .value(serde_json::json!(["a", "b", "c",]))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_for_superset_array() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/arrayA").unwrap())
            .value(serde_json::json!(["a", "b", "c", "d"]))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_false_for_subset_array() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/arrayA").unwrap())
            .value(serde_json::json!(["a", "b"]))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_false_for_string_disguised_as_array() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/arrayA").unwrap())
            .value("abcd")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_false_for_array_disguised_as_a_string() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value(serde_json::json!(["A", "B", "C"]))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_for_string_match_inside_object_honoring_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objX").unwrap())
            .ignore_case(true)
            .value(serde_json::json!({ "num1": 1, "stringAbc": "aBc", "objY": { "num2": 2 } }))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_for_string_inside_object_mismatched_only_by_case_without_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objX").unwrap())
            .value(serde_json::json!({ "num1": 1, "stringAbc": "aBc", "objY": { "num2": 2 } }))
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_err_for_undefined_value() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .value("dzodeiojdoizej")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_debug_snapshot!(result, @r###"
    Err(
        PredicateError {
            kind: JsonPointerIssue {
                issue: NoSuchKey(
                    "objZZZ",
                ),
            },
        },
    )
    "###);
}

#[test]
pub fn returns_err_for_undefined_value_with_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        TestBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .ignore_case(true)
            .value("dzodeiojdoizej")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_debug_snapshot!(result, @r###"
    Err(
        PredicateError {
            kind: JsonPointerIssue {
                issue: NoSuchKey(
                    "objZZZ",
                ),
            },
        },
    )
    "###);
}
