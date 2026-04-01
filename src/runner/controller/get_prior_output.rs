use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::activities::{AsyncActivity, Activity, SyncActivity};
use serde_json::Value;

pub fn resolve_prior_output(
    execution_state: &DefaultExecutionState,
    activity_id: &str,
) -> Option<Value> {
    let activities: Vec<&Activity> = execution_state.activities().collect();
    let pos = activities.iter().position(|s| s.id() == activity_id)?;
    if pos == 0 {
        return None;
    }
    match activities[pos - 1] {
        Activity::Sync(SyncActivity::Completed(sc)) => sc.completed.output.clone(),
        Activity::Async(AsyncActivity::Completed(ac)) => ac.completed.output.clone(),
        _ => None,
    }
}
