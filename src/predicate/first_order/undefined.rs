use derive_builder::Builder;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{ser::SerializeStruct, Serialize};
use serde_json::Value;

use crate::context::PredicateContext;
use crate::json_path::JSONPath;
use crate::predicate::error::PredicateError;
use crate::{FirstOrder, PredicateImpl};

#[derive(Debug, Builder, Clone, PartialEq, Eq)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Undefined {
    /// Must be a [JSON Pointer](https://tools.ietf.org/html/rfc6901)
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    pub path: Option<JSONPath>,
}

impl Serialize for Undefined {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Undefined", 2)?;
        state.serialize_field("op", "undefined")?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Undefined {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            op,
            path,
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

        struct VisitorUndefined<'de> {
            marker: PhantomData<Undefined>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorUndefined<'de> {
            type Value = Undefined;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Undefined")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<Option<JSONPath>> = None;
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
                        Field::__ignore => {}
                    }
                }

                let path = path.ok_or(serde::de::Error::missing_field("path"))?;
                let op = op.ok_or(serde::de::Error::missing_field("op"))?;

                if op != "undefined" {
                    return Err(serde::de::Error::custom("`op` should be `undefined`"));
                }

                Ok(Undefined { path })
            }
        }

        const FIELDS: &[&str] = &["path", "op"];
        Deserializer::deserialize_struct(
            deserializer,
            "Undefined",
            FIELDS,
            VisitorUndefined {
                marker: PhantomData::<Undefined>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Undefined> for FirstOrder {
    fn from(value: Undefined) -> Self {
        FirstOrder::Undefined(value)
    }
}

impl PredicateImpl for Undefined {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path).unwrap_or(JSONPath::empty());
        let ptr = path.take();

        Ok(ptr.get(data).is_err())
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{json_path::JSONPath, predicate::first_order::undefined::Undefined};

    #[test]
    fn snapshot_test() {
        let undefined_expect = serde_json::json!({
             "op": "undefined",
             "path": "/a/b",
        });

        let undefined = Undefined {
            path: Some(JSONPath::new("/a/b").unwrap()),
        };

        assert_eq!(serde_json::to_value(undefined).unwrap(), undefined_expect);
    }

    #[test]
    fn deser_test() {
        let undefined_expect = serde_json::json!({
             "op": "undefined",
             "path": "/a/b",
        });

        let undefined = Undefined {
            path: Some(JSONPath::new("/a/b").unwrap()),
        };

        let deser = Undefined::deserialize(undefined_expect).unwrap();

        assert_eq!(undefined, deser);
    }
}
