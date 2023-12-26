use json_pointer::{JsonPointer, ParseError};
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONPath(JsonPointer<String, Vec<String>>);

impl Default for JSONPath {
    fn default() -> Self {
        Self(JsonPointer::new(Vec::new()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JSONPathError {
    /// An invalid escape sequence was encountered, either a `~` escape or a
    /// `%` escape.
    #[error("An invalid escape sequence was encountered, either a `~` escape or a `%` escape.")]
    InvalidEscape,
    /// An error caused by not having a leading slash on the JSON pointer.
    ///
    /// For example, the string `a/b/c` is not a valid JSON pointer, while
    /// `/a/b/c` is.
    #[error("An error caused by not having a leading slash on the JSON pointer.")]
    NoLeadingSlash,
}

impl JSONPath {
    pub fn new<S: AsRef<str>>(value: S) -> Result<JSONPath, JSONPathError> {
        let ptr = value.as_ref().parse::<JsonPointer<_, _>>();
        match ptr {
            Ok(ptr) => Ok(JSONPath(ptr)),
            Err(ParseError::NoLeadingSlash) => Err(JSONPathError::NoLeadingSlash),
            Err(ParseError::InvalidEscape(_)) => Err(JSONPathError::InvalidEscape),
        }
    }

    pub fn empty() -> Self {
        Self::new("/").expect("Can't fail")
    }

    pub fn take(self) -> JsonPointer<String, Vec<String>> {
        self.0
    }
}

impl Serialize for JSONPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let val = self.0.to_string();
        Ok(serializer.serialize_str(&val)?)
    }
}

impl Add for JSONPath {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // TODO: Fix it because it's an unwrap that shouldn't be here as we should be sure that it
        // works.
        Self::new(format!("{}{}", self.0.to_string(), other.0.to_string())).unwrap()
    }
}

impl<'de> Deserialize<'de> for JSONPath {
    fn deserialize<D>(deserializer: D) -> Result<JSONPath, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let ptr = s
            .parse::<JsonPointer<_, _>>()
            .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
        Ok(JSONPath(ptr))
    }
}
