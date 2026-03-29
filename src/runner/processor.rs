//! Solely responsible for calling an activity function
use crate::api::events::{Event, SystemErrorData};
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::activities::{ActivityConfig, ActivityEvent, ActivityInput};
use crate::runner::registry::Registry;
use log::{error, trace};

/// Knows how to call an activity function. Is at the edge of side effects.
/// Takes an ActivityEvent (what happened), returns an Event (which may be activity or system).
pub fn process(
    state: &DefaultExecutionState,
    registry: &Registry,
    activity_event: &ActivityEvent,
) -> Event {
    trace!("executor - event: {:?}", activity_event);
    let id = activity_event.activity_id().to_string();
    match activity_event {
        ActivityEvent::Start(activity_id) => {
            let activity = state.get_activity_state(activity_id).unwrap();
            let Some(handler) = registry.get_sync_module(&activity.kind()) else {
                return Event::error(id, Some("Could not find activity module".to_string()));
            };
            let config = ActivityConfig(activity.config().cloned());
            let input = ActivityInput(activity.input().cloned());
            let result = (handler.handler)(config, input);
            Event::complete(activity_id.to_string(), Some(result.unwrap()))
        }
        _ => Event::system_error(SystemErrorData {
            activity_id: id,
            errors: vec!["Invalid event sent to processor".to_string()],
            source: "processor::process".to_string(),
        }),
    }
}

fn invariant_violation(id: &str, message: &str) -> Event {
    error!("Executor invariant violation {} {}", id, message);
    Event::error(id.to_string(), Some(message.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_executor() {}
}
