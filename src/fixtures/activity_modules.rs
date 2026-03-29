use crate::api::activities::SyncActivityHandler;
use crate::activities::{get_echo_module, get_fixed_output, get_shell_module};

pub fn get_test_activity_modules() -> Vec<SyncActivityHandler> {
    vec![get_shell_module(), get_echo_module(), get_fixed_output()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::Registry;

    #[test]
    fn test_get_test_activity_modules() {
        let modules = get_test_activity_modules();
        let registry = Registry::new(Some(modules), None);
        let activity_module = registry.get_sync_module("echo");
        assert_eq!(
            activity_module.unwrap().name,
            "Synchronous Echo Activity"
        );
    }
}
