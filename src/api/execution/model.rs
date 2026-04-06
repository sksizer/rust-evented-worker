use crate::api::activities::{Activity, ActivityId};
use crate::runner::get_execution_status;
use petgraph::Graph;
use std::collections::HashMap;
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

    fn activity_dependents(&self, id: ActivityId) -> Vec<&Activity>;

    // POLICIES
    fn max_retries(&self) -> u32;
}

pub(crate) enum ExecutionGraphRelation {
    Precedes,
}

pub struct DefaultExecutionState {
    pub(crate) activity_to_graph_map: HashMap<ActivityId, Activity>,
    pub(crate) activity_graph: Graph<ActivityId, ExecutionGraphRelation>,
    pub(crate) max_retries: u32,
}

impl DefaultExecutionState {
    pub fn new(activity_states: Option<Vec<Activity>>) -> Self {
        let activities = activity_states.unwrap_or_default();
        let activity_map = activities.into_iter().map(|a| (a.id().to_string(), a)).collect();
        DefaultExecutionState { activity_to_graph_map: activity_map, activity_graph: Graph::new(), max_retries: 3 }
    }
}

/// Side-effect create functions can go on the trait implementation for convenience
impl ExecutionState for DefaultExecutionState {
    fn new() -> DefaultExecutionState {
        DefaultExecutionState { activity_to_graph_map: HashMap::new(), activity_graph: Graph::new(), max_retries: 3 }
    }
    fn status(&self) -> ExecutionStatus {
        get_execution_status(self)
    }

    fn is_stopped(&self) -> bool {
        matches!(self.status(), ExecutionStatus::Error | ExecutionStatus::Failed | ExecutionStatus::Finished)
    }

    fn get_activity_state(&self, id: &str) -> Option<&Activity> {
        self.activity_to_graph_map.get(id)
    }

    fn activities(&self) -> impl Iterator<Item = &Activity> {
        self.activity_to_graph_map.values()
    }

    fn activity_count(&self) -> usize {
        self.activity_to_graph_map.len()
    }

    fn activity_dependents(&self, _id: ActivityId) -> Vec<&Activity> {
        vec![]
    }

    fn max_retries(&self) -> u32 {
        self.max_retries
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

    #[error("Activity depends on itself")]
    SelfReferentialDependency,
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
    fn activity_dependents() {
        let _empty_execution_state = DefaultExecutionState::new(None);
    }
}
