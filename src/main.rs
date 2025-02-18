pub mod blocks;
pub mod client;
pub mod request;

use crate::request::Request;

use crate::client::Client;

fn main() {
    let client = Client::new();

    let request = Request {
        name: "administrator".to_string(),
        password: "123456".to_string(),
    };

    client.process(request);
}
