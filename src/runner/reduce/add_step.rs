use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStateError};
use crate::api::steps::Step;

pub fn append_step_state(
    mut execution_state: DefaultExecutionState,
    step: Step,
) -> Result<DefaultExecutionState, ExecutionStateError> {
    // Check for duplicate step id
    if execution_state.get_step_state(step.id()).is_some() {
        return Err(ExecutionStateError::DuplicateStepIdError);
    }

    if !execution_state.step_states.is_empty() {
        let prior_id = execution_state.step_states[execution_state.step_states.len() - 1]
            .id()
            .to_string();
        if prior_id == step.id() {
            return Err(ExecutionStateError::DuplicateStepIdError);
        }
    }

    // If empty step list
    execution_state.step_states.push(step);
    Ok(execution_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::steps::{StepCore, SyncNew, SyncStep};

    fn make_ready_step(id: &str, kind: &str) -> Step {
        let core = StepCore {
            id: id.to_string(),
            kind: kind.to_string(),
            config: None,
        };
        Step::from(SyncStep::from(SyncNew::new(core).make_ready(None)))
    }

    #[test]
    fn append_step_state_with_duplicate_id_error() {
        let execution_state = DefaultExecutionState {
            step_states: vec![make_ready_step("1", "alpha")],
        };
        let result = append_step_state(
            execution_state,
            make_ready_step("1", "beta"),
        );
        assert!(result.is_err());
    }
}
