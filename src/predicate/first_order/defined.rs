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
pub struct Defined {
    /// Must be a [JSON Pointer](https://tools.ietf.org/html/rfc6901)
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    pub path: Option<JSONPath>,
}

impl Serialize for Defined {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Defined", 2)?;
        state.serialize_field("op", "defined")?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Defined {
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

        struct VisitorDefined<'de> {
            marker: PhantomData<Defined>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorDefined<'de> {
            type Value = Defined;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Defined")
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

                if op != "defined" {
                    return Err(serde::de::Error::custom("`op` should be `defined`"));
                }

                Ok(Defined { path })
            }
        }

        const FIELDS: &[&str] = &["path", "op"];
        Deserializer::deserialize_struct(
            deserializer,
            "Defined",
            FIELDS,
            VisitorDefined {
                marker: PhantomData::<Defined>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Defined> for FirstOrder {
    fn from(value: Defined) -> Self {
        FirstOrder::Defined(value)
    }
}

impl PredicateImpl for Defined {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path).unwrap_or(JSONPath::empty());
        let ptr = path.take();

        let _context_value = ptr.get(data)?;

        // The "defined" predicate evaluates as true if the referenced element
        // exists within the target context.
        //
        // If the element doesn't exist we woulnd't be here so even if
        // it's null, it's enough to tell it exists.
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{json_path::JSONPath, predicate::first_order::defined::Defined};

    #[test]
    fn snapshot_test() {
        let defined_expect = serde_json::json!({
             "op": "defined",
             "path": "/a/b",
        });

        let defined = Defined {
            path: Some(JSONPath::new("/a/b").unwrap()),
        };

        assert_eq!(serde_json::to_value(defined).unwrap(), defined_expect);
    }

    #[test]
    fn deser_test() {
        let defined_expect = serde_json::json!({
             "op": "defined",
             "path": "/a/b",
        });

        let defined = Defined {
            path: Some(JSONPath::new("/a/b").unwrap()),
        };

        let deser = Defined::deserialize(defined_expect).unwrap();

        assert_eq!(defined, deser);
    }
}
