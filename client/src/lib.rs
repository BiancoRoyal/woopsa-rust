extern crate woopsa;

pub mod core;

use crate::core::client_type::WoopsaClientType;

trait Client {
    fn send_request();
}

struct WoopsaClient {
    server_url: String,
    client_type: WoopsaClientType
}

impl WoopsaClient {
    fn send_request() {
        // Todo
    }
}
