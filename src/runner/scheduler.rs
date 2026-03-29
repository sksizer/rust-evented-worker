use crate::api::execution::DefaultExecutionState;
use crate::api::activities::ActivityEvent;

/// Given an execution state (and later a system state) this determines what activity to run and
/// produces an event that can be persisted
pub fn scheduler(execution_state: &DefaultExecutionState) -> Option<ActivityEvent> {
    let next_activity = execution_state
        .activity_states
        .iter()
        .find(|activity| activity.is_runnable());
    match next_activity {
        Some(activity) => Some(ActivityEvent::start(activity.id().to_string())),
        None => None,
    }
}
