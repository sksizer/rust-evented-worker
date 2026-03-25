use evented_worker::api::steps::StepEvent;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::view;
use log::trace;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;
use evented_worker::steps::shell::{get_step, StepParameters};
use serde_command::ShellCommand;

fn main() {
    trace!("Example 3: Update Readme");
    let event_log = Rc::new(RefCell::new(vec![
        get_step("0", StepParameters {
            commands: vec![
                ShellCommand::new("pnpm").args(vec![
                    "dlx",
                    "@anthropic-ai/claude-code",
                    "--permission-mode",
                    "acceptEdits",
                    "-p",
                    "Please create or update a suitable README for this project",
                ])
            ]
        }),
        get_step("1", StepParameters {
            commands: vec![
                ShellCommand::new("git").args(vec![
                    "add",
                    "README.md"
                ])
            ]
        }),
        get_step("2", StepParameters {
            commands: vec![
                ShellCommand::new("git_commit_message").args(vec![
                    "-a"
                ])
            ]
        })
    ]));
    let mut controller = Controller::new(get_registry(), event_log.clone());
    let execution_state = controller.start();
    view::summarize::execution_state(&execution_state);
}
