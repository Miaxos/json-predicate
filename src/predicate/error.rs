use std::fmt::Display;

use json_pointer::IndexError;

#[derive(Debug)]
pub struct PredicateError {
    kind: PredicateErrorKind,
}

impl PredicateError {
    pub fn unimplemented() -> Self {
        Self {
            kind: PredicateErrorKind::Unimplemented,
        }
    }
    pub fn ty_issue() -> Self {
        Self {
            kind: PredicateErrorKind::IncorrectType,
        }
    }
}

#[derive(Debug)]
enum PredicateErrorKind {
    Unimplemented,
    /// The Predicate Object specifies an unknown predicate operation.
    #[allow(dead_code)]
    UnknownPredicateOperation,
    /// The Predicate Object specifies a JSON Pointer referencing a value that
    /// does not exist and the specified Predicate operation is not
    /// specifically intended to test for the absence of a value
    /// (i.e. the "undefined" and "defined" predicates)
    JsonPointerIssue {
        issue: IndexError,
    },
    /// A First Order Predicate Object specifies a predicate operation that
    /// requires a "value" member providing the condition to test but no
    /// "value" member is provided.
    #[allow(dead_code)]
    NoValueMember,
    /// The "value" member given for a given predicate operation is of an
    /// unexpected or unsupported type for that operation
    /// (e.g. specifying a string value for the "more" and "less" predicate operations).
    IncorrectType,
}

impl Display for PredicateErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPredicateOperation { .. } => f.write_str("UnknownPredicateOperation"),
            Self::JsonPointerIssue { issue } => match issue {
                IndexError::NoSuchKey(key) => f.write_fmt(format_args!(
                    "The pointer pointed to a nonexistent key, pointed key: {}",
                    key
                )),
                IndexError::NotIndexable => {
                    f.write_str("The pointer resulted in trying to index a non-indexable type")
                }
                IndexError::OutOfBounds(idx) => f.write_fmt(format_args!(
                    "The pointer pointed to an out-of-bounds value in an array, pointed at {}",
                    idx
                )),
            },
            Self::NoValueMember { .. } => f.write_str("NoValueMember"),
            Self::IncorrectType { .. } => f.write_str("IncorrectType"),
            Self::Unimplemented { .. } => f.write_str("Unimplemented"),
        }
    }
}

impl std::error::Error for PredicateError {}

impl Display for PredicateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.kind))
    }
}

impl From<IndexError> for PredicateError {
    fn from(value: IndexError) -> Self {
        PredicateError {
            kind: PredicateErrorKind::JsonPointerIssue { issue: value },
        }
    }
}
