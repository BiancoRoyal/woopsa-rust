extern crate solicit;

use crate::core::*;
use crate::extensions::*;

use solicit::http::Response;
use solicit::server::SimpleServer;

use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut server = SimpleServer::new(stream, |req| {
        println!("Received request:");
        for header in req.headers.iter() {
            println!(
                "  {}: {}",
                str::from_utf8(&header.0).unwrap(),
                str::from_utf8(&header.1).unwrap()
            );
        }
        println!("Body:\n{}", str::from_utf8(&req.body).unwrap());

        // Return a dummy response for every request
        Response {
            headers: vec![
                (b":status".to_vec(), b"200".to_vec()),
                (b"x-solicit".to_vec(), b"Hello, World!".to_vec()),
            ],
            body: req.body.to_vec(),
            stream_id: req.stream_id,
        }
    })
    .unwrap();
    while let Ok(_) = server.handle_next() {}
    println!("Server done (client disconnected)");
}

struct WoopsaServer {
    listener: TcpListener,
}

impl WoopsaServer {
    fn start(&mut self, address: &str) {
        self.listener = TcpListener::bind(address).unwrap();
        for stream in self.listener.incoming() {
            thread::spawn(move || handle_client(stream.unwrap()));
        }
    }
}
