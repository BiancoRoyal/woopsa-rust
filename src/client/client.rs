extern crate solicit;

use crate::core::*;
use crate::extensions::*;

use solicit::client::Client;
use solicit::http::client::CleartextConnector;

use std::str;
use std::thread;

struct WoopsaClient {
    async_client: Client,
}

impl WoopsaClient {
    fn start(&mut self) {
        // Connect to a server that supports HTTP/2
        self.async_client =
            Client::with_connector(CleartextConnector::new("http2bin.org")).unwrap();

        // Issue 5 requests from 5 different threads concurrently and wait for all
        // threads to receive their response.
        let threads: Vec<_> = (0..5)
            .map(|i| {
                let this = self.async_client.clone();
                thread::spawn(move || {
                    let resp = this
                        .get(b"/get", &[(b"x-thread".to_vec(), vec![b'0' + i])])
                        .unwrap();
                    let response = resp.recv().unwrap();

                    println!(
                        "Thread {} got response ... {}",
                        i,
                        response.status_code().ok().unwrap()
                    );

                    response
                })
            })
            .collect();

        let responses: Vec<_> = threads.into_iter().map(|thread| thread.join()).collect();

        println!("All threads joined. Full responses are:");
        for response in responses.into_iter() {
            let response = response.unwrap();
            println!("The response contains the following headers:");
            for header in response.headers.iter() {
                println!(
                    "  {}: {}",
                    str::from_utf8(&header.0).unwrap(),
                    str::from_utf8(&header.1).unwrap()
                );
            }
            println!("{}", str::from_utf8(&response.body).unwrap());
        }
    }
}
