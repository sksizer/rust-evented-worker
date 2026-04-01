use cmd_spec::ShellCommand;
use evented_worker::api::events::{Event, EventStream};
use evented_worker::fixtures::get_registry;
use evented_worker::fixtures::get_test_activity_modules;
use evented_worker::runner::Controller;
use evented_worker::runner::Registry;
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use evented_worker::{runner, view};
use log::trace;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    pretty_env_logger::init();
    let registry = Registry::new(Some(get_test_activity_modules()), None);
    let event_stream: EventStream = vec![Event::add_sync(
        "1",
        "echo",
        Some(json!({ "config": "echo" })),
        None
    )];
    let mut execution_state = runner::restore(&event_stream);
    view::summarize::execution_state(&execution_state);

    // Activity 1: schedule (produces an ActivityEvent), reduce it, then process and reduce the result
    let activity_event = runner::scheduler(&execution_state).unwrap();
    let start_event = Event::from(activity_event.clone());
    execution_state = runner::reduce(execution_state, &start_event);
    let result_event = runner::process(&execution_state, &registry, &activity_event);
    execution_state = runner::reduce(execution_state, &result_event);
    view::summarize::execution_state(&execution_state);

    // Activity 2
    execution_state = runner::reduce(execution_state, &Event::add_sync("2", "echo", None, None));
    let activity_event = runner::scheduler(&execution_state).unwrap();
    let start_event = Event::from(activity_event.clone());
    execution_state = runner::reduce(execution_state, &start_event);
    let result_event = runner::process(&execution_state, &registry, &activity_event);
    execution_state = runner::reduce(execution_state, &result_event);
    view::summarize::execution_state(&execution_state);

    example_one();
    example_two();
}

fn example_one() {
    trace!("Example 1");
    let event_log = Rc::new(RefCell::new(vec![
        Event::add_sync("0", "fixed_output", Some(json!({ "config": "DATA" })), None),
        Event::add_sync("1", "echo", None, None),
        Event::add_sync("2", "echo", None, None),
    ]));

    let mut controller = Controller::new(get_registry(), event_log);
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}

fn example_two() {
    trace!("Example 2");
    let event_log = Rc::new(RefCell::new(vec![
        get_activity(
            "0",
            ActivityParameters {
                commands: vec![ShellCommand::new("ls")],
            },
        ),
        Event::add_sync("echo", "echo", None, None),
    ]));

    let mut controller = Controller::new(get_registry(), event_log.clone());
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
    trace!("Recorded events: {:?}", event_log.borrow());
}
