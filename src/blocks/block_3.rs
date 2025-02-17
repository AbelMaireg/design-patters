use super::handler::Handler;
use crate::request::Request;

pub struct Block3 {
    pub next: Option<Box<dyn Handler>>,
}

impl Handler for Block3 {
    fn get_next_handler(&self) -> &Option<Box<dyn Handler>> {
        &self.next
    }

    fn do_handle(&self, request: &Request) -> bool {
        match request.name.eq("administrator") {
            true => true,
            false => false,
        }
    }
}
