extern crate oak_http_server;
use oak_http_server::{Request, Response, Server};

fn example_handler_func(_request: Request, response: Response) {
    // This handler responds to the HTTP request with a predefined string
    response.send("I am a concrete function handler!!!");
}

fn main() {
    let example_handler_closure = |_request: Request, response: Response| {
        // This handler responds to the HTTP request with a predefined string
        response.send("And I am a closure!!!");
    };

    let hostname = "localhost";
    let port: u16 = 2300;

    // Create a Server instance
    let mut server = Server::new(hostname, port);

    // Append generic function handler
    server.on("/function", example_handler_func);

    // Append closure handler that responds only to `GET` requests
    server.on_get("/closure", example_handler_closure);

    // Start the HTTP server
    server.start(|| {
        println!("HTTP server is now running...");
    });
}
