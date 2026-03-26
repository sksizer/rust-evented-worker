//! Solely responsible for calling a step function
use crate::api::steps::{Step, SystemErrorData};
use crate::api::steps::{StepConfig, StepEvent, StepInput, SyncStep};
use crate::runner::registry::Registry;
use log::{error, trace};
use serde::Deserializer;
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::CompletePayload;

/// Knows how to call a step function. Is at the edge of side effects
pub fn process(state:&DefaultExecutionState, registry: &Registry, step_event: &StepEvent) -> StepEvent {
    trace!("executor - event: {:?}", step_event);
    let id = step_event.step_id().to_string();
    match (step_event) {
        StepEvent::Start(step_id) => {
            let step = state.get_step_state(step_id).unwrap();
            let Some(handler) = registry.get_sync_module(&step.kind()) else {
                return StepEvent::error(id, Some("Could not find step module".to_string()));
            };
            let config = StepConfig(step.config().cloned());
            let input = StepInput(step.input().cloned());
            let result = (handler.handler)(config, input);
            StepEvent::Complete(CompletePayload {
                id: step_id.to_string(),
                output: Some(result.unwrap())
            })
        }
        _ => {
            StepEvent::SystemError(SystemErrorData {
               step_id: id.to_string(),
                errors: vec!["Invalid event sent to processor".to_string()],
                source: "processor::process".to_string()
            })
        }
    }
    // match step {
    //     // SYNC
    //     Step::Sync(s) => match s {
    //         SyncStep::Ready(ready) => match registry.get_sync_module(&ready.core.kind) {
    //             Some(step_module) => {
    //                 match (step_module.handler)(
    //                     StepConfig(ready.core.config.clone()),
    //                     StepInput(ready.input.clone()),
    //                 ) {
    //                     Ok(value) => StepEvent::complete(id, Some(value)),
    //                     Err(errors) => StepEvent::error(id, Some(errors.join("; "))),
    //                 }
    //             }
    //             None => StepEvent::error(
    //                 id,
    //                 Some(format!("No step handler registered for: {}", ready.core.kind)),
    //             ),
    //         },
    //         SyncStep::Completed(_) => {
    //             invariant_violation(&id, "executor called on step in completed state")
    //         }
    //         SyncStep::Failed(_) => {
    //             invariant_violation(&id, "executor called on step in failed state")
    //         }
    //         SyncStep::Error(_) => {
    //             invariant_violation(&id, "executor called on step in error state")
    //         }
    //         _ => {
    //             invariant_violation(&id, "executor called on step in unexpected state")
    //         }
    //     },
    //
    //     // ASYNC
    //     Step::Async(_) => {
    //         unimplemented!("Async step execution not yet implemented")
    //     }
    // }
}

fn invariant_violation(id: &str, message: &str) -> StepEvent {
    error!("Executor invariant violation {} {}", id, message);
    StepEvent::error(id.to_string(), Some(message.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_executor() {}
}
