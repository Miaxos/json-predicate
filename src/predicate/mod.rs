use serde::{Deserialize, Serialize};

pub mod first_order;
use first_order::FirstOrder;
use serde_json::Value;

use self::{context::PredicateContext, error::PredicateError, second_order::SecondOrder};

pub mod context;
pub mod second_order;

mod error;

/// A Predicate is an Object whose members describe a testable
/// condition that evaluates as either true or false.
///
/// The essential components of a Predicate include:
///
/// - A label identifying the predicate operation,
/// - A reference to the value being tested, and
/// - The condition against which the referenced value is to be evaluated.
///
/// Predicate objects MUST have exactly one "op" member whose value
/// indicates the type of predicate operation to perform.  It's value
/// MUST be one of: "and", "contains", "contains-", "defined", "ends",
/// "ends-", "in", "in-", "less", "matches", "matches-", "more", "not",
/// "or", "starts", "starts-", "test", "test-", "type", or "undefined".
///
/// The semantics for each are defined in the sections that follow.
///
/// Note that the value of the "op" member is case-sensitive and that
/// each of the operations listed are in lower-case.  The value "Starts",
/// for example, is not equivalent to "starts".
///
/// If the "op" member specifies any value other than one of those listed
/// above, the evaluation of the predicate operation MUST cease and be
/// handled as if a boolean value of "false" was returned.  The
/// application processing the predicate operation MAY signal that an
/// error condition has occurred depending on the specific requirements
/// of the application within which JSON Predicates are being used.
///
/// The remaining structure of each predicate operation depends on the
/// specific type.  There are two basic types of predicates.
///
/// -  First Order Predicates that are used to test one name value pair
///    against a single condition, and
///
/// -  Second Order Predicates that aggregate one or more subordinate
///    First or Second Order Predicates.
///
/// In addition to the required "op" member, First Order Predicates have
/// exactly one "path" member whose value MUST be a string containing a
/// JSON-Pointer [RFC6901] value referencing the name value pair that is
/// to be tested.  If the "path" member is not specified within the
/// predicate object, it's value is assumed to be an empty string.
///
/// Second Order Predicates MUST have exactly one "apply" member whose
/// value is a JSON Array containing one or more First or Second Order
/// Predicate Objects.
///
///
/// Additional members can be required depending on the specific
/// predicate operation.  All other members not explicitly defined by
/// this specification MUST be ignored.
///
/// Note that the ordering of members in JSON objects is not significant;
/// therefore the following operations are equivalent:
///
/// ```json
/// {"op": "contains", "path": "/a/b/c", "value": "ABC"}
/// {"path": "/a/b/c", "op": "contains", "value": "ABC"}
/// {"value": "ABC", "path": "/a/b/c", "op": "contains"}
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum Predicate {
    FirstOrder(FirstOrder),
    SecondOrder(SecondOrder),
}

impl PredicateImpl for Predicate {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        match self {
            Predicate::FirstOrder(fo) => fo.evaluate(data, ctx),
            Predicate::SecondOrder(fo) => fo.evaluate(data, ctx),
        }
    }
}

pub trait PredicateImpl {
    /// Evaluate the predicate against the provided JSON
    ///
    /// The error result means the predicate couldn't be evaluated properly so
    /// you have to assume it's a false result and you'll also have more information about why it
    /// didn't validate.
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError>;
}
