use derive_builder::Builder;
use serde_json::Value;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{ser::SerializeStruct, Serialize};

use crate::json_path::JSONPath;
use crate::predicate::context::PredicateContext;
use crate::predicate::error::PredicateError;
use crate::predicate::PredicateImpl;
use crate::Predicate;

use super::SecondOrder;

/// The "or" predicate evaluates as "true" if at least one of it's contained
/// set of predicate operations evaluate as "true".
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Or {
    /// Must be a [JSON Pointer](https://tools.ietf.org/html/rfc6901)
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    #[builder(default)]
    pub path: Option<JSONPath>,
    pub apply: Vec<Predicate>,
}

impl Serialize for Or {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Or", 3)?;
        state.serialize_field("op", "or")?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("apply", &self.apply)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Or {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            op,
            path,
            apply,
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
                    "apply" => Ok(Field::apply),
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

        struct VisitorOr<'de> {
            marker: PhantomData<Or>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorOr<'de> {
            type Value = Or;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Or")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<Option<JSONPath>> = None;
                let mut apply: Option<Vec<Predicate>> = None;
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
                        Field::apply => {
                            if apply.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            apply = Some(MapAccess::next_value::<Vec<Predicate>>(&mut map)?);
                        }
                        Field::__ignore => {}
                    }
                }

                let path = path.flatten();
                let apply = apply.ok_or(serde::de::Error::missing_field("apply"))?;
                let op = op.ok_or(serde::de::Error::missing_field("op"))?;

                if op.as_str() != "or" {
                    return Err(serde::de::Error::custom("`op` should be `or`"));
                }

                Ok(Or { apply, path })
            }
        }

        const FIELDS: &[&str] = &["op", "apply", "path"];
        Deserializer::deserialize_struct(
            deserializer,
            "Or",
            FIELDS,
            VisitorOr {
                marker: PhantomData::<Or>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<Or> for SecondOrder {
    fn from(value: Or) -> Self {
        SecondOrder::Or(value)
    }
}

impl PredicateImpl for Or {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path);
        let ctx = PredicateContext::from(path);

        let mut acc = true;

        for ap in &self.apply {
            let pred = ap.evaluate(data, ctx.clone());

            if pred.is_ok() {
                return Ok(true);
            } else {
                acc = false;
            }
        }

        Ok(acc)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::predicate::second_order::or::Or;

    #[test]
    fn deser_test() {
        let or_expect = serde_json::json!({
             "op": "or",
             "apply": [
               {
                 "op": "defined",
                 "path": "/a/b"
               },
               {
                 "op": "less",
                 "path": "/a/c/d",
                 "value": 15
               }
            ]
        });

        let deser = Or::deserialize(or_expect).unwrap();

        insta::assert_json_snapshot!(deser);
    }
}
