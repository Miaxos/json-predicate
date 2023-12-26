// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::MoreBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use serde::Deserialize;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        MoreBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value(12)
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_err_for_non_numeric_comparaisons() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "more",
        "path": "/stringABC",
        "value": "XYZ",
    }));

    assert!(predicate.is_err());
    insta::assert_debug_snapshot!(predicate.unwrap_err(), @r###"Error("data did not match any variant of untagged enum Predicate", line: 0, column: 0)"###);

    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "more",
        "path": "/objA/stringXYZ",
        "value": "ABC",
    }));

    assert!(predicate.is_err());
    insta::assert_debug_snapshot!(predicate.unwrap_err(), @r###"Error("data did not match any variant of untagged enum Predicate", line: 0, column: 0)"###);

    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "more",
        "path": "/objA/stringXYZ",
        "value": ["a", "b"],
    }));

    assert!(predicate.is_err());
    insta::assert_debug_snapshot!(predicate.unwrap_err(), @r###"Error("data did not match any variant of untagged enum Predicate", line: 0, column: 0)"###);

    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "more",
        "path": "/objA/stringXYZ",
        "value": {"a": "foo", "b": "bar"},
    }));

    assert!(predicate.is_err());
    insta::assert_debug_snapshot!(predicate.unwrap_err(), @r###"Error("data did not match any variant of untagged enum Predicate", line: 0, column: 0)"###);
}

#[test]
pub fn returns_false_for_greater_predicate_value() {
    let predicate: Predicate = FirstOrder::from(
        MoreBuilder::default()
            .path(JSONPath::new("/objA/objB/num3").unwrap())
            .value(4)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_for_lesser_predicate_value() {
    let predicate: Predicate = FirstOrder::from(
        MoreBuilder::default()
            .path(JSONPath::new("/objA/objB/num3").unwrap())
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
pub fn returns_false_for_equal_numeric_value() {
    let predicate: Predicate = FirstOrder::from(
        MoreBuilder::default()
            .path(JSONPath::new("/objA/objB/num3").unwrap())
            .value(3)
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
        MoreBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .value(3)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}
