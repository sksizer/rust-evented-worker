use serde_json::Value;
use crate::api::steps::StepCore;

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

    pub fn config(&self) -> Option<&Value> {
        self.core().config.as_ref()
    }

    pub fn input(&self) -> Option<&Value> {
        match self {
            Step::Sync(s) => s.input(),
            Step::Async(a) => a.input(),
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            Step::Sync(s) => matches!(s, SyncStep::Completed { .. } | SyncStep::Failed { .. } | SyncStep::Error { .. }),
            Step::Async(a) => matches!(a, AsyncStep::Completed { .. } | AsyncStep::Failed { .. } | AsyncStep::Error { .. }),
        }
    }

    pub fn is_runnable(&self) -> bool {
        match self {
            Step::Sync(s) => matches!(s, SyncStep::Ready { .. }),
            Step::Async(a) => matches!(a, AsyncStep::Ready(_) | AsyncStep::Running { .. }),
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


#[derive(Clone, Debug)]
pub enum SyncStep {
    Ready { core: StepCore, input: Option<Value> },
    Completed { core: StepCore, input: Option<Value>, output: Option<Value> },
    Failed { core: StepCore, input: Option<Value>, failure: Option<String> },
    Error { core: StepCore, input: Option<Value>, failure: Option<String> },
}

impl SyncStep {
    pub fn core(&self) -> &StepCore {
        match self {
            SyncStep::Ready { core, .. } => core,
            SyncStep::Completed { core, .. } => core,
            SyncStep::Failed { core, .. } => core,
            SyncStep::Error { core, .. } => core,
        }
    }

    pub fn input(&self) -> Option<&Value> {
        match self {
            SyncStep::Ready { input, .. } => input.as_ref(),
            SyncStep::Completed { input, .. } => input.as_ref(),
            SyncStep::Failed { input, .. } => input.as_ref(),
            SyncStep::Error { input, .. } => input.as_ref(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AsyncStep {
    Ready(StepCore),
    Running { core: StepCore, input: Option<Value> },
    Completed { core: StepCore, input: Option<Value>, output: Option<Value> },
    Failed { core: StepCore, input: Option<Value>, failure: Option<String> },
    Error { core: StepCore, input: Option<Value>, failure: Option<String> },
}

impl AsyncStep {
    pub fn core(&self) -> &StepCore {
        match self {
            AsyncStep::Ready(core) => core,
            AsyncStep::Running { core, .. } => core,
            AsyncStep::Completed { core, .. } => core,
            AsyncStep::Failed { core, .. } => core,
            AsyncStep::Error { core, .. } => core,
        }
    }

    pub fn input(&self) -> Option<&Value> {
        match self {
            AsyncStep::Ready(_) => None,
            AsyncStep::Running { input, .. } => input.as_ref(),
            AsyncStep::Completed { input, .. } => input.as_ref(),
            AsyncStep::Failed { input, .. } => input.as_ref(),
            AsyncStep::Error { input, .. } => input.as_ref(),
        }
    }
}
