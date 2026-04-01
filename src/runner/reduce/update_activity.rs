use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStateError};
use crate::api::activities::Activity;
use log::error;

pub(super) fn update(
    mut execution_state: DefaultExecutionState,
    activity: Activity,
) -> Result<DefaultExecutionState, ExecutionStateError> {
    if execution_state.activity_count() == 0 {
        error!("Attempt to transition an activity on an empty execution_state activity list");
        return Err(ExecutionStateError::InvalidActivityTransition);
    }

    if execution_state.is_stopped() {
        return Err(ExecutionStateError::TransitionOnClosedExecutionState);
    }

    if execution_state.replace_activity(activity) {
        Ok(execution_state)
    } else {
        error!("Attempt to transition an activity that does not exist");
        Err(ExecutionStateError::InvalidActivityTransition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activities::{
        CompletedActivity, RanActivity, ActivityCore, SyncCompleted, SyncNew, SyncActivity,
    };
    use chrono::Utc;

    fn sync_core(id: &str) -> ActivityCore {
        ActivityCore {
            id: id.to_string(),
            kind: "alpha".to_string(),
            config: None,
            depends_on: None,
        }
    }
    #[test]
    fn updating_finished_activity_error() {
        let completed = SyncCompleted {
            core: sync_core("1"),
            completed: CompletedActivity {
                ran: RanActivity {
                    started_at: Utc::now(),
                    input: None,
                },
                output: None,
            },
        };
        let execution_state = DefaultExecutionState::new(
            Some(vec![Activity::from(SyncActivity::from(completed))]));


        let ready = SyncNew::new(sync_core("1")).make_ready(None);
        let result = update(
            execution_state,
            Activity::from(SyncActivity::from(ready)),
        );
        assert!(result.is_err());
    }
}
