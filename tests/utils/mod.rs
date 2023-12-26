// Fixture adapted from https://github.com/MalcolmDwyer/json-predicate/blob/master/test/test.js

use serde_json::json;

lazy_static::lazy_static! {
    pub static ref ENTRY: serde_json::Value = json!({
        "num1": 1,
        "null1": null,
        "stringA": "A",
        "stringABC": "ABC",
        "stringAbC_123": "AbC_123",
        "arrayA": ["a","b", "c"],
        "arrayB": ["a", {"foo": "b"}, 3],
        "arrayC": ["a", {"foo": {"bar": "b"}}, 3],
        "arrayN": ["a", {"foo": {"num1": 1}}, 3],
        "objA": {
           "num2": 2,
           "null2": null,
           "boolT": true,
           "boolTS": "true",
           "boolF": false,
           "dateTime": "2010-10-10T10:10:10Z",
           "dateTimeFuture": "2080-10-10T10:10:10Z",
           "dateTimeOffset": "2010-10-10T10:10:10+05:30",
           "date": "2010-10-10",
           "timeZ": "10:10:10Z",
           "timeOffset": "10:10:10+05:30",
           "lang": "en-US",
           "langRange": "CH-*",
           "langRange2": "*",
           "langRange3": "CH-de",
           "iri": "https://github.com/MalcolmDwyer/json-predicate#test",
           "absoluteIri": "https://github.com/MalcolmDwyer/json-predicate",
           "stringX": "X",
           "stringXYZ": "XYZ",
           "stringXyZ_789": "XyZ_789",
           "objB": {
                "num3": 3,
                "null3": null,
                "stringM": "M",
                "stringMNO": "MNO",
                "stringMnO_456": "MnO_456"
            }
        },
        "objX": {
            "num1": 1,
            "stringAbc": "Abc",
            "objY": {
                "num2": 2
            }
        }
    });
}
