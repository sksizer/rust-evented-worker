use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStateError};
use crate::api::steps::Step;
use log::error;

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

    match execution_state
        .step_states
        .iter_mut()
        .find(|s| s.id() == step.id())
    {
        Some(existing) => {
            *existing = step; // replace in place
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
    use super::*;
    use crate::api::steps::{StepCore, SyncCompleted, SyncNew, SyncStep, CompletedStep, RanStep};
    use chrono::Utc;

    fn sync_core(id: &str) -> StepCore {
        StepCore {
            id: id.to_string(),
            kind: "alpha".to_string(),
            config: None,
        }
    }
    #[test]
    fn updating_finished_step_error() {
        let completed = SyncCompleted {
            core: sync_core("1"),
            completed: CompletedStep {
                ran: RanStep { started_at: Utc::now(), input: None },
                output: None,
            },
        };
        let execution_state = DefaultExecutionState {
            step_states: vec![Step::from(SyncStep::from(completed))],
        };

        let ready = SyncNew::new(sync_core("1")).make_ready(None);
        let result = update(
            execution_state,
            Step::from(SyncStep::from(ready)),
        );
        assert!(result.is_err());
    }
}
