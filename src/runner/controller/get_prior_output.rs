use crate::api::execution::DefaultExecutionState;
use crate::api::steps::{AsyncStep, Step, SyncStep};
use serde_json::Value;

pub fn resolve_prior_output(
    execution_state: &DefaultExecutionState,
    step_id: &str,
) -> Option<Value> {
    let pos = execution_state
        .step_states
        .iter()
        .position(|s| s.id() == step_id)?;
    if pos == 0 {
        return None;
    }
    match &execution_state.step_states[pos - 1] {
        Step::Sync(SyncStep::Completed(sc)) => sc.completed.output.clone(),
        Step::Async(AsyncStep::Completed(ac)) => ac.completed.output.clone(),
        _ => None,
    }
}
