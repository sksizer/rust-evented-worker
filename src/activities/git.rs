use crate::api::activities::ModuleDef;
use crate::api::events::Event;
use cmd_spec::ShellCommand;
use log::trace;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct ShellConfig {
    pub commands: Vec<ShellCommand>,
}

pub static SHELL: ModuleDef<ShellConfig, Value, Vec<String>> = ModuleDef {
    id: "shell",
    validate_config: |config| !config.commands.is_empty(),
    validate_input: |_| true,
    execute: |config, _input| {
        let mut results: Vec<String> = vec![];
        for command in &config.commands {
            trace!("Executing command: {} {:?}", command.program, command.args);
            let output = std::process::Command::from(command)
                .output()
                .map_err(|e| vec![format!("command execution failed: {}", e)])?
                .stdout;
            let std_out = String::from_utf8_lossy(&output);
            trace!("Std Out: {}", std_out);
            results.push(std_out.to_string());
        }
        Ok(results)
    },
};

pub struct ActivityParameters {
    pub commands: Vec<ShellCommand>,
}

pub fn get_activity(id: &str, activity_parameters: ActivityParameters) -> Event {
    Event::add_sync(
        id,
        SHELL.id,
        Some(json!({ "commands": activity_parameters.commands })),
        None,
    )
}
