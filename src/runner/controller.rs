//! Stateful object that ties together processing loop with functions

use log::trace;
use serde_json::Value;
use crate::api::events::EventStream;
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::{AsyncStep, StepEvent, SyncStep};
use crate::api::steps::Step;
use crate::runner::{executor, reduce, restore, scheduler};
use crate::runner::registry::Registry;

pub struct Controller {
    registry: Registry,
    event_stream: EventStream,
}

impl Controller {
    pub fn new(registry: Registry, event_stream: Option<EventStream>) -> Controller {
        Controller {
            registry,
            event_stream: event_stream.unwrap_or_else(EventStream::new),
        }
    }

    pub fn start(&self) -> DefaultExecutionState {
        let mut execution_state = restore(&self.event_stream);

        while !execution_state.is_stopped() {
            trace!("Controller - processing");
            match scheduler(&execution_state) {
                None => break,
                Some(step) => {
                    let input = resolve_prior_output(&execution_state, step.id());
                    let start_event = StepEvent::start(step.id().to_string(), input);
                    execution_state = reduce(execution_state, &start_event);

                    let step = scheduler(&execution_state).unwrap();
                    let event = executor(&self.registry, step);
                    execution_state = reduce(execution_state, &event);
                }
            }
        }
        execution_state
    }
}

pub fn resolve_prior_output(execution_state: &DefaultExecutionState, step_id: &str) -> Option<Value> {
    let pos = execution_state.step_states.iter().position(|s| s.id() == step_id)?;
    if pos == 0 {
        return None;
    }
    match &execution_state.step_states[pos - 1] {
        Step::Sync(SyncStep::Completed { output, .. }) => output.clone(),
        Step::Async(AsyncStep::Completed { output, .. }) => output.clone(),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::api::steps::StepEvent;
    use super::*;

    #[test]
    fn test_controller_start() {
        let event_stream = vec![
            StepEvent::add_sync("1", "echo", Some(json!({ "message": "hello" }))),
        ];
        let controller = Controller::new(Registry::new(None, None), None);
        controller.start();
    }
}
