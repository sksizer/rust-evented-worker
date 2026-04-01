use crate::api::activities::SerdeModule;

pub struct Registry {
    modules: Vec<SerdeModule>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            modules: Vec::new(),
        }
    }

    pub fn register_module(&mut self, module: SerdeModule) -> Result<(), String> {
        if self.get_module(module.id).is_some() {
            return Err(format!("Module with id {} already exists", module.id));
        }
        self.modules.push(module);
        Ok(())
    }

    pub fn get_module(&self, activity_kind: &str) -> Option<&SerdeModule> {
        self.modules
            .iter()
            .find(|m| m.id == activity_kind)
    }
}
