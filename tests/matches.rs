// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::MatchesBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .ignore_case(false)
            .value(Regex::new(".*").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_false_for_any_non_string_target_path() {
    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/num1".to_owned()).unwrap())
            .value(Regex::new("[\\w\\s]*").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/null1".to_owned()).unwrap())
            .value(Regex::new("[\\w\\s]*").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/objA".to_owned()).unwrap())
            .value(Regex::new("[\\w\\s]*").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/arrayA".to_owned()).unwrap())
            .value(Regex::new("[\\w\\s]*").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn returns_false_for_string_that_would_make_an_invalid_regex() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/stringABC",
        "value": "\\",
    }));

    assert!(predicate.is_err());
    insta::assert_debug_snapshot!(predicate.unwrap_err(), @r###"Error("data did not match any variant of untagged enum Predicate", line: 0, column: 0)"###);
}

#[test]
pub fn returns_false_for_any_value_other_than_a_regexp_or_a_string() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/stringABC",
        "value": 1,
    }));

    assert!(predicate.is_err());

    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/stringABC",
        "value": { "a": 1},
    }));

    assert!(predicate.is_err());

    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/stringABC",
        "value": ["a", "b"],
    }));

    assert!(predicate.is_err());
}

#[test]
pub fn returns_true_for_match_when_providing_string_that_will_become_regexp() {
    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/stringABC".to_owned()).unwrap())
            .value(Regex::new("[A-Z]*").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_true_for_different_case_string_with_ignore_case() {
    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/stringABC".to_owned()).unwrap())
            .value(Regex::new("aBc").unwrap())
            .ignore_case(true)
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_false_for_different_case_string_without_ignore_case_true() {
    let predicate: Predicate = FirstOrder::from(
        MatchesBuilder::default()
            .path(JSONPath::new("/stringABC".to_owned()).unwrap())
            .value(Regex::new("aBc").unwrap())
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
#[ignore = "failing"]
pub fn returns_true_when_providing_matching_regexp_directly() {
    let json = r###"
    {
      "op": "matches",
      "path": "/stringABC",
      "value": /[A-Z]+/
    }
    "###;
    let predicate = Predicate::deserialize(serde_json::from_str::<Value>(json).unwrap());

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_false_when_providing_matching_regex_with_mismatch_case() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/stringABC",
        "value": "/[a-z]+",
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
pub fn returns_true_when_providing_matching_i_regexp_without_mismatch_case() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/stringABC",
        "value": "(?i)[a-z]+",
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_true_when_providing_matching_i_regexp_with_mismatch_case() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches-",
        "path": "/stringABC",
        "value": "(?i)[a-z]+",
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
pub fn returns_err_for_undefined_value() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches",
        "path": "/objZZZ/objZZZZZZZZ",
        "value": "[whatever]",
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}

#[test]
pub fn returns_err_for_undefined_value_with_ignore_case() {
    let predicate = Predicate::deserialize(serde_json::json!({
        "op": "matches-",
        "path": "/objZZZ/objZZZZZZZZ",
        "value": "[whatever]",
    }));

    assert!(predicate.is_ok());
    let result = predicate
        .unwrap()
        .evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_err());
}
