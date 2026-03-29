use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStateError};
use crate::api::activities::Activity;

pub fn append_activity_state(
    mut execution_state: DefaultExecutionState,
    activity: Activity,
) -> Result<DefaultExecutionState, ExecutionStateError> {
    // Check for duplicate activity id
    if execution_state
        .get_activity_state(activity.id())
        .is_some()
    {
        return Err(ExecutionStateError::DuplicateActivityIdError);
    }

    if !execution_state.activity_states.is_empty() {
        let prior_id =
            execution_state.activity_states[execution_state.activity_states.len() - 1]
                .id()
                .to_string();
        if prior_id == activity.id() {
            return Err(ExecutionStateError::DuplicateActivityIdError);
        }
    }

    // If empty activity list
    execution_state.activity_states.push(activity);
    Ok(execution_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activities::{ActivityCore, SyncNew, SyncActivity};

    fn make_ready_activity(id: &str, kind: &str) -> Activity {
        let core = ActivityCore {
            id: id.to_string(),
            kind: kind.to_string(),
            config: None,
        };
        Activity::from(SyncActivity::from(SyncNew::new(core).make_ready(None)))
    }

    #[test]
    fn append_activity_state_with_duplicate_id_error() {
        let execution_state = DefaultExecutionState {
            activity_states: vec![make_ready_activity("1", "alpha")],
        };
        let result =
            append_activity_state(execution_state, make_ready_activity("1", "beta"));
        assert!(result.is_err());
    }
}
