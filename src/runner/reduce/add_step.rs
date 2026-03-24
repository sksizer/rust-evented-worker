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
    use crate::api::steps::{AsyncStep, SyncStep};
    use super::*;
    #[test]
    fn append_step_state_with_duplicate_id_error() {
        let execution_state = DefaultExecutionState {
            step_states: vec![Step::Sync(crate::api::steps::SyncStep::Ready { core: crate::api::steps::StepCore { id: "1".to_string(), kind: "alpha".to_string(), config: None }, input: None })],
        };
        let result = append_step_state(
            execution_state,
            Step::Sync(crate::api::steps::SyncStep::Ready { core: crate::api::steps::StepCore { id: "1".to_string(), kind: "beta".to_string(), config: None }, input: None }),
        );
        assert!(result.is_err());
    }

}