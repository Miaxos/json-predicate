// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::DefinedBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_true_for_existing_key_shallow() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/num1").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_existing_key_with_null_value() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/null1").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_err_for_non_existent_key_shallow() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/not_a_key").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}

#[test]
pub fn returns_true_for_existing_key_deep() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/objA/objB/num3").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_for_existing_key_with_null_value_deep() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/objA/objB/null3").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_err_for_non_existent_key_deep() {
    let predicate: Predicate = FirstOrder::from(
        DefinedBuilder::default()
            .path(JSONPath::new("/objA/objB/not_a_key").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}
