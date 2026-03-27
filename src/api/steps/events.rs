use crate::api::steps::{StepId, StepKind};
use serde_json::Value;

// --- Payloads ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AddStepPayload {
    pub id: StepId,
    pub kind: StepKind,
    pub config: Option<Value>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CompletePayload {
    pub id: StepId,
    pub output: Option<Value>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FailurePayload {
    pub id: StepId,
    pub reason: Option<String>,
}

// --- Step events ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StepEvent {
    AddSync(AddStepPayload),
    AddAsync(AddStepPayload),
    Start(StepId),
    Complete(CompletePayload),
    Failed(FailurePayload),
    Error(FailurePayload),
}

impl StepEvent {
    pub fn add_sync(id: impl Into<String>, kind: impl Into<String>, config: Option<Value>) -> Self {
        StepEvent::AddSync(AddStepPayload {
            id: id.into(),
            kind: kind.into(),
            config,
        })
    }

    pub fn add_async(id: impl Into<String>, kind: impl Into<String>, config: Option<Value>) -> Self {
        StepEvent::AddAsync(AddStepPayload {
            id: id.into(),
            kind: kind.into(),
            config,
        })
    }

    pub fn start(id: impl Into<String>) -> Self {
        StepEvent::Start(id.into())
    }

    pub fn complete(id: impl Into<String>, output: Option<Value>) -> Self {
        StepEvent::Complete(CompletePayload {
            id: id.into(),
            output,
        })
    }

    pub fn failed(id: impl Into<String>, reason: Option<String>) -> Self {
        StepEvent::Failed(FailurePayload {
            id: id.into(),
            reason,
        })
    }

    pub fn error(id: impl Into<String>, reason: Option<String>) -> Self {
        StepEvent::Error(FailurePayload {
            id: id.into(),
            reason,
        })
    }

    pub fn step_id(&self) -> &StepId {
        match self {
            StepEvent::AddSync(p) => &p.id,
            StepEvent::AddAsync(p) => &p.id,
            StepEvent::Start(id) => id,
            StepEvent::Complete(p) => &p.id,
            StepEvent::Failed(p) => &p.id,
            StepEvent::Error(p) => &p.id,
        }
    }
}
