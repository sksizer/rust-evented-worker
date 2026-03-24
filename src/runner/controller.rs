//! Stateful object that ties together processing loop with functions

mod get_prior_output;

use std::rc::Rc;
use std::cell::RefCell;
use log::trace;
use crate::api::events::EventStream;
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::steps::StepEvent;
use crate::api::steps::Step;
use crate::runner::{executor, reduce, restore, scheduler};
use crate::runner::registry::Registry;
pub use get_prior_output::resolve_prior_output;

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
        while !execution_state.is_stopped() {
            trace!("Controller - processing");
            for loop_fn in &self.loop_fn {
                (loop_fn)(&execution_state);
            }
            match scheduler(&execution_state) {
                None => break,
                Some(step) => {
                    let input = get_prior_output::resolve_prior_output(&execution_state, step.id());
                    let start_event = StepEvent::start(step.id().to_string(), input);
                    self.event_log.borrow_mut().push(start_event.clone());
                    execution_state = reduce(execution_state, &start_event);

                    let step = scheduler(&execution_state).unwrap();
                    let event = executor(&self.registry, step);
                    for event_handler in &mut self.event_handlers {
                        (event_handler)(&event);
                    }
                    self.event_log.borrow_mut().push(event.clone());
                    execution_state = reduce(execution_state, &event);
                }
            }
        }
        execution_state
    }

    pub fn on_loop(&mut self, loop_fn: LoopFn) {
        self.loop_fn.push(loop_fn);
    }

    pub fn on_event(&mut self, event_handler: EventHandlerFn) {
        self.event_handlers.push(event_handler);
    }
}

type LoopFn = fn(&DefaultExecutionState);
type EventHandlerFn = Box<dyn FnMut(&StepEvent)>;

#[cfg(test)]
mod test {
    use serde_json::json;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::api::steps::StepEvent;
    use crate::runner::registry::Registry;
    use super::*;

    #[test]
    fn test_controller_start() {
        let event_log = Rc::new(RefCell::new(vec![
            StepEvent::add_sync("1", "echo", Some(json!({ "message": "hello" }))),
        ]));
        let mut controller = Controller::new(Registry::new(None, None), event_log);
        controller.start();
    }
}
