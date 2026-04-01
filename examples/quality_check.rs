use cmd_spec::ShellCommand;
use evented_worker::api::events::EventStream;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use evented_worker::view::summarize;
use std::cell::RefCell;
use std::rc::Rc;

fn quality_check() -> EventStream {
    vec![get_activity(
        "0",
        ActivityParameters {
            commands: vec![
                ShellCommand::new("cargo").arg("fmt"),
                ShellCommand::new("cargo").args(["clippy", "--fix"]),
            ],
        },
    )]
}

fn run_quality_check() {
    let registry = get_registry();
    let event_stream = quality_check();
    let event_log = Rc::new(RefCell::new(event_stream));
    let mut controller = Controller::new(registry, event_log);
    let execution_state = controller.start();
    summarize::execution_state(&execution_state);
}

fn main() {
    run_quality_check();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_quality_check() {
        run_quality_check();
    }
}
