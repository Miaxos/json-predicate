use crate::Predicate;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::{
    contains::Contains, defined::Defined, end::End, less::Less, matches::Matches, more::More,
    r#in::In, r#type::Type, start::Start, test::Test, undefined::Undefined,
};

use super::{context::PredicateContext, error::PredicateError, PredicateImpl};

pub mod contains;
pub mod defined;
pub mod end;
pub mod r#in;
pub mod less;
pub mod matches;
pub mod more;
pub mod start;
pub mod test;
pub mod r#type;
pub mod undefined;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum FirstOrder {
    Contains(Contains),
    Defined(Defined),
    Undefined(Undefined),
    Start(Start),
    End(End),
    Type(Type),
    In(In),
    Test(Test),
    Matches(Matches),
    Less(Less),
    More(More),
    // Contained,
    // Intersects,
}

impl PredicateImpl for FirstOrder {
    fn evaluate(&self, data: &Value, ctx: PredicateContext) -> Result<bool, PredicateError> {
        match self {
            FirstOrder::Contains(contains) => contains.evaluate(data, ctx),
            FirstOrder::Defined(defined) => defined.evaluate(data, ctx),
            FirstOrder::Undefined(undefined) => undefined.evaluate(data, ctx),
            FirstOrder::Start(start) => start.evaluate(data, ctx),
            FirstOrder::End(end) => end.evaluate(data, ctx),
            FirstOrder::Type(ty) => ty.evaluate(data, ctx),
            FirstOrder::In(ty) => ty.evaluate(data, ctx),
            FirstOrder::Test(ty) => ty.evaluate(data, ctx),
            FirstOrder::Matches(ty) => ty.evaluate(data, ctx),
            FirstOrder::Less(ty) => ty.evaluate(data, ctx),
            FirstOrder::More(ty) => ty.evaluate(data, ctx),
        }
    }
}

impl From<FirstOrder> for Predicate {
    fn from(value: FirstOrder) -> Self {
        Predicate::FirstOrder(value)
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
