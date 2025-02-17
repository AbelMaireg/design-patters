use crate::blocks::block_1::Block1;
use crate::blocks::block_2::Block2;
use crate::blocks::block_3::Block3;
use crate::blocks::handler::Handler;
use crate::request::Request;

#[derive(Default)]
pub struct Client {
    handler: Option<Box<dyn Handler>>,
}

impl Client {
    pub fn new() -> Self {
        let third = Block3 { next: None };

        let second = Block2 {
            next: Some(Box::new(third)),
        };

        let first = Block1 {
            next: Some(Box::new(second)),
        };

        Self {
            handler: Some(Box::new(first)),
        }
    }

    pub fn process(&self, request: Request) {
        if let Some(handler) = &self.handler {
            handler.handle(&request);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::request::Request;

    use super::Client;

    #[test]
    fn test_1() {
        let client = Client::new();
        let request: Request = Request {
            name: "abelm".to_string(),
            password: "12345".to_string(),
        };

        client.process(request);
    }
}
