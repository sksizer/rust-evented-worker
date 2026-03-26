//! Stateful object that ties together processing loop with functions

mod get_prior_output;

use crate::api::events::EventStream;
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::Step;
use crate::api::steps::StepEvent;
use crate::runner::registry::Registry;
use crate::runner::{process, processor, reduce, restore, scheduler};
pub use get_prior_output::resolve_prior_output;
use log::trace;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Controller {
    registry: Registry,
    event_log: Rc<RefCell<EventStream>>,
    loop_fn: Vec<LoopFn>,
    event_handlers: Vec<EventHandlerFn>,
}

impl Controller {
    pub fn new(registry: Registry, event_log: Rc<RefCell<EventStream>>) -> Controller {
        Controller {
            registry,
            event_log,
            loop_fn: vec![],
            event_handlers: vec![],
        }
    }

    pub fn start(&mut self) -> DefaultExecutionState {
        let mut execution_state = restore(&self.event_log.borrow());
        loop {
            let Some(event) = scheduler(&execution_state) else { break };
            self.event_log.borrow_mut().push(event.clone());
            execution_state = reduce(execution_state, &event);

            let event = process(&execution_state, &self.registry, &event);
            self.event_log.borrow_mut().push(event.clone());
            execution_state = reduce(execution_state, &event);
        }
        execution_state
    }
}

type LoopFn = fn(&DefaultExecutionState);
type EventHandlerFn = Box<dyn FnMut(&StepEvent)>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::steps::StepEvent;
    use crate::runner::registry::Registry;
    use serde_json::json;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_controller_start() {
        let event_log = Rc::new(RefCell::new(vec![StepEvent::add_sync(
            "1",
            "echo",
            Some(json!({ "message": "hello" })),
        )]));
        let mut controller = Controller::new(Registry::new(None, None), event_log);
        controller.start();
    }
}
