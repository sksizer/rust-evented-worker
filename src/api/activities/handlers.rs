use super::errors::ActivityError;
use serde_json::Value;
use std::pin::Pin;

pub struct ActivityConfig(pub Option<Value>);
pub struct ActivityInput(pub Option<Value>);

pub type ValidateConfig = fn(Option<Value>) -> Result<(), ActivityError>;
pub type ValidateInput = fn(Option<Value>) -> Result<(), String>;

type SyncHandler = fn(ActivityConfig, ActivityInput) -> Result<Value, Vec<String>>;

pub struct SyncActivityHandler {
    pub name: String,
    pub id: String,
    pub description: String,
    pub validate_config: Option<ValidateConfig>,
    pub validate_input: Option<ValidateInput>,
    pub handler: SyncHandler,
}

type AsyncHandler =
    fn(
        ActivityConfig,
        ActivityInput,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Value, Vec<String>>> + Send>>;

pub struct AsyncActivityHandler {
    pub name: String,
    pub id: String,
    pub description: String,
    pub validate_config: Option<ValidateConfig>,
    pub validate_input: Option<ValidateInput>,
    pub handler: AsyncHandler,
}
