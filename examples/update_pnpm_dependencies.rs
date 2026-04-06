mod workflow;

use evented_worker::InMemoryEventStore;
use evented_worker::api::EventStore;
use evented_worker::api::activities::{ModuleDef, SerdeModule};
use evented_worker::api::events::Event;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::serde_module;
use evented_worker::view;
use serde_json::{Value, json};

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
    // let mut workflow = Workflow::new("Update PNPM Dependencies");

    // workflow.add_activities(Sequence::new( vec![
    //     // Given a target directory
    //     SetExecutionEnvironment("/Users/sksizer2/Development/templates/template-tauri-nuxt"),
    //     // Check initial constraints
    //     RunShellCommands(vec![]), // Make shell commands list
    //     ]),
    //     // Go there, make a worktree
    //                         // Either RunShellCommand or Specialized tasks (ideally the latter probably)
    //     // go to the worktree
    //     SetExecutionEnvironment("/Users/sksizer2/Development/templates/template-tauri-nuxt"),
    //     // perform pnpm updates
    //                         RunShellCommands(vec![]), // Make shell commands list
    //     // Perform quality checks
    //                         RunAgentCommand({}),
    //     // Commit changes
    //                         RunShellCommand(// Git add and all that)
    //     // Make a PR
    //                                         GH::make_pr
    //
    // ]));

    // Then we need to translate workflow ^ to events to run the the evented worker

    let mut registry = get_registry();

    let error_mod = serde_module!(ERROR_MODULE, config: Value, input: Value, output: Value);
    registry.register_module(error_mod).unwrap();

    let mut store = InMemoryEventStore::from_events(vec![
        // Steps

        // Given a target directory check initial constrains
        Event::add_sync("0", "shell", Some(json!({ "config": "DATA" })), None),
        // Go there, make a worktree

        // go to the worktree

        // perform pnpm updates

        // Perform quality checks

        // Commit changes

        // Make a PR
        Event::add_sync("1", "error", Some(json!({ "config": "DATA" })), Some(vec!["0".to_string()])),
    ]);

    let mut controller = Controller::new(registry, &mut store);
    let execution_state = controller.start();

    view::summarize::execution_state(&execution_state);
    view::summarize::event_stream(&store.get_events());
}
