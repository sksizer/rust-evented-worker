//! Solely responsible for calling a step function
use log::{error, trace};
use crate::api::steps::{StepEvent, SyncStep, StepConfig, StepInput};
use crate::api::steps::Step;
use crate::runner::registry::Registry;

/// Knows how to call a step function. Is at the edge of side effects
pub fn executor(registry: &Registry, step: &Step) -> StepEvent {
    trace!("executor - step: {:?}", step);
    let id = step.id().to_string();
    match step {
        // SYNC
        Step::Sync(s) => {
            match s {
                SyncStep::Ready { core, input } => {
                    match registry.get_sync_module(&core.kind) {
                        Some(step_module) => {
                            match (step_module.handler)(StepConfig(core.config.clone()), StepInput(input.clone())) {
                                Ok(value) => StepEvent::complete(id, Some(value)),
                                Err(errors) => StepEvent::error(id, Some(errors.join("; "))),
                            }
                        }
                        None => StepEvent::error(id, Some(format!("No step handler registered for: {}", core.kind))),
                    }
                }
                SyncStep::Completed { .. } => invariant_violation(&id, "executor called on step in completed state"),
                SyncStep::Failed { .. } => invariant_violation(&id, "executor called on step in failed state"),
                SyncStep::Error { .. } => invariant_violation(&id, "executor called on step in error state"),
            }
        }

        // ASYNC
        Step::Async(_) => {
            unimplemented!("Async step execution not yet implemented")
        }
    }
}

fn invariant_violation(id: &str, message: &str) -> StepEvent {
    error!("Executor invariant violation {} {}", id, message);
    StepEvent::error(id.to_string(), Some(message.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_executor() {
    }
}