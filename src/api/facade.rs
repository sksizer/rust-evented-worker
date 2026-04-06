#[allow(dead_code)]
pub struct Facade {}

#[allow(dead_code)]
impl Facade {
    fn new() -> Self {
        Facade {}
    }

    // register_sync_handler(),
    // register_async_handler(),
    // add_activity(),
    // start()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _facade = Facade::new();
    }
}
