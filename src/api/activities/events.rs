use crate::api::activities::{Activity, ActivityId, ActivityKind};
use serde_json::Value;

// --- Payloads ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AddActivityPayload {
    pub id: ActivityId,
    pub kind: ActivityKind,
    pub config: Option<Value>,
    pub depends_on: Option<Vec<ActivityId>>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CompletePayload {
    pub id: ActivityId,
    pub output: Option<Value>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FailurePayload {
    pub id: ActivityId,
    pub reason: Option<String>,
}

// --- Activity events ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ActivityEvent {
    AddSync(AddActivityPayload),
    AddAsync(AddActivityPayload),
    Start(ActivityId),
    Complete(CompletePayload),
    Failed(FailurePayload),
    Error(FailurePayload),
    Retry(ActivityId),
}

impl ActivityEvent {
    pub fn add_sync(
        id: impl Into<String>,
        kind: impl Into<String>,
        config: Option<Value>,
        depends_on: Option<Vec<ActivityId>>,
    ) -> Self {
        ActivityEvent::AddSync(AddActivityPayload {
            id: id.into(),
            kind: kind.into(),
            config,
            depends_on,
        })
    }

    pub fn add_async(
        id: impl Into<String>,
        kind: impl Into<String>,
        config: Option<Value>,
    ) -> Self {
        ActivityEvent::AddAsync(AddActivityPayload {
            id: id.into(),
            kind: kind.into(),
            config,
            depends_on: None,
        })
    }

    pub fn start(id: impl Into<String>) -> Self {
        ActivityEvent::Start(id.into())
    }

    pub fn complete(id: impl Into<String>, output: Option<Value>) -> Self {
        ActivityEvent::Complete(CompletePayload {
            id: id.into(),
            output,
        })
    }

    pub fn failed(id: impl Into<String>, reason: Option<String>) -> Self {
        ActivityEvent::Failed(FailurePayload {
            id: id.into(),
            reason,
        })
    }

    pub fn error(id: impl Into<String>, reason: Option<String>) -> Self {
        ActivityEvent::Error(FailurePayload {
            id: id.into(),
            reason,
        })
    }

    pub fn retry(id: impl Into<String>) -> Self {
        ActivityEvent::Retry(id.into())
    }

    pub fn activity_id(&self) -> &ActivityId {
        match self {
            ActivityEvent::AddSync(p) => &p.id,
            ActivityEvent::AddAsync(p) => &p.id,
            ActivityEvent::Start(id) => id,
            ActivityEvent::Complete(p) => &p.id,
            ActivityEvent::Failed(p) => &p.id,
            ActivityEvent::Error(p) => &p.id,
            ActivityEvent::Retry(id) => id,
        }
    }
}
