use derive_builder::Builder;
use serde_json::{Number, Value};
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{ser::SerializeStruct, Serialize};

use crate::json_path::JSONPath;
use crate::predicate::context::PredicateContext;
use crate::predicate::error::PredicateError;
use crate::predicate::PredicateImpl;

use super::FirstOrder;

/// The "less" predicate evaluates as true if the referenced element is defined
/// and specifies a number whose value is less than that specified by the
/// predicate object's "value" member.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Less {
    /// Must be a [JSON Pointer](https://tools.ietf.org/html/rfc6901)
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    pub path: Option<JSONPath>,
    pub value: Number,
}

impl Serialize for Less {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Less", 3)?;
        state.serialize_field("op", "less")?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Less {
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

        struct VisitorLess<'de> {
            marker: PhantomData<Less>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorLess<'de> {
            type Value = Less;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Less")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<Option<JSONPath>> = None;
                let mut value: Option<Number> = None;
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
                            value = Some(MapAccess::next_value::<Number>(&mut map)?);
                        }
                        Field::__ignore => {}
                    }
                }

                let path = path.ok_or(serde::de::Error::missing_field("path"))?;
                let value = value.ok_or(serde::de::Error::missing_field("value"))?;
                let op = op.ok_or(serde::de::Error::missing_field("op"))?;

                if op.as_str() != "less" {
                    return Err(serde::de::Error::custom("`op` should be `less`"));
                }

                Ok(Less { path, value })
            }
        }

        const FIELDS: &[&str] = &["path", "op", "value"];
        Deserializer::deserialize_struct(
            deserializer,
            "Less",
            FIELDS,
            VisitorLess {
                marker: PhantomData::<Less>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Less> for FirstOrder {
    fn from(value: Less) -> Self {
        FirstOrder::Less(value)
    }
}

impl PredicateImpl for Less {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path).unwrap_or(JSONPath::empty());
        let ptr = path.take();

        let value = &self.value;
        let context_value = ptr.get(data)?;

        if context_value.is_number() {
            Ok(context_value.as_f64() < value.as_f64())
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::Number;

    use crate::{json_path::JSONPath, predicate::first_order::less::Less};

    #[test]
    fn snapshot_test() {
        let less_expect = serde_json::json!({
             "op": "less",
             "path": "/a/b",
             "value": 12
        });

        let less = Less {
            path: Some(JSONPath::new("/a/b").unwrap()),
            value: Number::from(12),
        };

        assert_eq!(serde_json::to_value(less).unwrap(), less_expect);
    }

    #[test]
    fn deser_test() {
        let less_expect = serde_json::json!({
             "op": "less",
             "path": "/a/b",
             "value": 12
        });

        let less = Less {
            path: Some(JSONPath::new("/a/b").unwrap()),
            value: Number::from(12),
        };

        let deser = Less::deserialize(less_expect).unwrap();

        assert_eq!(less, deser);
    }
}
