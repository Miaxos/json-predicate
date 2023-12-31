use derive_builder::Builder;
use regex::RegexBuilder;
use serde_json::Value;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{ser::SerializeStruct, Serialize};

use crate::json_path::JSONPath;
use crate::predicate::context::PredicateContext;
use crate::predicate::error::PredicateError;
use crate::predicate::PredicateImpl;
use crate::regex::Regex;

use super::FirstOrder;

/// The "matches" predicate evaluates as true if the referenced element is
/// defined and has a value whose completely string representation matches the
/// regular expression provided by the predicate object's "value" member.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Matches {
    /// Must be a [JSON Pointer](https://tools.ietf.org/html/rfc6901)
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    pub path: Option<JSONPath>,
    #[builder(default)]
    pub ignore_case: bool,
    /// Must be a Regex
    pub value: Regex,
}

impl Serialize for Matches {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Matches", 3)?;
        if self.ignore_case {
            state.serialize_field("op", "matches-")?;
        } else {
            state.serialize_field("op", "matches")?;
        }
        state.serialize_field("path", &self.path)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Matches {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            op,
            path,
            value,
            __ignore,
        }
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("field identifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "op" => Ok(Field::op),
                    "path" => Ok(Field::path),
                    "value" => Ok(Field::value),
                    _ => Ok(Field::__ignore),
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }

        struct VisitorMatches<'de> {
            marker: PhantomData<Matches>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorMatches<'de> {
            type Value = Matches;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Matches")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<Option<JSONPath>> = None;
                let mut value: Option<Regex> = None;
                let mut op: Option<String> = None;

                while let Some(key) = MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::op => {
                            if op.is_some() {
                                return Err(serde::de::Error::duplicate_field("op"));
                            }
                            op = Some(MapAccess::next_value::<String>(&mut map)?);
                        }
                        Field::path => {
                            if path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path = Some(MapAccess::next_value::<Option<JSONPath>>(&mut map)?);
                        }
                        Field::value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(MapAccess::next_value::<Regex>(&mut map)?);
                        }
                        Field::__ignore => {}
                    }
                }

                let path = path.ok_or(serde::de::Error::missing_field("path"))?;
                let value = value.ok_or(serde::de::Error::missing_field("value"))?;
                let op = op.ok_or(serde::de::Error::missing_field("op"))?;

                let ignore_case = match op.as_str() {
                    "matches" => false,
                    "matches-" => true,
                    _ => {
                        return Err(serde::de::Error::custom(
                            "`op` should be either `matches` or `matches-`",
                        ));
                    }
                };

                Ok(Matches {
                    path,
                    ignore_case,
                    value,
                })
            }
        }

        const FIELDS: &[&str] = &["path", "op", "value"];
        Deserializer::deserialize_struct(
            deserializer,
            "Matches",
            FIELDS,
            VisitorMatches {
                marker: PhantomData::<Matches>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Matches> for FirstOrder {
    fn from(value: Matches) -> Self {
        FirstOrder::Matches(value)
    }
}

impl PredicateImpl for Matches {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path).unwrap_or(JSONPath::empty());
        let ptr = path.take();

        let value = &self.value;
        let context_value = ptr.get(data)?;

        match context_value {
            Value::String(context) => {
                if self.ignore_case {
                    let value = RegexBuilder::new(value.as_ref().as_str())
                        .case_insensitive(true)
                        .build()
                        .map_err(|_err| PredicateError::ty_issue())?;

                    Ok(value.is_match(context))
                } else {
                    Ok(value.as_ref().is_match(context))
                }
            }
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use serde::Deserialize;

    use crate::{json_path::JSONPath, predicate::first_order::matches::Matches};

    #[test]
    fn snapshot_test() {
        let matches_expect = serde_json::json!({
             "op": "matches",
             "path": "/a/b",
             "value": ".*"
        });

        let matches = Matches {
            path: Some(JSONPath::new("/a/b").unwrap()),
            ignore_case: false,
            value: Regex::new(".*").unwrap().into(),
        };

        assert_eq!(serde_json::to_value(matches).unwrap(), matches_expect);
    }

    #[test]
    fn deser_test() {
        let matches_expect = serde_json::json!({
             "op": "matches",
             "path": "/a/b",
             "value": ".*"
        });

        let matches = Matches {
            path: Some(JSONPath::new("/a/b").unwrap()),
            ignore_case: false,
            value: Regex::new(".*").unwrap().into(),
        };

        let deser = Matches::deserialize(matches_expect).unwrap();

        assert_eq!(matches, deser);
    }
}
