use crate::api::steps::SyncStepModule;
use crate::steps::{get_echo_module, get_shell_module};

pub fn get_test_step_modules() -> Vec<SyncStepModule> {
    vec![
        get_shell_module(),
        get_echo_module(),
    ]
}

#[cfg(test)]
mod tests {
    use crate::runner::Registry;
    use super::*;

    #[test]
    fn test_get_test_step_modules() {
        let modules = get_test_step_modules();
        let registry = Registry::new(Some(modules), None);
        let step_module = registry.get_sync_module("echo");
        assert_eq!(step_module.unwrap().name, "Synchronous Echo Step");
    }
}