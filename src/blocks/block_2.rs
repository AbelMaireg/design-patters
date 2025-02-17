use super::handler::Handler;
use crate::request::Request;

pub struct Block2 {
    pub next: Option<Box<dyn Handler>>,
}

impl Handler for Block2 {
    fn get_next_handler(&self) -> &Option<Box<dyn Handler>> {
        &self.next
    }

    fn do_handle(&self, request: &Request) -> bool {
        match request.password.len() > 5 {
            true => true,
            false => false,
        }
    }
}
