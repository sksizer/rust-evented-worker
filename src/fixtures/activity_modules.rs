use crate::activities::{echo, fixed_output, shell};
use crate::api::activities::SerdeModule;
use crate::serde_module;

pub fn get_test_serde_modules() -> Vec<SerdeModule> {
    vec![
        serde_module!(shell::SHELL, config: shell::ShellConfig, input: serde_json::Value, output: Vec<String>),
        serde_module!(echo::ECHO, config: serde_json::Value, input: serde_json::Value, output: serde_json::Value),
        serde_module!(fixed_output::FIXED_OUTPUT, config: serde_json::Value, input: serde_json::Value, output: serde_json::Value),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::Registry;

    #[test]
    fn test_get_test_serde_modules() {
        let modules = get_test_serde_modules();
        let mut registry = Registry::new();
        for m in modules {
            registry.register_module(m).unwrap();
        }
        let module = registry.get_module("echo");
        assert!(module.is_some());
        assert_eq!(module.unwrap().id, "echo");
    }
}
