//! Solely responsible for calling an activity function
use crate::api::activities::ActivityEvent;
use crate::api::events::{Event, SystemErrorData};
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::runner::registry::Registry;
use log::trace;
use serde_json::Value;

/// Knows how to call an activity function. Is at the edge of side effects.
/// Takes an ActivityEvent (what happened), returns an Event (which may be activity or system).
pub fn process(state: &DefaultExecutionState, registry: &Registry, activity_event: &ActivityEvent) -> Event {
    trace!("executor - event: {:?}", activity_event);
    let id = activity_event.activity_id().to_string();
    match activity_event {
        ActivityEvent::Start(activity_id) => {
            let activity = state.get_activity_state(activity_id).unwrap();
            let kind = activity.kind();

            let Some(module) = registry.get_module(kind) else {
                return Event::error(id, Some("Could not find activity module".to_string()));
            };

            let config = activity.config().cloned().unwrap_or(Value::Null);
            let input = activity.input().cloned().unwrap_or(Value::Null);

            match (module.execute)(&config, &input) {
                Ok(result) => Event::complete(activity_id.to_string(), Some(result)),
                Err(errors) => Event::error(activity_id.to_string(), Some(errors.join(","))),
            }
        }
        _ => Event::system_error(SystemErrorData {
            activity_id: id,
            errors: vec!["Invalid event sent to processor".to_string()],
            source: "processor::process".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_executor() {}
}
