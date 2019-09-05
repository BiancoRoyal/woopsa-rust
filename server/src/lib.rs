extern crate woopsa;

pub mod core;
pub mod extension;

use crate::core::request_handler::WoopsaRequestHandler;
use crate::core::check_request_type::WoopsaCheckRequestType;

trait Server {
    fn handle_request(&self);
    fn check_request_data_for_complete() -> WoopsaCheckRequestType;
}

struct WoopsaServer {
    prefix: String,
    request_handler: WoopsaRequestHandler,
    request_data: String
}

impl WoopsaServer {}

impl Server for WoopsaServer {
    fn handle_request(&self) {
        self.request_handler.handle();
    }

    fn check_request_data_for_complete() -> WoopsaCheckRequestType {
        return WoopsaCheckRequestType::Complete;
    }
}
