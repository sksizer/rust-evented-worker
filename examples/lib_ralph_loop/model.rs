mod specification;
mod work_item;

pub struct Database {
    pub work_items: Vec<work_item::WorkItem>,
}

impl Database {
    pub fn new() -> Database {
        Self {
            work_items: Vec::new(),
        }
    }
}
