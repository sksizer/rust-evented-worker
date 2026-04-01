use std::cell::RefCell;
use std::rc::Rc;
use serde_json::{json, Value};
use evented_worker::api::activities::{ModuleDef, SerdeModule};
use evented_worker::api::events::Event;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::serde_module;
use evented_worker::view;

static ERROR_MODULE: ModuleDef<Value, Value, Value> = ModuleDef {
    id: "error",
    validate_config: |_| true,
    validate_input: |_| true,
    execute: |_config, _input| Err(vec!["error".to_string()]),
};

fn main() {
    pretty_env_logger::init();
    example_one();
}

fn example_one() {
    let mut registry = get_registry();

    let error_mod = serde_module!(ERROR_MODULE, config: Value, input: Value, output: Value);
    registry.register_module(error_mod).unwrap();

    let event_log = Rc::new(RefCell::new(vec![
        Event::add_sync("0", "error", Some(json!({ "config": "DATA" })), None),
        Event::add_sync("1", "error", Some(json!({ "config": "DATA" })), Some(vec!["0".to_string()])),
    ]));

    let mut controller = Controller::new(registry, event_log.clone());
    let execution_state = controller.start();

    view::summarize::execution_state(&execution_state);
    view::summarize::event_stream(&event_log.borrow());
}
