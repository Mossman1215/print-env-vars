use astra::{Body, ResponseBuilder, Server};
use base64::{engine::general_purpose, Engine as _};
use std::env;
use std::str::from_utf8;
use std::time::Duration;

fn main() {
    Server::bind("localhost:3000")
        .serve(|_req, _info| {
            // Putting the worker thread to sleep will allow
            // other workers to run.
            std::thread::sleep(Duration::from_secs(1));

            // Regular blocking I/O is fine too!
            let body = match env::var("CERT_CHAIN") {
                Ok(val) => from_utf8(&general_purpose::STANDARD_NO_PAD.decode(val).unwrap())
                    .unwrap()
                    .to_string(),
                Err(_e) => "none".to_string(),
            };
            ResponseBuilder::new()
                .header("Content-Type", "text/html")
                .body(Body::new(body))
                .unwrap()
        })
        .expect("serve failed");
}
