//! Stateful object that ties together processing loop with functions

mod get_prior_output;

use crate::api::events::{Event, EventStream};
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::runner::registry::Registry;
use crate::runner::{process, reduce, restore, scheduler};
pub use get_prior_output::resolve_prior_output;
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
            let Some(activity_event) = scheduler(&execution_state) else {
                break;
            };
            let start_event = Event::from(activity_event.clone());
            self.event_log.borrow_mut().push(start_event.clone());
            execution_state = reduce(execution_state, &start_event);

            let result_event = process(&execution_state, &self.registry, &activity_event);
            self.event_log.borrow_mut().push(result_event.clone());
            execution_state = reduce(execution_state, &result_event);
        }
        execution_state
    }
}

type LoopFn = fn(&DefaultExecutionState);
type EventHandlerFn = Box<dyn FnMut(&Event)>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::events::Event;
    use crate::runner::registry::Registry;
    use serde_json::json;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_controller_start() {
        let event_log = Rc::new(RefCell::new(vec![Event::add_sync(
            "1",
            "echo",
            Some(json!({ "message": "hello" })),
            None
        )]));
        let mut controller = Controller::new(Registry::new(None, None), event_log);
        controller.start();
    }
}
