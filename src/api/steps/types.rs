use serde_json::Value;

pub type StepKind = String;

#[derive(Clone, Debug)]
pub struct StepCore {
    pub id: String,
    pub kind: StepKind,
    pub input: Option<Value>,
}

#[derive(Clone, Debug)]
pub enum SyncStep {
    Ready(StepCore),
    Completed { core: StepCore, output: Option<Value> },
    Failed { core: StepCore, failure: Option<String> },
    Error { core: StepCore, failure: Option<String> },
}

impl SyncStep {
    pub fn core(&self) -> &StepCore {
        match self {
            SyncStep::Ready(core) => core,
            SyncStep::Completed { core, .. } => core,
            SyncStep::Failed { core, .. } => core,
            SyncStep::Error { core, .. } => core,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AsyncStep {
    Ready(StepCore),
    Running(StepCore),
    Completed { core: StepCore, output: Option<Value> },
    Failed { core: StepCore, failure: Option<String> },
    Error { core: StepCore, failure: Option<String> },
}

impl AsyncStep {
    pub fn core(&self) -> &StepCore {
        match self {
            AsyncStep::Ready(core) => core,
            AsyncStep::Running(core) => core,
            AsyncStep::Completed { core, .. } => core,
            AsyncStep::Failed { core, .. } => core,
            AsyncStep::Error { core, .. } => core,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Step {
    Sync(SyncStep),
    Async(AsyncStep),
}

impl Step {
    pub fn core(&self) -> &StepCore {
        match self {
            Step::Sync(s) => s.core(),
            Step::Async(a) => a.core(),
        }
    }

    pub fn id(&self) -> &str {
        &self.core().id
    }

    pub fn kind(&self) -> &str {
        &self.core().kind
    }

    pub fn is_closed(&self) -> bool {
        match self {
            Step::Sync(s) => matches!(s, SyncStep::Completed { .. } | SyncStep::Failed { .. } | SyncStep::Error { .. }),
            Step::Async(a) => matches!(a, AsyncStep::Completed { .. } | AsyncStep::Failed { .. } | AsyncStep::Error { .. }),
        }
    }

    pub fn is_runnable(&self) -> bool {
        match self {
            Step::Sync(s) => matches!(s, SyncStep::Ready(_)),
            Step::Async(a) => matches!(a, AsyncStep::Ready(_) | AsyncStep::Running(_)),
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Step::Sync(SyncStep::Completed { .. }) | Step::Async(AsyncStep::Completed { .. }))
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Step::Sync(SyncStep::Failed { .. }) | Step::Async(AsyncStep::Failed { .. }))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Step::Sync(SyncStep::Error { .. }) | Step::Async(AsyncStep::Error { .. }))
    }
}
