//! Solely responsible for calling a step function
use crate::api::events::{Event, SystemErrorData};
use crate::api::steps::{StepConfig, StepEvent, StepInput};
use crate::runner::registry::Registry;
use log::{error, trace};
use crate::api::execution::{DefaultExecutionState, ExecutionState};

/// Knows how to call a step function. Is at the edge of side effects.
/// Takes a StepEvent (what happened), returns an Event (which may be step or system).
pub fn process(state: &DefaultExecutionState, registry: &Registry, step_event: &StepEvent) -> Event {
    trace!("executor - event: {:?}", step_event);
    let id = step_event.step_id().to_string();
    match step_event {
        StepEvent::Start(step_id) => {
            let step = state.get_step_state(step_id).unwrap();
            let Some(handler) = registry.get_sync_module(&step.kind()) else {
                return Event::error(id, Some("Could not find step module".to_string()));
            };
            let config = StepConfig(step.config().cloned());
            let input = StepInput(step.input().cloned());
            let result = (handler.handler)(config, input);
            Event::complete(step_id.to_string(), Some(result.unwrap()))
        }
        _ => {
            Event::system_error(SystemErrorData {
                step_id: id,
                errors: vec!["Invalid event sent to processor".to_string()],
                source: "processor::process".to_string(),
            })
        }
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
