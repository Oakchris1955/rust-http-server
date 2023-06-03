use oak_http_server::HttpServer;

fn main() {
	let hostname = "localhost";
	let port: u16 = 2300;

	let mut server = HttpServer::new(hostname, port);
	server.on("/test", |request, response| {
		response.send(format!(
			"Your current query options are:\n{}",
			request.target.queries.iter().map(|x| x.to_string()).collect::<String>()
		))
	});

	server.start();
}
