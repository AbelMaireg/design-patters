use crate::request::Request;

pub trait Handler {
    fn get_next_handler(&self) -> &Option<Box<dyn Handler>>;

    fn handle(&self, request: &Request) {
        if self.do_handle(request) {
            if let Some(handler) = self.get_next_handler() {
                println!("into handler");
                handler.handle(request);
            } else {
                println!("no more handlers")
            }
        } else {
            println!("error happended");
        }
    }

    fn do_handle(&self, request: &Request) -> bool;
}
