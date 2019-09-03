extern crate woopsa_http2;
extern crate woopsa;

trait RequestHandler {
    fn handle(&self);
}

pub struct WoopsaRequestHandler {}

impl WoopsaRequestHandler {
    pub fn handle(&self) {
        unimplemented!();
    }
}

impl RequestHandler for WoopsaRequestHandler {
    fn handle(&self) {
        self.handle();
    }
}