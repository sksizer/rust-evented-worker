use cmd_spec::ShellCommand;
use evented_worker::InMemoryEventStore;
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::view;
use log::trace;

fn main() {
    trace!("Example 3: Update Readme");
    let mut store = InMemoryEventStore::from_events(vec![
        get_activity(
            "0",
            ActivityParameters {
                commands: vec![ShellCommand::new("pnpm").args(vec![
                    "dlx",
                    "@anthropic-ai/claude-code",
                    "--permission-mode",
                    "acceptEdits",
                    "-p",
                    "Please create or update a suitable README for this project",
                ])],
            },
        ),
        get_activity(
            "1",
            ActivityParameters {
                commands: vec![ShellCommand::new("git").args(vec!["add", "README.md"])],
            },
        ),
        get_activity(
            "2",
            ActivityParameters {
                commands: vec![ShellCommand::new("git_commit_message").args(vec!["-a"])],
            },
        ),
    ]);
    let mut controller = Controller::new(get_registry(), &mut store);
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}
