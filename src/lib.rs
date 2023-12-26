//! ## Introduction
//!
//! This specification defines JSON Predicates, a JSON-based
//! [RFC4627](https://datatracker.ietf.org/doc/html/rfc4627)
//! syntax for the description and serialization of logical boolean
//! predicate operations intended to be used in conjunction with other
//! JSON-based mechanisms, such as JSON Patch [RFC6902](https://datatracker.ietf.org/doc/html/rfc6902),
//! as a means of incorporating conditional processing.
//!
//! JSON Predicates can be used, for instance, to extend a JSON Patch
//! [RFC6902](https://datatracker.ietf.org/doc/html/rfc6902) document to provide
//! for a broader range of conditional processing options not currently supported
//! by JSON Patch.
//!
//! ## Example
//!
//! Given this JSON:
//! ```json
//! {
//!     "a": {
//!         "b": {
//!             "c": "ABC!XYZ"
//!         }
//!     }
//! }
//! ```
//!
//! We could have a predicate like this:
//! ```json
//! {
//!     "op": "and",
//!     "path": "/a/b/c",
//!     "apply": [
//!         {
//!             "op": "type",
//!             "value": "string"
//!         },
//!         {
//!             "op": "contains",
//!             "value": "ABC"
//!         }
//!     ]
//! }
//! ```
//!
//! which would evaluate as `true` if evaluated against the previous JSON.
//!
//! ## Rust Example
//!
//! ```rust
//! let predicate = Predicate::deserialize(serde_json::json!({
//!     "op": "and",
//!     "path": "/objA",
//!     "apply": [
//!       {
//!         "op": "defined",
//!         "path": "/stringX"
//!       },
//!       {
//!         "op": "defined",
//!         "path": "/stringXYZ"
//!       }
//!     ],
//! }))?;
//!
//! let evaluted_predicate: bool = predicate
//!     .test(&ENTRY, PredicateContext::default());
//! ```
//!
//! ## JSON Patch
//!
//! The JSON Patch methods described in [draft-snell-json-test-07](https://datatracker.ietf.org/doc/html/draft-snell-json-test-07)
//! are not implemented yet.
//!
//! ## Features
//!
//! ### First order predicate
//!
//! - [x] "contains"
//! - [x] "contains-"
//! - [x] "defined"
//! - [x] "undefined"
//! - [x] "starts"
//! - [x] "starts-"
//! - [x] "ends"
//! - [x] "ends-"
//! - [x] "type"
//! - [x] "in"
//! - [x] "in-"
//! - [x] "matches"
//! - [x] "matches-"
//! - [x] "test"
//! - [x] "test-"
//! - [x] "less"
//! - [x] "more"
//!
//! ### Second order predicate
//!
//! - [x] "and"
//! - [x] "not"
//! - [x] "or"
//!
//! ## References
//!
//! - [Specification draft-snell-json-test-07](https://tools.ietf.org/html/draft-snell-json-test-07)
//!
//! ## License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0, (LICENSE-APACHE or [LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
//! - MIT license (LICENSE-MIT or [MIT](http://opensource.org/licenses/MIT)) at your option.
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
