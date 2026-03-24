use std::rc::Rc;
use std::cell::RefCell;
use log::trace;
use serde_json::json;
use evented_worker::api::events::EventStream;
use evented_worker::api::steps::StepEvent;
use evented_worker::fixtures::{get_registry, get_test_step_modules};
use evented_worker::runner::{resolve_prior_output, Controller, Registry};
use evented_worker::{runner, view};

fn main() {
    pretty_env_logger::init();
    let registry = Registry::new(Some(get_test_step_modules()), None);
    let event_stream: EventStream = vec![
        StepEvent::add_sync("1", "echo", Some(json!({ "config": "echo" }))),
    ];
    let mut execution_state = runner::restore(&event_stream);
    view::summarize::execution_state(&execution_state);

    let step_id = runner::scheduler(&execution_state).unwrap().id().to_string();
    let input = resolve_prior_output(&execution_state, &step_id);
    execution_state = runner::reduce(execution_state, &StepEvent::start(step_id, input));
    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    view::summarize::execution_state(&execution_state);

    execution_state = runner::reduce(execution_state, &StepEvent::add_sync("2", "echo", None));
    let step_id = runner::scheduler(&execution_state).unwrap().id().to_string();
    let input = resolve_prior_output(&execution_state, &step_id);
    execution_state = runner::reduce(execution_state, &StepEvent::start(step_id, input));
    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    view::summarize::execution_state(&execution_state);

    example_one();
    example_two();
}

fn example_one() {
    trace!("Example 1");
    let event_log = Rc::new(RefCell::new(vec![
        StepEvent::add_sync("0", "fixed_output", Some(json!({ "config": "DATA" }))),
        StepEvent::add_sync("1", "echo", None),
        StepEvent::add_sync("2", "echo", None),
    ]));

    let mut controller = Controller::new(get_registry(), event_log);
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}

fn example_two() {
    trace!("Example 2");
    let event_log = Rc::new(RefCell::new(vec![
        StepEvent::add_sync("shell", "shell", Some(json!({ "commands" : ["ls"]}))),
        StepEvent::add_sync("echo", "echo", None),
    ]));

    let mut controller = Controller::new(get_registry(), event_log.clone());
    controller.on_loop(|execution_state| {
        view::summarize::execution_state(&execution_state);
    });

    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
    trace!("Recorded events: {:?}", event_log.borrow());
}
