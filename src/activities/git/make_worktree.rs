use crate::api::activities::ModuleDef;
use crate::api::events::Event;
use cmd_spec::ShellCommand;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct WorktreeConfig {
    repo: String,
    target_dir: String,
    branch_name: String,
}

fn handler(_config: &WorktreeConfig, _: &Value) -> Result<Vec<String>, Vec<String>> {
    // let worktree = fluent_git::builder::repo(config.repo.clone())
    //     .worktree()
    //     .add(config.branch_name.clone())
    //     .branch(config.branch_name.clone())
    //     .run();
    // match worktree {
    //     Ok(_worktree) => Ok(vec![]),
    //     Err(_) => Err(vec!["Something went wrong".to_string()]),
    // }
    Ok(vec![])
}

pub static MAKE_WORKTREE: ModuleDef<WorktreeConfig, Value, Vec<String>> =
    ModuleDef { id: "make_worktree", validate_config: |_config| true, validate_input: |_| true, execute: handler };

pub struct ActivityParameters {
    pub commands: Vec<ShellCommand>,
}

pub fn get_activity(id: &str, activity_parameters: ActivityParameters) -> Event {
    Event::add_sync(id, MAKE_WORKTREE.id, Some(json!({ "commands": activity_parameters.commands })), None)
}
