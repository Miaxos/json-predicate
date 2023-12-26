// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::ContainsBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("AB")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn return_true_contained_string() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/stringABC".to_owned()).unwrap())
            .value("AB")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn return_false_for_non_contained_string() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/stringABC".to_owned()).unwrap())
            .value("XY")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn return_true_for_contained_string() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/objA/objB/stringMNO".to_owned()).unwrap())
            .value("MN")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn return_false_for_non_contained_string_deep() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ".to_owned()).unwrap())
            .value("AB")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn return_false_for_mismatching_case() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ".to_owned()).unwrap())
            .value("xy")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn return_err_for_undefined_value() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ".to_owned()).unwrap())
            .value("does not matter")
            .ignore_case(false)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_display_snapshot!(result.unwrap_err(), @"The pointer pointed to a nonexistent key, pointed key: objZZZ");
}

#[test]
pub fn return_false_for_undefined_value_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ".to_owned()).unwrap())
            .ignore_case(true)
            .value("does not matter")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
    insta::assert_display_snapshot!(result.unwrap_err(), @"The pointer pointed to a nonexistent key, pointed key: objZZZ");
}

#[test]
pub fn honors_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        ContainsBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ".to_owned()).unwrap())
            .ignore_case(true)
            .value("xy")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}
