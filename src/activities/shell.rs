use crate::api::events::Event;
use crate::api::activities::{ActivityConfig, ActivityError, ActivityInput, SyncActivityHandler};
use cmd_spec::ShellCommand;
use log::trace;
use serde::Deserialize;
use serde_json::{Value, json};

static NAME: &str = "shell";

fn validate_config(config: Option<Value>) -> Result<(), ActivityError> {
    match config {
        None => Err(ActivityError::InvalidConfig(
            "config is required".to_string(),
        )),
        Some(value) => get_config(value)
            .map(|_| ())
            .map_err(|e| ActivityError::InvalidConfig(e.to_string())),
    }
}

fn get_config(value: Value) -> Result<Config, serde_json::Error> {
    serde_json::from_value(value)
}

fn validate_input(_: Option<Value>) -> Result<(), String> {
    Ok(())
}

#[derive(Deserialize)]
struct Config {
    commands: Vec<ShellCommand>,
}

fn shell_handler(config: ActivityConfig, _input: ActivityInput) -> Result<Value, Vec<String>> {
    let config = get_config(config.0.unwrap()).unwrap();
    let mut results: Vec<String> = vec![];
    config.commands.iter().for_each(|command| {
        trace!("Executing command: {} {:?}", command.program, command.args);
        let output = std::process::Command::from(command)
            .output()
            .unwrap()
            .stdout;
        let std_out = String::from_utf8_lossy(&output);
        trace!("Std Out: {}", std_out);
        results.push(std_out.to_string());
    });
    Ok(Value::Array(
        results.into_iter().map(Value::String).collect(),
    ))
}

pub fn get_shell_module() -> SyncActivityHandler {
    SyncActivityHandler {
        name: "Synchronous Shell Activity".to_string(),
        id: NAME.to_string(),
        description: "Executes a shell command synchronously".to_string(),
        validate_config: Some(validate_config),
        validate_input: Some(validate_input),
        handler: shell_handler,
    }
}

pub struct ActivityParameters {
    pub commands: Vec<ShellCommand>,
}

pub fn get_activity(id: &str, activity_parameters: ActivityParameters) -> Event {
    Event::add_sync(
        id,
        NAME,
        Some(json!({ "commands": activity_parameters.commands })),
    )
}
