//! Creates a realized execution state from an event stream
use crate::api::events::EventStream;
use crate::api::execution::DefaultExecutionState;
use crate::runner::reduce::reduce;

/// helper function to return a single execution state over a series of events
pub fn restore(event_stream: &EventStream) -> DefaultExecutionState {
    let execution_state = DefaultExecutionState {
        step_states: Vec::new(),
    };
    event_stream.iter().fold(execution_state, reduce)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::execution::{ExecutionState, ExecutionStatus};
    use crate::api::steps::{AsyncStep, Step, StepEvent, SyncStep};

    #[test]
    fn test_adding_a_single_step() {
        let event_stream = vec![StepEvent::add_sync("1", "alpha", None)];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert_eq!(execution_state.step_states[0].id(), "1");
    }

    #[test]
    fn test_adding_multiple_steps() {
        let event_stream = vec![
            StepEvent::add_sync("1", "alpha", None),
            StepEvent::add_sync("2", "beta", None),
            StepEvent::add_sync("3", "gamma", None),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.step_states.len(), 3);
    }

    #[test]
    fn single_step_progression() {
        let event_stream = vec![
            StepEvent::add_sync("1", "alpha", None),
            StepEvent::start("1"),
            StepEvent::complete("1", None),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert!(matches!(
            execution_state.step_states[0],
            Step::Sync(SyncStep::Completed(_))
        ));
        assert_eq!(execution_state.step_states[0].id(), "1");
    }

    #[test]
    fn two_step_progression() {
        let event_stream = vec![
            StepEvent::add_sync("1", "alpha", None),
            StepEvent::start("1"),
            StepEvent::complete("1", None),
            StepEvent::add_sync("2", "beta", None),
            StepEvent::start("2"),
            StepEvent::complete("2", None),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.step_states.len(), 2);
        assert!(matches!(
            execution_state.step_states[0],
            Step::Sync(SyncStep::Completed(_))
        ));
        assert!(matches!(
            execution_state.step_states[1],
            Step::Sync(SyncStep::Completed(_))
        ));
    }

    #[test]
    fn three_step_failure() {
        let event_stream = vec![
            StepEvent::add_sync("1", "alpha", None),
            StepEvent::start("1"),
            StepEvent::complete("1", None),
            StepEvent::add_sync("2", "beta", None),
            StepEvent::start("2"),
            StepEvent::complete("2", None),
            StepEvent::add_sync("3", "gamma", None),
            StepEvent::start("3"),
            StepEvent::failed("3", Some("something went wrong".into())),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.step_states.len(), 3);
        assert!(matches!(
            execution_state.step_states[0],
            Step::Sync(SyncStep::Completed(_))
        ));
        assert!(matches!(
            execution_state.step_states[1],
            Step::Sync(SyncStep::Completed(_))
        ));
        assert!(matches!(
            execution_state.step_states[2],
            Step::Sync(SyncStep::Failed(_))
        ));

        assert_eq!(execution_state.status(), ExecutionStatus::Failed);
    }

    #[test]
    fn async_step_start_running() {
        let event_stream = vec![
            StepEvent::add_async("1", "fetch", None),
            StepEvent::start("1"),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.step_states.len(), 1);
        assert!(matches!(
            execution_state.step_states[0],
            Step::Async(AsyncStep::Running(_))
        ));
    }
}
