//! Partial implementation of [Snell Json Predicate Draft 07](https://tools.ietf.org/html/draft-snell-json-test-07).
pub mod json_path;

mod predicate;
mod regex;

pub use predicate::context;
pub use predicate::first_order::FirstOrder;
pub use predicate::second_order::SecondOrder;
pub use predicate::{Predicate, PredicateImpl};

pub mod builder {
    pub use crate::predicate::first_order::contains::{ContainsBuilder, ContainsBuilderError};
    pub use crate::predicate::first_order::defined::{DefinedBuilder, DefinedBuilderError};
    pub use crate::predicate::first_order::end::{EndBuilder, EndBuilderError};
    pub use crate::predicate::first_order::less::{LessBuilder, LessBuilderError};
    pub use crate::predicate::first_order::matches::{MatchesBuilder, MatchesBuilderError};
    pub use crate::predicate::first_order::more::{MoreBuilder, MoreBuilderError};
    pub use crate::predicate::first_order::r#in::{InBuilder, InBuilderError};
    pub use crate::predicate::first_order::r#type::{TypeBuilder, TypeBuilderError};
    pub use crate::predicate::first_order::start::{StartBuilder, StartBuilderError};
    pub use crate::predicate::first_order::test::{TestBuilder, TestBuilderError};
    pub use crate::predicate::first_order::undefined::{UndefinedBuilder, UndefinedBuilderError};
    pub use crate::predicate::second_order::and::{AndBuilder, AndBuilderError};
    pub use crate::predicate::second_order::not::{NotBuilder, NotBuilderError};
    pub use crate::predicate::second_order::or::{OrBuilder, OrBuilderError};
}
