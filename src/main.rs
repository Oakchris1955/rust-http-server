use oak_http_server::HttpServer;

fn main() {
	let mut server = HttpServer::new("localhost", 2300 as u16);
	server.on("/test", |request, response| {
		response.send(format!(
			"Your current query options are:\n{}",
			request.target.queries.iter().map(|x| {
				format!("{}: {}\n", x.name, x.value.clone().unwrap_or(String::new()))
			}).collect::<String>()
		))
	});

	server.start();
}
