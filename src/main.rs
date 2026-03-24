use log::trace;
use serde_json::json;
use api::events::EventStream;

mod steps;
mod view;
mod runner;
mod fixtures;
mod api;
mod r#impl;

use runner::Registry;
use crate::api::execution::ExecutionState;
use crate::api::steps::StepEvent;
use crate::fixtures::get_registry;
use crate::runner::{resolve_prior_output, Controller};


fn main() {
    pretty_env_logger::init();
    let registry = Registry::new(Some(fixtures::get_test_step_modules()), None);
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
}

fn example_one() {
    trace!("Example 1");
    let event_stream: EventStream = vec![
        StepEvent::add_sync("0", "fixed_output", Some(json!({ "config": "DATA" }))),
        StepEvent::add_sync("1", "echo", None),
        StepEvent::add_sync("2", "echo", None),
        StepEvent::add_sync("3", "echo", None),
        StepEvent::add_sync("4", "echo", None),
        StepEvent::add_sync("5", "echo", None),
        StepEvent::add_sync("6", "echo", None),
    ];

    let controller = Controller::new(
        get_registry(),
        Some(event_stream));
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}
