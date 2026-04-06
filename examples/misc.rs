use cmd_spec::ShellCommand;
use evented_worker::InMemoryEventStore;
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use evented_worker::api::EventStore;
use evented_worker::api::activities::ActivityEvent;
use evented_worker::api::events::{Event, EventStream};
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::runner::Registry;
use evented_worker::{runner, view};
use log::trace;
use serde_json::json;

fn main() {
    pretty_env_logger::init();
    let registry = get_registry();
    let event_stream: EventStream = vec![Event::add_sync("1", "echo", Some(json!({ "config": "echo" })), None)];
    let mut execution_state = runner::restore(&event_stream);
    view::summarize::execution_state(&execution_state);

    // Activity 1: schedule, create start event, reduce it, then process and reduce the result
    let activity_id = runner::scheduler(&execution_state).unwrap();
    let activity_event = ActivityEvent::start(activity_id);
    let start_event = Event::from(activity_event.clone());
    execution_state = runner::reduce(execution_state, &start_event);
    let result_event = runner::process(&execution_state, &registry, &activity_event);
    execution_state = runner::reduce(execution_state, &result_event);
    view::summarize::execution_state(&execution_state);

    // Activity 2
    execution_state = runner::reduce(execution_state, &Event::add_sync("2", "echo", None, None));
    let activity_id = runner::scheduler(&execution_state).unwrap();
    let activity_event = ActivityEvent::start(activity_id);
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
    let mut store = InMemoryEventStore::from_events(vec![
        Event::add_sync("0", "fixed_output", Some(json!({ "config": "DATA" })), None),
        Event::add_sync("1", "echo", None, None),
        Event::add_sync("2", "echo", None, None),
    ]);

    let mut controller = Controller::new(get_registry(), &mut store);
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}

fn example_two() {
    trace!("Example 2");
    let mut store = InMemoryEventStore::from_events(vec![
        get_activity("0", ActivityParameters { commands: vec![ShellCommand::new("ls")] }),
        Event::add_sync("echo", "echo", None, None),
    ]);

    let mut controller = Controller::new(get_registry(), &mut store);
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
    view::summarize::event_stream(&store.get_events());
}
