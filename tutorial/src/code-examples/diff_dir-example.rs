extern crate oak_http_server;
use oak_http_server::{handlers::read_diff_dir, Server};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = Server::new(hostname, port);

    // The handler will serve static files for the '/www' path from the local '/diff' path
    server.on_directory("/www", read_diff_dir("/diff"));

    server.start(|| {
        println!("HTTP server is now running...");
    });
}
