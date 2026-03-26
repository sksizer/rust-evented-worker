use crate::api::execution::DefaultExecutionState;
use crate::api::steps::{Step, StepEvent};

/// Given an execution state (and later a system state) this determines what step to take and
/// produces an event that can be persisted
pub fn scheduler(execution_state: &DefaultExecutionState) -> Option<StepEvent> {
    // Finds the next thing to run and creates an appropriate event for it
    let next_step = execution_state.step_states.iter().find(|step| step.is_runnable());
    match next_step {
        Some(step) => {
            Some(StepEvent::start(step.id().to_string()))
        }
        None => {
            return None;
        }
    }
}
