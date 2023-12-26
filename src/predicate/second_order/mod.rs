use crate::Predicate;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::{and::And, not::Not, or::Or};

use super::{context::PredicateContext, error::PredicateError, PredicateImpl};

pub mod and;
pub mod not;
pub mod or;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum SecondOrder {
    And(And),
    Or(Or),
    Not(Not),
}

impl PredicateImpl for SecondOrder {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        match self {
            Self::And(ty) => ty.evaluate(data, ctx),
            Self::Or(ty) => ty.evaluate(data, ctx),
            Self::Not(ty) => ty.evaluate(data, ctx),
        }
    }
}

impl From<SecondOrder> for Predicate {
    fn from(value: SecondOrder) -> Self {
        Predicate::SecondOrder(value)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::predicate::first_order::FirstOrder;

    #[test]
    fn deser_test() {
        let contains_expect = serde_json::json!({
             "op": "contains",
             "path": "/a/b",
             "value": " is a "
        });

        let deser = FirstOrder::deserialize(contains_expect).unwrap();

        insta::assert_debug_snapshot!(deser, @r###"
        Contains(
            Contains {
                path: Some(
                    JSONPath(
                        JsonPointer {
                            ref_toks: [
                                "a",
                                "b",
                            ],
                            _phantom: PhantomData<alloc::string::String>,
                        },
                    ),
                ),
                ignore_case: false,
                value: String(
                    " is a ",
                ),
            },
        )
        "###);
    }
}
