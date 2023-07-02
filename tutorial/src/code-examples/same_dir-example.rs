extern crate oak_http_server;
use oak_http_server::{Server, handlers::read_same_dir};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = Server::new(hostname, port);

	// The handler will serve static files from the local '/www' path
    server.on_directory("/www", read_same_dir);

    server.start(|| {
        println!("HTTP server is now running...");
    });
}
