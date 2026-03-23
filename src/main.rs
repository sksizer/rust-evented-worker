use serde_json::json;
use api::events::{EventStream, StepEvent};

mod execution_state;
mod steps;
mod view;
mod runner;
mod fixtures;
mod api;
mod r#impl;

use runner::Registry;

fn main() {
    let registry = Registry::new(Some(fixtures::get_test_step_modules()), None);

    let event_stream: EventStream = vec![
        StepEvent::AddSync("1".to_string(), "echo".to_string(), Some(json!("echo"))),
    ];
    let mut execution_state = runner::restore(event_stream);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);

    execution_state = runner::reduce(execution_state, &(StepEvent::add_sync("2", "echo",  Some(json!("echo")))));
    let next_step = runner::scheduler(&execution_state).unwrap();
    let result_event = runner::executor(&registry, next_step);
    execution_state = runner::reduce(execution_state, &result_event);
    println!("Execution Status: {:?}", execution_state.status());
    view::summarize::execution_state(&execution_state);
}
