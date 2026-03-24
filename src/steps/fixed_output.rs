//! This is a test step module that basically just passes the input through to output

use log::trace;
use serde_json::Value;
use crate::api::steps::{SyncStepHandler, StepConfig, StepInput};

static NAME: &str = "fixed_output";

fn validate_config(_: Option<Value>) -> Result<(), String> { Ok(()) }
fn validate_input(_: Option<Value>) -> Result<(), String> { Ok(()) }

fn fixed_output_handler(config: StepConfig, input: StepInput) -> Result<Value, Vec<String>> {
    trace!("Fixed Output - config: {:?}", config.0);
    Ok(config.0.unwrap_or(Value::Null))
}

// TODO - implement actual echoing for testing
pub fn get_fixed_output() -> SyncStepHandler {
    SyncStepHandler {
        name: "Synchronous Echo Step".to_string(),
        id: NAME.to_string(),
        description: "Passes input to output synchronously".to_string(),
        validate_config: Some(validate_config),
        validate_input: Some(validate_input),
        handler: fixed_output_handler,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn passes_input_through_to_output() {
        let module = get_fixed_output();
        let input = json!({ "message": "hello" });
        let output = (module.handler)(StepConfig(None), StepInput(Some(input.clone())));
        assert_eq!(output.unwrap(), input);
    }

    #[test]
    fn none_input_returns_null() {
        let module = get_fixed_output();
        let output = (module.handler)(StepConfig(None), StepInput(None));
        assert_eq!(output.unwrap(), Value::Null);
    }
}
