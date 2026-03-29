use crate::api::activities::Activity;
use crate::runner::get_execution_status;
use thiserror::Error;

pub trait ExecutionState {
    fn new() -> Self;
    fn status(&self) -> ExecutionStatus;
    fn is_stopped(&self) -> bool;

    fn get_activity_state(&self, id: &str) -> Option<&Activity>;
}

pub struct DefaultExecutionState {
    // todo - make this private to enforce valid transitions
    pub activity_states: Vec<Activity>,
}

impl ExecutionState for DefaultExecutionState {
    fn new() -> DefaultExecutionState {
        DefaultExecutionState {
            activity_states: vec![],
        }
    }
    fn status(&self) -> ExecutionStatus {
        get_execution_status(self)
    }

    fn is_stopped(&self) -> bool {
        matches!(
            self.status(),
            ExecutionStatus::Error | ExecutionStatus::Failed | ExecutionStatus::Finished
        )
    }

    fn get_activity_state(&self, id: &str) -> Option<&Activity> {
        self.activity_states.iter().find(|s| s.id() == id)
    }
}

#[derive(Error, Debug)]
pub enum ExecutionStateError {
    #[error("Attempt to transition on closed execution state")]
    TransitionOnClosedExecutionState,

    #[error("An activity with a duplicate id was appended")]
    DuplicateActivityIdError,

    #[error("Invalid activity transition")]
    InvalidActivityTransition,

    #[error("Invalid activity transition on closed activity")]
    InvalidActivityTransitionOnClosedActivity,
}

#[derive(Debug, PartialEq)]
pub enum ExecutionStatus {
    New, // No activities established or any other state
    Error,
    Failed,
    Running,
    Finished,
}
