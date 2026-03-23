use crate::api::steps::SyncStepModule;

static NAME: &str = "shell";

pub fn get_shell_module() -> SyncStepModule {
    SyncStepModule {
        name: "Synchronous Shell Step".to_string(),
        id: NAME.to_string(),
        description: "Executes a shell command synchronously".to_string(),
        handler: |input| {
            let output = std::process::Command::new("ls").output();
            println!("Shell Module - input: {:?}", input);
            None
        }
    }
}
