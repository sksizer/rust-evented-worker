use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::activities::{AsyncActivity, Activity, SyncActivity};
use serde_json::Value;

/// Returns the output of the first dependency of the given activity.
pub fn resolve_prior_output(
    execution_state: &DefaultExecutionState,
    activity_id: &str,
) -> Option<Value> {
    let activity = execution_state.get_activity_state(activity_id)?;
    let dep_id = activity.core().depends_on.as_ref()?.first()?;
    let dep = execution_state.get_activity_state(dep_id)?;
    match dep {
        Activity::Sync(SyncActivity::Completed(sc)) => sc.completed.output.clone(),
        Activity::Async(AsyncActivity::Completed(ac)) => ac.completed.output.clone(),
        _ => None,
    }
}
