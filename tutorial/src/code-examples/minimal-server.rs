extern crate oak_http_server;
use oak_http_server::{Server, Status};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = Server::new(hostname, port);

    server.on("/hello", |_request, response| {
        response.end_with("Hello, World!")
    });

    server.start(|| {
        println!("HTTP server is now running...");
    });
}
