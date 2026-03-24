use log::error;
use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStateError};
use crate::api::steps::{Step, SyncStep};

pub(super) fn update(
    mut execution_state: DefaultExecutionState,
    step: Step,
) -> Result<DefaultExecutionState, ExecutionStateError> {
    if execution_state.step_states.is_empty() {
        error!("Attempt to transition a step on an empty execution_state step list");
        return Err(ExecutionStateError::InvalidStepTransition);
    }

    if execution_state.is_stopped() {
        return Err(ExecutionStateError::TransitionOnClosedExecutionState);
    }

    match execution_state.step_states.iter_mut().find(|s| s.id() == step.id()) {
        Some(existing) => {
            *existing = step;  // replace in place
            Ok(execution_state)
        }
        None => {
            error!("Attempt to transition a step that does not exist");
            Err(ExecutionStateError::InvalidStepTransition)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::steps::StepCore;
    use super::*;

    fn sync_core(id: &str) -> StepCore {
        StepCore { id: id.to_string(), kind: "alpha".to_string(), config: None }
    }
    #[test]
    fn updating_finished_step_error() {
        let execution_state = DefaultExecutionState {
            step_states: vec![Step::Sync(SyncStep::Completed { core: sync_core("1"), input: None, output: None })],
        };

        let result = update(
            execution_state,
            Step::Sync(SyncStep::Ready { core: sync_core("1"), input: None }),
        );
        assert!(result.is_err());
    }
}