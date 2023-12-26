use derive_builder::Builder;
use serde_json::Value;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{ser::SerializeStruct, Serialize};

use crate::json_path::JSONPath;
use crate::predicate::context::PredicateContext;
use crate::predicate::error::PredicateError;
use crate::predicate::PredicateImpl;

use super::FirstOrder;

/// The "start" predicate evaluates as true if the referenced
/// element is defined and has a value whose string representation
/// start the exact sequence of characters given by the predicate
/// object's "value" member.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Start {
    /// Must be a JSON Pointer
    /// https://tools.ietf.org/html/rfc6901
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    pub path: Option<JSONPath>,
    #[builder(default)]
    pub ignore_case: bool,
    pub value: serde_json::Value,
}

impl Serialize for Start {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Start", 3)?;
        if self.ignore_case {
            state.serialize_field("op", "start-")?;
        } else {
            state.serialize_field("op", "start")?;
        }
        state.serialize_field("path", &self.path)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Start {
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

        struct VisitorStart<'de> {
            marker: PhantomData<Start>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorStart<'de> {
            type Value = Start;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Start")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<Option<JSONPath>> = None;
                let mut value: Option<serde_json::Value> = None;
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
                            value = Some(MapAccess::next_value::<serde_json::Value>(&mut map)?);
                        }
                        Field::__ignore => {}
                    }
                }

                let path = path.ok_or(serde::de::Error::missing_field("path"))?;
                let value = value.ok_or(serde::de::Error::missing_field("value"))?;
                let op = op.ok_or(serde::de::Error::missing_field("op"))?;

                let ignore_case = match op.as_str() {
                    "start" => false,
                    "start-" => true,
                    _ => {
                        return Err(serde::de::Error::custom(
                            "`op` should be either `start` or `start-`",
                        ));
                    }
                };

                Ok(Start {
                    path,
                    ignore_case,
                    value,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["path", "op", "value"];
        Deserializer::deserialize_struct(
            deserializer,
            "Start",
            FIELDS,
            VisitorStart {
                marker: PhantomData::<Start>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Start> for FirstOrder {
    fn from(value: Start) -> Self {
        FirstOrder::Start(value)
    }
}

impl PredicateImpl for Start {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path).unwrap_or(JSONPath::empty());
        let ptr = path.take();

        let value = &self.value;
        let context_value = ptr.get(data)?;

        match (context_value, value) {
            (Value::String(context), Value::String(value)) => {
                if self.ignore_case {
                    Ok(context.to_lowercase().starts_with(&value.to_lowercase()))
                } else {
                    Ok(context.starts_with(value))
                }
            }
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{json_path::JSONPath, predicate::first_order::start::Start};

    #[test]
    fn snapshot_test() {
        let start_expect = serde_json::json!({
             "op": "start",
             "path": "/a/b",
             "value": " is a "
        });

        let start = Start {
            path: Some(JSONPath::new("/a/b".to_string()).unwrap()),
            ignore_case: false,
            value: serde_json::Value::String(" is a ".to_string()),
        };

        assert_eq!(serde_json::to_value(start).unwrap(), start_expect);
    }

    #[test]
    fn deser_test() {
        let start_expect = serde_json::json!({
             "op": "start",
             "path": "/a/b",
             "value": " is a "
        });

        let start = Start {
            path: Some(JSONPath::new("/a/b".to_string()).unwrap()),
            ignore_case: false,
            value: serde_json::Value::String(" is a ".to_string()),
        };

        let deser = Start::deserialize(start_expect).unwrap();

        assert_eq!(start, deser);
    }
}
