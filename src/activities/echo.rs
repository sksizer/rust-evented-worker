//! Echo activity — passes input through to output

use crate::api::activities::ModuleDef;
use log::trace;
use serde_json::Value;

pub static ECHO: ModuleDef<Value, Value, Value> = ModuleDef {
    id: "echo",
    validate_config: |_| true,
    validate_input: |_| true,
    execute: |_config, input| {
        trace!("Echo Module - input: {:?}", input);
        Ok(input.clone())
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn passes_input_through_to_output() {
        let input = json!({ "message": "hello" });
        let output = (ECHO.execute)(&Value::Null, &input);
        assert_eq!(output.unwrap(), input);
    }

    #[test]
    fn null_input_returns_null() {
        let output = (ECHO.execute)(&Value::Null, &Value::Null);
        assert_eq!(output.unwrap(), Value::Null);
    }
}
