//! Creates a realized execution state from an event stream

use crate::api::events::{EventStream, StepEvent};
use crate::execution_state::ExecutionState;
use crate::runner::reduce::reduce;

/// helper function to return a single execution state over a series of events
pub fn restore(event_stream: EventStream) -> ExecutionState {
    let execution_state = ExecutionState {
        step_states: Vec::new(),
    };
    event_stream.iter().fold(execution_state, reduce)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::steps::{AsyncStep, Step, SyncStep};
    use crate::execution_state::ExecutionStatus;

    #[test]
    fn test_adding_a_single_step() {
        let event_stream = vec![
            StepEvent::AddSync(String::from("1"), "alpha".to_string(), None),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert_eq!(execution_state.step_states[0].id(), "1");
    }

    #[test]
    fn test_adding_multiple_steps() {
        let event_stream = vec![
            StepEvent::AddSync(String::from("1"), "alpha".to_string(), None),
            StepEvent::AddSync(String::from("2"), "beta".to_string(), None),
            StepEvent::AddSync(String::from("3"), "gamma".to_string(), None),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 3);
    }

    #[test]
    fn single_step_progression() {
        let event_stream = vec![
            StepEvent::AddSync(String::from("1"), "alpha".to_string(), None),
            StepEvent::Complete(String::from("1"), None),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert!(matches!(execution_state.step_states[0], Step::Sync(SyncStep::Completed { .. })));
        assert_eq!(execution_state.step_states[0].id(), "1");
    }

    #[test]
    fn two_step_progression() {
        let event_stream = vec![
            StepEvent::AddSync(String::from("1"), "alpha".to_string(), None),
            StepEvent::Complete(String::from("1"), None),
            StepEvent::AddSync(String::from("2"), "beta".to_string(), None),
            StepEvent::Complete(String::from("2"), None),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 2);
        assert!(matches!(execution_state.step_states[0], Step::Sync(SyncStep::Completed { .. })));
        assert!(matches!(execution_state.step_states[1], Step::Sync(SyncStep::Completed { .. })));
    }

    #[test]
    fn three_step_failure() {
        let event_stream = vec![
            StepEvent::AddSync(String::from("1"), "alpha".to_string(), None),
            StepEvent::Complete(String::from("1"), None),
            StepEvent::AddSync(String::from("2"), "beta".to_string(), None),
            StepEvent::Complete(String::from("2"), None),
            StepEvent::AddSync(String::from("3"), "gamma".to_string(), None),
            StepEvent::Failed(String::from("3"), Some("something went wrong".into())),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 3);
        assert!(matches!(execution_state.step_states[0], Step::Sync(SyncStep::Completed { .. })));
        assert!(matches!(execution_state.step_states[1], Step::Sync(SyncStep::Completed { .. })));
        assert!(matches!(execution_state.step_states[2], Step::Sync(SyncStep::Failed { .. })));

        assert_eq!(execution_state.status(), ExecutionStatus::Failed);
    }

    #[test]
    fn async_step_start_running() {
        let event_stream = vec![
            StepEvent::AddAsync(String::from("1"), "fetch".to_string(), None),
            StepEvent::Start(String::from("1")),
        ];
        let execution_state = restore(event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert!(matches!(execution_state.step_states[0], Step::Async(AsyncStep::Running(_))));
    }
}
