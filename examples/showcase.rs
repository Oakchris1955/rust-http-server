use oak_http_server::HttpServer;

fn main() {
	let mut server = HttpServer::new("localhost", 2300 as u16);
	server.on("/test", |request, response| {
		response.send(format!(
			"Your current query options are:\n{}",
			request.target.queries.iter().map(|x| x.to_string()).collect::<String>()
		))
	});

	server.start();
}
