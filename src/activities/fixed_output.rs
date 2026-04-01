//! Fixed output activity — returns config as output

use crate::api::activities::ModuleDef;
use log::trace;
use serde_json::Value;

pub static FIXED_OUTPUT: ModuleDef<Value, Value, Value> = ModuleDef {
    id: "fixed_output",
    validate_config: |_| true,
    validate_input: |_| true,
    execute: |config, _input| {
        trace!("Fixed Output - config: {:?}", config);
        Ok(config.clone())
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn returns_config_as_output() {
        let config = json!({ "message": "hello" });
        let output = (FIXED_OUTPUT.execute)(&config, &Value::Null);
        assert_eq!(output.unwrap(), config);
    }

    #[test]
    fn null_config_returns_null() {
        let output = (FIXED_OUTPUT.execute)(&Value::Null, &Value::Null);
        assert_eq!(output.unwrap(), Value::Null);
    }
}
