use serde_json::Value;

// ---------------------------------------------------------------------------
// Typed module system
// ---------------------------------------------------------------------------

/// Type-erased module definition — the engine interacts with this.
/// Config, input, and output are all `serde_json::Value` at this boundary.
pub struct SerdeModule {
    pub id: &'static str,
    pub validate_config: fn(config: &Value) -> bool,
    pub validate_input: fn(input: &Value) -> bool,
    pub execute: fn(config: &Value, input: &Value) -> Result<Value, Vec<String>>,
}

/// Concrete, typed module definition — implementors define this.
///
/// - `CT`: Config type (`DeserializeOwned`)
/// - `IT`: Input type (`DeserializeOwned`)
/// - `OT`: Output type (`Serialize`)
pub struct ModuleDef<CT, IT, OT> {
    pub id: &'static str,
    pub validate_config: fn(config: &CT) -> bool,
    pub validate_input: fn(input: &IT) -> bool,
    pub execute: fn(config: &CT, input: &IT) -> Result<OT, Vec<String>>,
}

/// Macro that performs type erasure: wraps a concrete `ModuleDef` static
/// into a `SerdeModule` by inserting serde (de)serialization at the boundary.
///
/// Usage:
/// ```ignore
/// serde_module!(my_module::MY_STATIC, config: MyConfig, input: MyInput, output: MyOutput)
/// ```
#[macro_export]
macro_rules! serde_module {
    ($static_def:path, config: $CT:ty, input: $IT:ty, output: $OT:ty) => {
        $crate::api::activities::SerdeModule {
            id: $static_def.id,

            validate_config: |raw: &serde_json::Value| {
                let typed: $CT = match serde_json::from_value(raw.clone()) {
                    Ok(v) => v,
                    Err(_) => return false,
                };
                ($static_def.validate_config)(&typed)
            },

            validate_input: |raw: &serde_json::Value| {
                let typed: $IT = match serde_json::from_value(raw.clone()) {
                    Ok(v) => v,
                    Err(_) => return false,
                };
                ($static_def.validate_input)(&typed)
            },

            execute: |raw_cfg: &serde_json::Value, raw_input: &serde_json::Value| {
                let cfg: $CT = serde_json::from_value(raw_cfg.clone())
                    .map_err(|e| vec![format!("config deserialization failed: {}", e)])?;
                let input: $IT = serde_json::from_value(raw_input.clone())
                    .map_err(|e| vec![format!("input deserialization failed: {}", e)])?;
                let result: $OT = ($static_def.execute)(&cfg, &input)?;
                serde_json::to_value(result)
                    .map_err(|e| vec![format!("output serialization failed: {}", e)])
            },
        }
    };
}
