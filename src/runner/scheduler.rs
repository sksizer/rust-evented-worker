use crate::api::execution::{DefaultExecutionState, ExecutionState};

/// Given an execution state (and later a system state) this determines what activity to run next.
pub fn scheduler(execution_state: &DefaultExecutionState) -> Option<String> {
    execution_state.activities().find(|activity| activity.is_runnable()).map(|activity| activity.id().to_string())
}
