use petgraph::Graph;
use crate::api::activities::{Activity, ActivityId};
use crate::runner::get_execution_status;
use thiserror::Error;

pub trait ExecutionState {
    /// CONSTRUCTORS
    fn new() -> Self;

    /// *INSTANCE*
    fn status(&self) -> ExecutionStatus;
    fn is_stopped(&self) -> bool;

    fn get_activity_state(&self, id: &str) -> Option<&Activity>;
    fn activities(&self) -> impl Iterator<Item = &Activity>;
    fn activity_count(&self) -> usize;
}

enum ExecutionGraphEdge {
    DependsOn
}

pub struct DefaultExecutionState {
    activity_graph: Graph<ActivityId, ExecutionGraphEdge>,
    activity_states: Vec<Activity>,
}

impl DefaultExecutionState {
    pub fn new(activity_states: Option<Vec<Activity>>) -> Self {
        DefaultExecutionState {
            activity_states: activity_states.unwrap_or_default(),
            activity_graph: Graph::new(),
        }
    }

    pub(crate) fn push_activity(&mut self, activity: Activity) {
        self.activity_states.push(activity);
    }

    pub(crate) fn replace_activity(&mut self, activity: Activity) -> bool {
        match self.activity_states.iter_mut().find(|s| s.id() == activity.id()) {
            Some(existing) => {
                *existing = activity;
                true
            }
            None => false,
        }
    }
}

impl ExecutionState for DefaultExecutionState {
    fn new() -> DefaultExecutionState {
        DefaultExecutionState {
            activity_graph: Graph::new(),
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

    fn activities(&self) -> impl Iterator<Item = &Activity> {
        self.activity_states.iter()
    }

    fn activity_count(&self) -> usize {
        self.activity_states.len()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parents() {

    }
}