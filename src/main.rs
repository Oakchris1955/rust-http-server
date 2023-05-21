use oak_http_server::HttpServer;

fn main() {
	let server = HttpServer::new("localhost", 2300 as u16);
	server.start();
}
