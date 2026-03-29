//! This is a test activity module that returns config as output

use crate::api::activities::{ActivityConfig, ActivityError, ActivityInput, SyncActivityHandler};
use log::trace;
use serde_json::Value;

static NAME: &str = "fixed_output";

fn validate_config(_: Option<Value>) -> Result<(), ActivityError> {
    Ok(())
}
fn validate_input(_: Option<Value>) -> Result<(), String> {
    Ok(())
}

fn fixed_output_handler(config: ActivityConfig, input: ActivityInput) -> Result<Value, Vec<String>> {
    trace!("Fixed Output - config: {:?}", config.0);
    Ok(config.0.unwrap_or(Value::Null))
}

pub fn get_fixed_output() -> SyncActivityHandler {
    SyncActivityHandler {
        name: "Synchronous Fixed Output Activity".to_string(),
        id: NAME.to_string(),
        description: "Returns config as output synchronously".to_string(),
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
    fn returns_config_as_output() {
        let module = get_fixed_output();
        let config = json!({ "message": "hello" });
        let output =
            (module.handler)(ActivityConfig(Some(config.clone())), ActivityInput(None));
        assert_eq!(output.unwrap(), config);
    }

    #[test]
    fn none_input_returns_null() {
        let module = get_fixed_output();
        let output = (module.handler)(ActivityConfig(None), ActivityInput(None));
        assert_eq!(output.unwrap(), Value::Null);
    }
}
