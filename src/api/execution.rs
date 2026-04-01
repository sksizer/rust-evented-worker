mod model;
mod reduce;

pub use model::ExecutionState;
pub use model::*;
use crate::api::activities::ActivityEvent;

pub type Reducer = fn(DefaultExecutionState, &ActivityEvent ) -> DefaultExecutionState;