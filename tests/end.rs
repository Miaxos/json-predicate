// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::EndBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .ignore_case(false)
            .value("AB")
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_true_for_end_string_shallow() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("BC")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_for_non_ending_string_shallow() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("AB")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_for_end_string_deep() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/objA/objB/stringMNO").unwrap())
            .value("NO")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_for_non_ending_string_deep() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ").unwrap())
            .value("XY")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_false_for_mismatching() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ").unwrap())
            .value("yz")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn honors_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ").unwrap())
            .value("yz")
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
pub fn returns_false_for_undefined_value() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .value("does not matter")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}

#[test]
pub fn returns_false_for_undefined_value_with_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        EndBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .value("does not matter")
            .ignore_case(true)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}
