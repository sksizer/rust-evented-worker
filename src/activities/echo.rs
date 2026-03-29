//! This is a test activity module that basically just passes the input through to output

use crate::api::activities::{ActivityConfig, ActivityError, ActivityInput, SyncActivityHandler};
use log::trace;
use serde_json::Value;

static NAME: &str = "echo";

fn validate_config(_: Option<Value>) -> Result<(), ActivityError> {
    Ok(())
}
fn validate_input(_: Option<Value>) -> Result<(), String> {
    Ok(())
}

fn echo_handler(_config: ActivityConfig, input: ActivityInput) -> Result<Value, Vec<String>> {
    trace!("Echo Module - input: {:?}", input.0);
    Ok(input.0.unwrap_or(Value::Null))
}

// TODO - implement actual echoing for testing
pub fn get_echo_module() -> SyncActivityHandler {
    SyncActivityHandler {
        name: "Synchronous Echo Activity".to_string(),
        id: NAME.to_string(),
        description: "Passes input to output synchronously".to_string(),
        validate_config: Some(validate_config),
        validate_input: Some(validate_input),
        handler: echo_handler,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn passes_input_through_to_output() {
        let module = get_echo_module();
        let input = json!({ "message": "hello" });
        let output =
            (module.handler)(ActivityConfig(None), ActivityInput(Some(input.clone())));
        assert_eq!(output.unwrap(), input);
    }

    #[test]
    fn none_input_returns_null() {
        let module = get_echo_module();
        let output = (module.handler)(ActivityConfig(None), ActivityInput(None));
        assert_eq!(output.unwrap(), Value::Null);
    }
}
