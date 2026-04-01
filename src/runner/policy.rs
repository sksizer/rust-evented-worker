use crate::api::activities::ActivityEvent;
use crate::api::execution::{DefaultExecutionState, ExecutionState};

/// Checks the given activity and returns a Retry event if the policy allows it.
pub fn apply_policy(
    execution_state: &DefaultExecutionState,
    activity_id: &str,
) -> Option<ActivityEvent> {
    let activity = execution_state.get_activity_state(activity_id)?;
    if !activity.is_failed() && !activity.is_error() {
        return None;
    }
    let core = activity.core();
    if core.attempt < execution_state.max_retries() {
        Some(ActivityEvent::retry(core.id.clone()))
    } else {
        None
    }
}
