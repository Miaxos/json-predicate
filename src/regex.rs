use std::marker::PhantomData;
use std::str::FromStr;

use regex::Regex as LibRegex;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::Serialize;

/// A new type wrapping a `[regex::Regex]` with:
///
/// - Eq
/// - PartialEq
/// - Serialize
/// - Deserialize
#[derive(Clone, Debug)]
pub struct Regex(LibRegex);

impl AsRef<LibRegex> for Regex {
    fn as_ref(&self) -> &LibRegex {
        &self.0
    }
}

impl From<LibRegex> for Regex {
    fn from(value: LibRegex) -> Self {
        Self(value)
    }
}

impl From<Regex> for LibRegex {
    fn from(value: Regex) -> Self {
        value.0
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for Regex {}

impl Serialize for Regex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VisitorRegex<'de> {
            marker: PhantomData<Regex>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> Visitor<'de> for VisitorRegex<'de> {
            type Value = Regex;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Regex")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let inner_regex =
                    LibRegex::from_str(v).map_err(|err| serde::de::Error::custom(err))?;

                Ok(Regex(inner_regex))
            }
        }

        Deserializer::deserialize_str(
            deserializer,
            VisitorRegex {
                marker: PhantomData::<Regex>,
                lifetime: PhantomData,
            },
        )
    }
}
