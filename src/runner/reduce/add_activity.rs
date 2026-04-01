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

    if let Some(last) = execution_state.activities().last() {
        if last.id() == activity.id() {
            return Err(ExecutionStateError::DuplicateActivityIdError);
        }
    }

    execution_state.push_activity(activity);
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
            depends_on: None,
        };
        Activity::from(SyncActivity::from(SyncNew::new(core).make_ready(None)))
    }

    #[test]
    fn append_activity_state_with_duplicate_id_error() {
        let execution_state = DefaultExecutionState::new(
            Some(vec![make_ready_activity("1", "alpha")]));

        let result =
            append_activity_state(execution_state, make_ready_activity("1", "beta"));
        assert!(result.is_err());
    }
}
