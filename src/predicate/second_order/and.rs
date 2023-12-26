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

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct And {
    /// Must be a JSON Pointer
    /// https://tools.ietf.org/html/rfc6901
    /// If the "path" member is not specified within the predicate object, it's value is assumed to be an empty string.
    #[builder(default)]
    pub path: Option<JSONPath>,
    pub apply: Vec<Predicate>,
}

impl Serialize for And {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("And", 3)?;
        state.serialize_field("op", "and")?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("apply", &self.apply)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for And {
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

        struct VisitorAnd<'de> {
            marker: PhantomData<And>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorAnd<'de> {
            type Value = And;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("And")
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

                if op.as_str() != "and" {
                    return Err(serde::de::Error::custom("`op` should be `and`"));
                }

                Ok(And { apply, path })
            }
        }

        const FIELDS: &'static [&'static str] = &["op", "apply", "path"];
        Deserializer::deserialize_struct(
            deserializer,
            "And",
            FIELDS,
            VisitorAnd {
                marker: PhantomData::<And>,
                lifetime: PhantomData,
            },
        )
    }
}

impl From<And> for SecondOrder {
    fn from(value: And) -> Self {
        SecondOrder::And(value)
    }
}

impl PredicateImpl for And {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        let path = ctx.final_path(&self.path);
        let ctx = PredicateContext::from(path);

        for ap in &self.apply {
            let pred = ap.evaluate(data, ctx.clone());

            if pred.is_ok() {
                continue;
            } else {
                return pred;
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::predicate::second_order::and::And;

    #[test]
    fn deser_test() {
        let and_expect = serde_json::json!({
             "op": "and",
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

        let deser = And::deserialize(and_expect).unwrap();

        insta::assert_json_snapshot!(deser);
    }
}
