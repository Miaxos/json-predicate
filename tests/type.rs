// Test adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use json_predicate::builder::TypeBuilder;
use json_predicate::context::PredicateContext;
use json_predicate::json_path::JSONPath;
use json_predicate::{FirstOrder, Predicate, PredicateImpl};

mod utils;
use utils::ENTRY;

#[test]
pub fn test_base_predicate_snapshot() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/stringABC").unwrap())
            .value("string")
            .build()
            .unwrap(),
    )
    .into();

    insta::assert_json_snapshot!(predicate);
}

#[test]
pub fn returns_true_when_matching_number_to_type_number() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("number")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_string_to_type_number() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ").unwrap())
            .value("number")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_string_to_type_string() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/stringXYZ").unwrap())
            .value("string")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_maching_number_to_string() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("string")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_boolean_to_type_bool() {
    let predicate_t: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/boolT").unwrap())
            .value("boolean")
            .build()
            .unwrap(),
    )
    .into();

    let predicate_f: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/boolF").unwrap())
            .value("boolean")
            .build()
            .unwrap(),
    )
    .into();

    let result_t = predicate_t.evaluate(&ENTRY, PredicateContext::default());
    let result_f = predicate_f.evaluate(&ENTRY, PredicateContext::default());
    assert!(result_t.is_ok());
    assert!(result_t.unwrap());
    assert!(result_f.is_ok());
    assert!(result_f.unwrap());
}

#[test]
pub fn returns_false_when_matching_number_to_type_boolean() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("boolean")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_object_to_type_object() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/objB").unwrap())
            .value("object")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_number_to_type_object() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("object")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_array_to_type_array() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/arrayA").unwrap())
            .value("array")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_number_to_type_array() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("array")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_null_to_type_null() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/null2").unwrap())
            .value("null")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_number_to_type_null() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("null")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_undefined_to_type_undefined() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/not_a_thing").unwrap())
            .value("undefined")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_number_to_type_undefined() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("undefined")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_date_to_type_date() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/date").unwrap())
            .value("date")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_date_time_to_type_date() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/dateTime").unwrap())
            .value("date")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_time_z_to_type_time() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/timeZ").unwrap())
            .value("time")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_when_matching_time_offset_to_type_time() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/timeOffset").unwrap())
            .value("time")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_date_time_to_type_time() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/dateTime").unwrap())
            .value("time")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_date_time_z_to_type_date_time() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/dateTime").unwrap())
            .value("date-time")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_when_matching_date_time_offset_to_type_date_time() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/dateTimeOffset").unwrap())
            .value("date-time")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_date_to_type_date_time() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/date").unwrap())
            .value("date-time")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
pub fn returns_true_when_matching_lang_to_type_lang() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/lang").unwrap())
            .value("lang")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_num_to_type_lang() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("lang")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
#[ignore = "lang-range not fixed"]
pub fn returns_true_when_matching_lang_range_to_type_lang_range() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/langRange").unwrap())
            .value("lang-range")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
#[ignore = "lang-range not fixed"]
pub fn returns_true_when_matching_lang_range2_to_type_lang_range() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/langRange2").unwrap())
            .value("lang-range")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_true_when_matching_lang_range_3_to_type_lang_range() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/langRange3").unwrap())
            .value("lang-range")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
pub fn returns_false_when_matching_num_to_type_lang_range() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("lang-range")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
#[ignore = "iri not fixed"]
pub fn returns_true_when_matching_iri_to_type_iri() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/iri").unwrap())
            .value("iri")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
#[ignore = "iri not fixed"]
pub fn returns_false_when_matching_num_to_type_iri() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("iri")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
#[ignore = "absolute-iri not fixed"]
pub fn returns_true_when_matching_iri_to_type_absolute_iri() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/absolureIri").unwrap())
            .value("absolute-iri")
            .build()
            .unwrap(),
    )
    .into();

    let result = predicate.evaluate(&ENTRY, PredicateContext::default());
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
#[ignore = "absolute-iri not fixed"]
pub fn returns_false_when_matching_num_to_type_absolute_iri() {
    let predicate: Predicate = FirstOrder::from(
        TypeBuilder::default()
            .path(JSONPath::new("/objA/num2").unwrap())
            .value("absolute-iri")
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
        TypeBuilder::default()
            .path(JSONPath::new("/objZZZ/objZZZZZZZZ").unwrap())
            .value("doeijdzoijde eizjdz")
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
