use crate::api::execution::DefaultExecutionState;
use crate::api::activities::{AsyncActivity, Activity, SyncActivity};
use serde_json::Value;

pub fn resolve_prior_output(
    execution_state: &DefaultExecutionState,
    activity_id: &str,
) -> Option<Value> {
    let pos = execution_state
        .activity_states
        .iter()
        .position(|s| s.id() == activity_id)?;
    if pos == 0 {
        return None;
    }
    match &execution_state.activity_states[pos - 1] {
        Activity::Sync(SyncActivity::Completed(sc)) => sc.completed.output.clone(),
        Activity::Async(AsyncActivity::Completed(ac)) => ac.completed.output.clone(),
        _ => None,
    }
}
