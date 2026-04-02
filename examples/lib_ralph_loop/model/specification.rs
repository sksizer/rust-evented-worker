use std::marker::PhantomData;

pub struct New;
pub struct Attempted {
    pub count: u8,
}
pub struct Complete {
    pub evidence: Vec<SpecEvidence>,
}
pub struct Failed {
    pub attempts: u8,
    pub messages: Vec<String>,
}

pub struct SpecEvidence {
    pub file_path: String,
    // Describe how the specification was met with aforementioned evidence
    pub description: String,
}

// The state is a generic parameter, not a field
pub struct Specification<S> {
    work_item_id: String,
    id: String,
    body: String,
    state: S,
}

// Methods available in ALL states
impl<S> Specification<S> {
    pub fn id(&self) -> &str {
        &self.id
    }
}

// Methods only available when state is New
impl Specification<New> {
    pub fn create(work_item_id: String, id: String, body: String) -> Self {
        Specification {
            work_item_id,
            id,
            body,
            state: New,
        }
    }

    // Transitions: New -> Complete or New -> Attempted
    pub fn complete(self, evidence: Vec<SpecEvidence>) -> Specification<Complete> {
        Specification {
            work_item_id: self.work_item_id,
            id: self.id,
            body: self.body,
            state: Complete { evidence },
        }
    }

    pub fn attempt(self) -> Specification<Attempted> {
        Specification {
            work_item_id: self.work_item_id,
            id: self.id,
            body: self.body,
            state: Attempted { count: 1 },
        }
    }
}

// Methods only available when state is Attempted
impl Specification<Attempted> {
    pub fn retry(mut self) -> Specification<Attempted> {
        self.state.count += 1;
        self
    }

    pub fn complete(self, evidence: Vec<SpecEvidence>) -> Specification<Complete> {
        Specification {
            work_item_id: self.work_item_id,
            id: self.id,
            body: self.body,
            state: Complete { evidence },
        }
    }

    pub fn give_up(self, messages: Vec<String>) -> Specification<Failed> {
        Specification {
            work_item_id: self.work_item_id,
            id: self.id,
            body: self.body,
            state: Failed {
                attempts: self.state.count,
                messages,
            },
        }
    }
}
