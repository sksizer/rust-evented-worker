use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStateError};
use crate::api::activities::{Activity, SyncActivity, SyncNew};

pub fn append_activity_state(
    mut execution_state: DefaultExecutionState,
    activity: Activity,
) -> Result<DefaultExecutionState, ExecutionStateError> {
    if execution_state
        .get_activity_state(activity.id())
        .is_some()
    {
        return Err(ExecutionStateError::DuplicateActivityIdError);
    }

    if let Some(deps) = &activity.core().depends_on {
        if deps.contains(&activity.id().to_string()) {
            return Err(ExecutionStateError::SelfReferentialDependency);
        }
    }

    execution_state
        .activity_to_graph_map
        .insert(activity.id().to_string(), activity);
    Ok(execution_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activities::{ActivityCore, SyncActivity, SyncNew};

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

    fn make_new_activity_with_deps(id: &str, kind: &str, depends_on: Vec<String>) -> Activity {
        let core = ActivityCore {
            id: id.to_string(),
            kind: kind.to_string(),
            config: None,
            depends_on: Some(depends_on),
        };
        Activity::from(SyncActivity::from(SyncNew::new(core)))
    }

    #[test]
    fn append_activity_state_with_dependents() {
        let execution_state = DefaultExecutionState::new(
            Some(vec![make_ready_activity("1", "alpha")]));
        let activity = make_new_activity_with_deps("2", "beta", vec!["1".into()]);
        let result = append_activity_state(execution_state, activity);
        assert!(result.is_ok());
    }

    #[test]
    fn self_referential_dependency_error() {
        let execution_state = DefaultExecutionState::new(None);
        let activity = make_new_activity_with_deps("1", "alpha", vec!["1".into()]);
        let result = append_activity_state(execution_state, activity);
        assert!(matches!(
            result,
            Err(ExecutionStateError::SelfReferentialDependency)
        ));
    }
}
