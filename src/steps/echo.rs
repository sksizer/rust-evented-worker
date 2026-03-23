//! This is a test step module that basically just passes the input through to output
use crate::api::steps::SyncStepModule;

static NAME: &str = "echo";

// TODO - implement actual echoing for testing
pub fn get_echo_module() -> SyncStepModule {
    SyncStepModule {
        name: "Synchronous Echo Step".to_string(),
        id: NAME.to_string(),
        description: "Passes input to output synchronously".to_string(),
        handler: |input| {
            println!("Echo Module - input: {:?}", input);
            input
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn passes_input_through_to_output() {
        let module = get_echo_module();
        let input = Some(json!({ "message": "hello" }));
        let output = (module.handler)(input.clone());
        assert_eq!(output, input);
    }

    #[test]
    fn none_input_returns_none() {
        let module = get_echo_module();
        assert_eq!((module.handler)(None), None);
    }
}
