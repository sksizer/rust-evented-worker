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
            // TODO - get a start or continue event from the scheduler for a particular activity
            let Some(activity_event) = scheduler(&execution_state) else {
                break;
            };
            // We record the request to start or continue the event in the event log.
            // TODO - is this necessary or just basically logging noise?
            let start_event = Event::from(activity_event.clone());
            self.event_log.borrow_mut().push(start_event.clone());
            execution_state = reduce(execution_state, &start_event);

            // TODO - rename process to execute or run_activity perhaps?
            // TODO - add ability to get partially finished activity
            // TODO - add ability to get commands from processing an event - such as spawning child event
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
