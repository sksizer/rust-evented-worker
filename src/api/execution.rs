mod model;
mod reduce;

use crate::api::activities::ActivityEvent;
pub use model::ExecutionState;
pub use model::*;

pub type Reducer = fn(DefaultExecutionState, &ActivityEvent) -> DefaultExecutionState;
