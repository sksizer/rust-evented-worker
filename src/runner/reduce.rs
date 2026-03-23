use crate::api::events::StepEvent;
use crate::api::steps::{AsyncStep, Step, StepCore, SyncStep};
use crate::execution_state;
use crate::execution_state::ExecutionState;

/// Takes prior state + an event and returns an updated state
pub fn reduce(execution_state: ExecutionState, step_event: &StepEvent) -> ExecutionState {
    match step_event {
        StepEvent::AddSync(id, kind, _input) => {
            execution_state::append_step_state(
                execution_state,
                Step::Sync(SyncStep::Ready(StepCore {
                    id: id.clone(),
                    kind: kind.clone(),
                    input: None,
                })),
            )
            // TODO: propagate error instead of unwrap
            .unwrap()
        }
        StepEvent::AddAsync(id, kind, _input ) => {
            execution_state::append_step_state(
                execution_state,
                Step::Async(AsyncStep::Ready(StepCore {
                    id: id.clone(),
                    kind: kind.clone(),
                    input: None,
                })),
            )
            .unwrap()
        }
        StepEvent::Start(id) => {
            // Only meaningful for async steps; ignored for sync
            let core = get_step_core(&execution_state, id);
            let is_async = matches!(execution_state.step_states.last().unwrap(), Step::Async(_));
            if is_async {
                execution_state::update_step_state(
                    execution_state,
                    Step::Async(AsyncStep::Running(core)),
                )
                .unwrap()
            } else {
                execution_state
            }
        }
        StepEvent::Complete(id, output) => {
            let core = get_step_core(&execution_state, id);
            let is_async = matches!(execution_state.step_states.last().unwrap(), Step::Async(_));
            if is_async {
                execution_state::update_step_state(
                    execution_state,
                    Step::Async(AsyncStep::Completed { core, output: output.clone() }),
                )
            } else {
                execution_state::update_step_state(
                    execution_state,
                    Step::Sync(SyncStep::Completed { core, output: output.clone() }),
                )
            }
            .unwrap()
        }
        StepEvent::Failed(id, failure) => {
            let core = get_step_core(&execution_state, id);
            let is_async = matches!(execution_state.step_states.last().unwrap(), Step::Async(_));
            if is_async {
                execution_state::update_step_state(
                    execution_state,
                    Step::Async(AsyncStep::Failed { core, failure: failure.clone() }),
                )
            } else {
                execution_state::update_step_state(
                    execution_state,
                    Step::Sync(SyncStep::Failed { core, failure: failure.clone() }),
                )
            }
            .unwrap()
        }
        StepEvent::Error(id, failure) => {
            let core = get_step_core(&execution_state, id);
            let is_async = matches!(execution_state.step_states.last().unwrap(), Step::Async(_));
            if is_async {
                execution_state::update_step_state(
                    execution_state,
                    Step::Async(AsyncStep::Error { core, failure: failure.clone() }),
                )
            } else {
                execution_state::update_step_state(
                    execution_state,
                    Step::Sync(SyncStep::Error { core, failure: failure.clone() }),
                )
            }
            .unwrap()
        }
    }
}

/// Extract the core from the current step being transitioned
fn get_step_core(execution_state: &ExecutionState, id: &str) -> StepCore {
    let last = execution_state.step_states.last()
        .expect("Cannot transition step on empty execution state");
    assert_eq!(last.id(), id, "Event id does not match current step id");
    last.core().clone()
}
