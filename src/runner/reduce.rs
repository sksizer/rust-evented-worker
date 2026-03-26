mod add_step;
mod get_execution_status;
mod update_step;

use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::{
    AsyncStep, AsyncReady, Step, StepCore, StepEvent, SyncNew, SyncStep,
};

use add_step::append_step_state;
pub use get_execution_status::get_execution_status;
use update_step::update;

/// Takes prior state + an event and returns an updated state
pub fn reduce(
    execution_state: DefaultExecutionState,
    step_event: &StepEvent,
) -> DefaultExecutionState {
    match step_event {
        StepEvent::AddSync(payload) => {
            let core = StepCore {
                id: payload.id.clone(),
                kind: payload.kind.clone(),
                config: payload.config.clone(),
            };
            let step = Step::from(SyncStep::from(SyncNew::new(core).make_ready(None)));
            append_step_state(execution_state, step).unwrap()
        }
        StepEvent::AddAsync(payload) => {
            let core = StepCore {
                id: payload.id.clone(),
                kind: payload.kind.clone(),
                config: payload.config.clone(),
            };
            let step = Step::from(AsyncStep::from(AsyncReady::new(core)));
            append_step_state(execution_state, step).unwrap()
        }
        StepEvent::Start(id) => {
            let new_step = match execution_state.get_step_state(id) {
                Some(Step::Sync(SyncStep::Ready(ready))) => {
                    Step::from(SyncStep::from(ready.clone().start()))
                }
                Some(Step::Async(AsyncStep::Ready(ready))) => {
                    Step::from(AsyncStep::from(ready.clone().start(None)))
                }
                _ => panic!("Invalid step state for Start event: {}", id),
            };
            update(execution_state, new_step).unwrap()
        }
        StepEvent::Complete(payload) => {
            let new_step = match execution_state.get_step_state(&payload.id) {
                Some(Step::Sync(SyncStep::Running(running))) => {
                    Step::from(SyncStep::from(running.clone().complete(payload.output.clone())))
                }
                Some(Step::Async(AsyncStep::Running(running))) => {
                    Step::from(AsyncStep::from(running.clone().complete(payload.output.clone())))
                }
                _ => panic!("Invalid step state for Complete event: {}", payload.id),
            };
            update(execution_state, new_step).unwrap()
        }
        StepEvent::Failed(payload) => {
            let failure = payload.reason.as_ref().map(|r| vec![r.clone()]);
            let new_step = match execution_state.get_step_state(&payload.id) {
                Some(Step::Sync(SyncStep::Running(running))) => {
                    Step::from(SyncStep::from(running.clone().fail(failure)))
                }
                Some(Step::Async(AsyncStep::Running(running))) => {
                    Step::from(AsyncStep::from(running.clone().fail(failure)))
                }
                _ => panic!("Invalid step state for Failed event: {}", payload.id),
            };
            update(execution_state, new_step).unwrap()
        }
        StepEvent::Error(payload) => {
            let failure = payload.reason.as_ref().map(|r| vec![r.clone()]);
            let new_step = match execution_state.get_step_state(&payload.id) {
                Some(Step::Sync(SyncStep::Running(running))) => {
                    Step::from(SyncStep::from(running.clone().error(failure)))
                }
                Some(Step::Async(AsyncStep::Running(running))) => {
                    Step::from(AsyncStep::from(running.clone().error(failure)))
                }
                _ => panic!("Invalid step state for Error event: {}", payload.id),
            };
            update(execution_state, new_step).unwrap()
        }
        StepEvent::SystemError(payload) => {
            // TODO - consider this system error whether it even makes sense to have it
            execution_state
        }
    }
}
