use chrono::{DateTime, NaiveDate, NaiveTime};
use derive_builder::Builder;
use fluent_langneg::{LanguageIdentifier, LangugeIdentifierParserError};
use langtag::LanguageTag;
use serde_json::Value;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{ser::SerializeStruct, Serialize};

use crate::json_path::JSONPath;
use crate::predicate::context::PredicateContext;
use crate::predicate::error::PredicateError;
use crate::predicate::PredicateImpl;

use super::FirstOrder;

/// The "type" predicate evaluates as true if the referenced element exists and
/// specifies a value whose value type is equal to that specified by the
/// predicate's "value" member.
/// The "value" member MUST specify one of:
/// - "number"
/// - "string"
/// - "boolean"
/// - "object"
/// - "array"
/// - "null"
/// - "undefined"
/// - "date"
/// - "date-time"
/// - "time"
/// - "lang"
/// - "lang-range"
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Type {
    /// Must be a [JSON Pointer](https://tools.ietf.org/html/rfc6901)
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    pub path: Option<JSONPath>,
    pub value: String,
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Type", 3)?;
        state.serialize_field("op", "type")?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

pub fn parse_lang_range(s: &str) -> Result<Vec<LanguageIdentifier>, LangugeIdentifierParserError> {
    s.split(',')
        .flat_map(|t| t.trim().split(';').next())
        .filter(|t| !t.is_empty())
        .map(|t| t.parse())
        .collect::<Result<Vec<LanguageIdentifier>, LangugeIdentifierParserError>>()
}

impl<'de> Deserialize<'de> for Type {
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

        struct VisitorType<'de> {
            marker: PhantomData<Type>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorType<'de> {
            type Value = Type;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Type")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<Option<JSONPath>> = None;
                let mut value: Option<String> = None;
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
                            value = Some(MapAccess::next_value::<String>(&mut map)?);
                        }
                        Field::__ignore => {}
                    }
                }

                let path = path.ok_or(serde::de::Error::missing_field("path"))?;
                let value = value.ok_or(serde::de::Error::missing_field("value"))?;
                let op = op.ok_or(serde::de::Error::missing_field("op"))?;

                if op.as_str() != "type" {
                    return Err(serde::de::Error::custom("`op` should be `type`"));
                }

                Ok(Type { path, value })
            }
        }

        const FIELDS: &[&str] = &["path", "op", "value"];
        Deserializer::deserialize_struct(
            deserializer,
            "Type",
            FIELDS,
            VisitorType {
                marker: PhantomData::<Type>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Type> for FirstOrder {
    fn from(value: Type) -> Self {
        FirstOrder::Type(value)
    }
}

impl PredicateImpl for Type {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path).unwrap_or(JSONPath::empty());
        let ptr = path.take();

        let value = &self.value;
        let context_value = ptr.get(data);

        //   The "value" member MUST specify one of: "number", "string",
        //   "boolean", "object", "array", "null", "undefined", "date",
        //   "date- time", "time", "lang", "lang-range",
        //   "iri" or "absolute-iri".
        match (context_value, value) {
            (Err(err), value) => match value.as_str() {
                "undefined" => Ok(true),
                _ => Err(err.into()),
            },
            (Ok(context), value) => match value.as_str() {
                "number" => Ok(context.is_number()),
                "string" => Ok(context.is_string()),
                "boolean" => Ok(context.is_boolean()),
                "object" => Ok(context.is_object()),
                "array" => Ok(context.is_array()),
                "null" => Ok(context.is_null()),
                "date" => match context {
                    Value::String(possible_date) => Ok(NaiveDate::from_str(possible_date).is_ok()),
                    _ => Ok(false),
                },
                "time" => match context {
                    Value::String(possible_date) => {
                        let possible_time = NaiveTime::parse_from_str(possible_date, "%H:%M:%S%Z");
                        Ok(possible_time.is_ok())
                    }
                    _ => Ok(false),
                },
                "date-time" => match context {
                    Value::String(possible_date) => {
                        Ok(DateTime::parse_from_rfc3339(possible_date).is_ok())
                    }
                    _ => Ok(false),
                },
                "lang" => match context {
                    Value::String(possible_lang) => Ok(LanguageTag::parse(possible_lang).is_ok()),
                    _ => Ok(false),
                },
                "lang-range" => match context {
                    Value::String(possible_lang_range) => {
                        // TODO: Support err return
                        let a = parse_lang_range(possible_lang_range);
                        dbg!(&a);
                        Ok(a.is_ok())
                    }
                    _ => Ok(false),
                },
                "iri" => Err(PredicateError::unimplemented()),
                "absolute-iri" => Err(PredicateError::unimplemented()),
                _ => Ok(context == value),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{json_path::JSONPath, predicate::first_order::r#type::Type};

    #[test]
    fn snapshot_test() {
        let ty_expect = serde_json::json!({
             "op": "type",
             "path": "/a/b",
             "value": "string"
        });

        let ty = Type {
            path: Some(JSONPath::new("/a/b").unwrap()),
            value: "string".to_string(),
        };

        assert_eq!(serde_json::to_value(ty).unwrap(), ty_expect);
    }

    #[test]
    fn deser_test() {
        let ty_expect = serde_json::json!({
             "op": "type",
             "path": "/a/b",
             "value": "string"
        });

        let ty = Type {
            path: Some(JSONPath::new("/a/b").unwrap()),
            value: "string".to_string(),
        };

        let deser = Type::deserialize(ty_expect).unwrap();

        assert_eq!(ty, deser);
    }
}
