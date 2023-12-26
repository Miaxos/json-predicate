use std::error::Error;

use crate::json_path::{JSONPath, JSONPathError};

/// The context to run the evaluation, you can have an empty context with
/// `PredicateContext::default()`.
#[derive(Default, Clone, Debug)]
pub struct PredicateContext {
    location: Option<JSONPath>,
}

#[derive(Debug, thiserror::Error)]
pub enum PredicateContextError {
    #[error("{0}")]
    Deserialize(#[from] Box<dyn Error>),
    #[error("{0}")]
    JSONPath(#[from] JSONPathError),
}

impl From<JSONPath> for PredicateContext {
    fn from(value: JSONPath) -> Self {
        Self {
            location: Some(value),
        }
    }
}

impl From<Option<JSONPath>> for PredicateContext {
    fn from(value: Option<JSONPath>) -> Self {
        Self { location: value }
    }
}

impl PredicateContext {
    pub fn new(path: String) -> Result<Self, PredicateContextError> {
        let location = JSONPath::new(path)?;

        Ok(Self {
            location: Some(location),
        })
    }

    pub fn final_path(&self, path: &Option<JSONPath>) -> Option<JSONPath> {
        // TODO: Remove cloning
        match (self.location.clone(), path.clone()) {
            (None, None) => None,
            (Some(path), None) | (None, Some(path)) => Some(path),
            (Some(root), Some(path)) => Some(root + path),
        }
    }
}
