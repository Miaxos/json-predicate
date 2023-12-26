// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::UndefinedBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        UndefinedBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_true_for_non_existing_key_shallow() {
    let predicate: Predicate = FirstOrder::from(
        UndefinedBuilder::default()
            .path(JSONPath::new("/not_a_key".to_owned()).unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_true_for_non_existing_key_deep() {
    let predicate: Predicate = FirstOrder::from(
        UndefinedBuilder::default()
            .path(JSONPath::new("/objA/not_a_key".to_owned()).unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_false_for_existing_key_shallow() {
    let predicate: Predicate = FirstOrder::from(
        UndefinedBuilder::default()
            .path(JSONPath::new("/num1".to_owned()).unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn returns_false_for_existing_key_deep() {
    let predicate: Predicate = FirstOrder::from(
        UndefinedBuilder::default()
            .path(JSONPath::new("/objA/objB/num3".to_owned()).unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn returns_false_for_existing_key_with_null_value_deep() {
    let predicate: Predicate = FirstOrder::from(
        UndefinedBuilder::default()
            .path(JSONPath::new("/objA/objB/null3".to_owned()).unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}
