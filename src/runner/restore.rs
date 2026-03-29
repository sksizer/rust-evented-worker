//! Creates a realized execution state from an event stream
use crate::api::events::{Event, EventStream};
use crate::api::execution::DefaultExecutionState;
use crate::runner::reduce::reduce;

/// helper function to return a single execution state over a series of events
pub fn restore(event_stream: &EventStream) -> DefaultExecutionState {
    let execution_state = DefaultExecutionState {
        activity_states: Vec::new(),
    };
    event_stream.iter().fold(execution_state, reduce)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::events::Event;
    use crate::api::execution::{ExecutionState, ExecutionStatus};
    use crate::api::activities::{AsyncActivity, Activity, SyncActivity};

    #[test]
    fn test_adding_a_single_activity() {
        let event_stream = vec![Event::add_sync("1", "alpha", None)];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.activity_states.len(), 1);
        assert_eq!(execution_state.activity_states[0].id(), "1");
    }

    #[test]
    fn test_adding_multiple_activities() {
        let event_stream = vec![
            Event::add_sync("1", "alpha", None),
            Event::add_sync("2", "beta", None),
            Event::add_sync("3", "gamma", None),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.activity_states.len(), 3);
    }

    #[test]
    fn single_activity_progression() {
        let event_stream = vec![
            Event::add_sync("1", "alpha", None),
            Event::start("1"),
            Event::complete("1", None),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.activity_states.len(), 1);
        assert!(matches!(
            execution_state.activity_states[0],
            Activity::Sync(SyncActivity::Completed(_))
        ));
        assert_eq!(execution_state.activity_states[0].id(), "1");
    }

    #[test]
    fn two_activity_progression() {
        let event_stream = vec![
            Event::add_sync("1", "alpha", None),
            Event::start("1"),
            Event::complete("1", None),
            Event::add_sync("2", "beta", None),
            Event::start("2"),
            Event::complete("2", None),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.activity_states.len(), 2);
        assert!(matches!(
            execution_state.activity_states[0],
            Activity::Sync(SyncActivity::Completed(_))
        ));
        assert!(matches!(
            execution_state.activity_states[1],
            Activity::Sync(SyncActivity::Completed(_))
        ));
    }

    #[test]
    fn three_activity_failure() {
        let event_stream = vec![
            Event::add_sync("1", "alpha", None),
            Event::start("1"),
            Event::complete("1", None),
            Event::add_sync("2", "beta", None),
            Event::start("2"),
            Event::complete("2", None),
            Event::add_sync("3", "gamma", None),
            Event::start("3"),
            Event::failed("3", Some("something went wrong".into())),
        ];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.activity_states.len(), 3);
        assert!(matches!(
            execution_state.activity_states[0],
            Activity::Sync(SyncActivity::Completed(_))
        ));
        assert!(matches!(
            execution_state.activity_states[1],
            Activity::Sync(SyncActivity::Completed(_))
        ));
        assert!(matches!(
            execution_state.activity_states[2],
            Activity::Sync(SyncActivity::Failed(_))
        ));

        assert_eq!(execution_state.status(), ExecutionStatus::Failed);
    }

    #[test]
    fn async_activity_start_running() {
        let event_stream = vec![Event::add_async("1", "fetch", None), Event::start("1")];
        let execution_state = restore(&event_stream);
        assert_eq!(execution_state.activity_states.len(), 1);
        assert!(matches!(
            execution_state.activity_states[0],
            Activity::Async(AsyncActivity::Running(_))
        ));
    }
}
