// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::InBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .ignore_case(false)
            .value(vec!["AB".into()])
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_true_for_string_value_contained_in_supplied_array() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value(vec!["ABC".into(), "bar".into(), "foo".into()])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_for_string_mismatched_only_by_case() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value(vec!["aBc".into(), "bar".into(), "foo".into()])
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
        InBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value(vec!["aBc".into(), "bar".into(), "foo".into()])
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
pub fn returns_true_for_shallow_object_in_supplied_array() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/objX/objY").unwrap())
            .value(vec![
                serde_json::json!({ "num2": 2 }),
                "bar".into(),
                "foo".into(),
            ])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_deep_object_in_supplied_array() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/objX").unwrap())
            .value(vec![
                serde_json::json!({ "num1": 1, "stringAbc": "Abc", "objY": { "num2": 2} }),
                "bar".into(),
                "foo".into(),
            ])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_number_value_contained_in_supplied_array() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value(vec![
                serde_json::json!({ "foo": "foo" }),
                2.into(),
                "bar".into(),
            ])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_if_value_is_not_an_array() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/stringA").unwrap())
            .value(["ABC".into()])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_for_string_match_inside_object_with_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/objX").unwrap())
            .value([
                "foo".into(),
                serde_json::json!({"num1": 1, "stringAbc": "aBc", "objY": {"num2": 2}}),
                "bar".into(),
            ])
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
pub fn returns_false_for_string_inside_object_mismatched_only_by_case_without_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/objX").unwrap())
            .value([
                "foo".into(),
                serde_json::json!({"num1": 1, "stringAbc": "aBc", "objY": {"num2": 2}}),
                "bar".into(),
            ])
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
        InBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .value(["does not matter".into()])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_debug_snapshot!(result.unwrap_err(), @r###"
    PredicateError {
        kind: JsonPointerIssue {
            issue: NoSuchKey(
                "objZZZ",
            ),
        },
    }
    "###);
}

#[test]
pub fn returns_err_for_undefined_value_with_ignored_case() {
    let predicate: Predicate = FirstOrder::from(
        InBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .ignore_case(true)
            .value(["does not matter".into()])
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_debug_snapshot!(result.unwrap_err(), @r###"
    PredicateError {
        kind: JsonPointerIssue {
            issue: NoSuchKey(
                "objZZZ",
            ),
        },
    }
    "###);
}
