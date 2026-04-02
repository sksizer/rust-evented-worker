use cmd_spec::ShellCommand;
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use evented_worker::api::events::EventStream;
use evented_worker::fixtures::get_registry;
use evented_worker::runner::Controller;
use evented_worker::view::summarize;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WorkflowParameters {
    id: Option<String>,
}

pub struct Activity {}

pub struct Workflow {
    id: String,
}
impl Workflow {
    fn build() -> WorkflowParameters {
        WorkflowParameters { id: None }
    }

    fn id(&mut self, id: impl Into<String>) -> &Self {
        self.id = id.into();
        self
    }

    fn add_activity(&self) -> Activity {
        Activity {}
    }
}

pub struct Module {
    pub id: &'static str,
    pub display_name: &'static str,
    pub get_config: fn(), // pub process: fn(input: &[u8]) -> Vec<u8>,
                          // pub validate: fn(input: &[u8]) -> bool,
}

pub static WORKTREE_ACTIVITY: Module = Module {
    id: "worktree_activity",
    display_name: "Worktree",
    get_config: || {},
};

/// Facade to make it easy to compose coding tasks
pub struct CodingWorkflow {
    id: String,
}

impl CodingWorkflow {
    fn build(id: impl Into<String>) -> CodingWorkflow {
        CodingWorkflow { id: id.into() }
    }

    fn add() {}
}

fn run_quality_check() -> CodingWorkflow {
    let workflow = CodingWorkflow::build("qualityCheck-syn0111");
    workflow
    // workflow.
    //
    // workflow.add_activity(WORKTREE_ACTIVITY.id, WORKTREE_ACTIVITY, );
    //
    // workflow.add_activity("worktree:create", );
    // workflow.add_activity("coding_task", None);
    //
    //
    // let engine = Engine::new()
    // let submission = engine.submit_workflow(workflow);
    // engine.start()

    // let registyr = get_registry();
    // let event_stream = quality_check();
    // let event_log = Rc::new(RefCell::new(event_stream));
    // let mut controller = Controller::new(registyr, event_log);
    // let execution_state = controller.start();
    // summarize::execution_state(&execution_state);
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
