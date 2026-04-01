use std::cell::RefCell;
use std::rc::Rc;
use serde_json::json;
use evented_worker::api::activities::{SyncActivityHandler};
use evented_worker::api::events::Event;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::view;

fn main() {
    example_one();
}
// pub struct SyncActivityHandler {
//     pub name: String,
//     pub id: String,
//     pub description: String,
//     pub validate_config: Option<ValidateConfig>,
//     pub validate_input: Option<ValidateInput>,
//     pub handler: SyncHandler,
// }
//


fn example_one() {


     let mut registry = get_registry();

    _  =registry.register_sync( SyncActivityHandler {
        name: "Error on every call".to_string(),
        id: "error".to_string(),
        description: "".to_string(),
        validate_config: None,
        validate_input: None,
        handler: |_config, _input| {
            Err(vec!["error".to_string()])
        }
    });

    // TODO - add an activity handler that fails

    let event_log = Rc::new(RefCell::new(vec![
        Event::add_sync("0", "error", Some(json!({ "config": "DATA" })), None),
    ]));

    let mut controller = Controller::new(registry, event_log.clone());
    let execution_state = controller.start();

    view::summarize::execution_state(&execution_state);
    view::summarize::event_stream(&event_log.borrow());
}