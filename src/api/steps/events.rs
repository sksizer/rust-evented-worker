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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SystemErrorData {
    pub step_id: StepId,
    pub source: String,
    pub errors: Vec<String>,
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

// --- System events ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SystemEvent {
    Error(SystemErrorData),
}

// --- Wrapper enum ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Event {
    Step(StepEvent),
    System(SystemEvent),
}

impl Event {
    // Convenience constructors that produce Event::Step variants
    pub fn add_sync(id: impl Into<String>, kind: impl Into<String>, config: Option<Value>) -> Self {
        Event::Step(StepEvent::add_sync(id, kind, config))
    }

    pub fn add_async(id: impl Into<String>, kind: impl Into<String>, config: Option<Value>) -> Self {
        Event::Step(StepEvent::add_async(id, kind, config))
    }

    pub fn start(id: impl Into<String>) -> Self {
        Event::Step(StepEvent::start(id))
    }

    pub fn complete(id: impl Into<String>, output: Option<Value>) -> Self {
        Event::Step(StepEvent::complete(id, output))
    }

    pub fn failed(id: impl Into<String>, reason: Option<String>) -> Self {
        Event::Step(StepEvent::failed(id, reason))
    }

    pub fn error(id: impl Into<String>, reason: Option<String>) -> Self {
        Event::Step(StepEvent::error(id, reason))
    }

    pub fn system_error(data: SystemErrorData) -> Self {
        Event::System(SystemEvent::Error(data))
    }
}

impl From<StepEvent> for Event {
    fn from(e: StepEvent) -> Self { Event::Step(e) }
}

impl From<SystemEvent> for Event {
    fn from(e: SystemEvent) -> Self { Event::System(e) }
}
