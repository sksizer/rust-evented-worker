use crate::fixtures::get_test_serde_modules;
use crate::runner::Registry;

pub fn get_registry() -> Registry {
    let mut registry = Registry::new();
    for m in get_test_serde_modules() {
        registry.register_module(m).unwrap();
    }
    registry
}
