use crate::fixtures::get_test_activity_modules;
use crate::runner::Registry;

pub fn get_registry() -> Registry {
    Registry::new(Some(get_test_activity_modules()), None)
}
