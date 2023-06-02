use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream, SocketAddr, Shutdown};
use std::process::exit;

mod utils;
use utils::*;

mod enums;
pub use enums::*;

mod structs;
pub use structs::*;

const VERSION: &str = "HTTP/1.1";

enum HandlerHttpMethod {
	Specific(HttpMethod),
	Any
}

type HandlerCallback = fn(HttpRequest, HttpResponse);

type Handler = (HandlerHttpMethod, HandlerCallback);

pub struct HttpServer {
	pub hostname: String,
	pub port: u16,

	handlers: HashMap<String, Vec<Handler>>
}

impl HttpServer {
	pub fn new<S, N>(hostname: S, port:N) -> Self where S: Into<String>, N: Into<u16> {
		Self {
			hostname: hostname.into(),
			port: port.into(),

			handlers: HashMap::new()
		}
	}

	pub fn start(&self) {// Initiate a TCP Listener at localhost port 2300 (port and IP address are subject to change)
		let listener = TcpListener::bind(format!("{}:{}", self.hostname, self.port)).unwrap_or_else(|err| {
			eprintln!("Couldn't initiate TCP server. Error message: {}", err);
			exit(1);
		});
		println!("Successfully initiated server");

		// For each incoming connection request, accept connection and pass control of connection to "handle_client" function
		for stream in listener.incoming() {
			match stream {
				Ok(stream) => {
					self.handle_connection(stream);
				}
				Err(e) => {
					eprintln!("Failed to establish a new connection. Error message: {}", e);
				}
			}
		}
	}
	

	pub fn on<S>(&mut self, path: S, handler: HandlerCallback) where S: Into<String> {
		self.append_handler(path.into(), HandlerHttpMethod::Any, handler);
	}

	pub fn on_get<S>(&mut self, path: S, handler: HandlerCallback) where S: Into<String> {
		self.append_handler(path.into(), HandlerHttpMethod::Specific(HttpMethod::GET), handler);
	}

	pub fn on_head<S>(&mut self, path: S, handler: HandlerCallback) where S: Into<String> {
		self.append_handler(path.into(), HandlerHttpMethod::Specific(HttpMethod::HEAD), handler);
	}

	pub fn on_post<S>(&mut self, path: S, handler: HandlerCallback) where S: Into<String> {
		self.append_handler(path.into(), HandlerHttpMethod::Specific(HttpMethod::POST), handler);
	}

	pub fn on_put<S>(&mut self, path: S, handler: HandlerCallback) where S: Into<String> {
		self.append_handler(path.into(), HandlerHttpMethod::Specific(HttpMethod::PUT), handler);
	}

	pub fn on_delete<S>(&mut self, path: S, handler: HandlerCallback) where S: Into<String> {
		self.append_handler(path.into(), HandlerHttpMethod::Specific(HttpMethod::DELETE), handler);
	}


	fn append_handler(&mut self, path: String, method: HandlerHttpMethod, handler: HandlerCallback) {
		match self.handlers.get_mut(&path) {
			Some(handlers) => { handlers.push((method, handler)); },
			None => { self.handlers.insert(path, vec![(method, handler)]); }
		};
	}

	fn handle_connection(&self, stream: TcpStream) {
		let mut connection = HttpConnection::new(stream);
		
		let mut connection_open = true;
		while connection_open {
			let request = HttpRequest::new(&mut connection).unwrap();

			// Before responding, check if the HTTP version of the request is supported (HTTP/1.1)
			if request.version != HttpVersion::new(VERSION).unwrap() {
				eprintln!("Expected HTTP version {}, found {}. Dropping connection...", VERSION, request.version);
				let mut response = HttpResponse::new(&mut connection);
				response.status(HttpStatus::new(400).unwrap());
				response.end();
				connection.terminate_connection();
				return;
			}

			// Then check if a `Host` was sent, else respond with a 400 status code
			if request.version != HttpVersion::new(VERSION).unwrap() {
				eprintln!("Expected 'Host' header, found nothing. Dropping connection...");
				let mut response = HttpResponse::new(&mut connection);
				response.status(HttpStatus::new(400).unwrap());
				response.end();
				connection.terminate_connection();
				return;
			}

			// Process headers and print them in while doing so
			for header in request.headers.iter() {
				match header.name.as_str() {
					"Connection" => {
						match header.value.as_str() {
							"close" => connection_open = false,
							_ => ()
						}
					},
					_ => ()
				}
			}

			// If everything is alright, check if an appropriate handler exists for this request
			if let Some(handlers) = self.handlers.get(&request.target.absolute_path) {
				for handler in handlers {
					match &handler.0 {
						HandlerHttpMethod::Specific(method) => {
							if request.method == *method {
								(handler.1)(request.clone(), HttpResponse::new(&mut connection))
							}
						},
						HandlerHttpMethod::Any => {
							(handler.1)(request.clone(), HttpResponse::new(&mut connection))
						}
					}
				}
			} else {
				// Otherwise, respond with a HTTP 404 Not Found status
				let mut response = HttpResponse::new(&mut connection);
				response.status(HttpStatus::new(404).unwrap());
				response.end();
				connection.terminate_connection();
				break;
			}
		}

		connection.terminate_connection()
	}
}

pub struct HttpConnection {
	pub peer_address: io::Result<SocketAddr>,

	stream: TcpStream
}

impl HttpConnection {
	pub fn new(stream: TcpStream) -> Self {
		// Obtain peer address (if possible) and log it to stdout
		let peer_address = stream.peer_addr();
		
		// Code below will probably be uncommented when logging is implemented
		/*let _readable_peer_address = match peer_address {
			Ok(sock_addr) => sock_addr.ip().to_string(),
			Err(_) => String::from("COULDN'T OBTAIN PEER ADDRESS")
		};*/

		Self {
			peer_address,
			stream
		}
	}

	/// Note: the `HttpConnection` struct shouldn't be used after this function returns
	pub fn terminate_connection(&self) {
		loop {
			match self.stream.shutdown(Shutdown::Both) {
				Ok(_) => break,
				Err(_) => ()
			}
		}
	}
}

#[derive(Clone)]
pub struct HttpRequest {
	pub method: HttpMethod,
	pub target: HttpTarget,
	pub version: HttpVersion,

	pub headers: Vec<HttpHeader>,
}

impl HttpRequest {
	pub fn new(parent: &mut HttpConnection) -> Option<Self> {
		// Begin by reading the first line
		let first_line = read_line(&mut parent.stream);
		// Then split it by whitespace
		let mut splitted_first_line = first_line.split_whitespace();

		// Check if the resulting slices aren't three in number (as they should be)
		if splitted_first_line.clone().count() != 3 {
			// If yes, print an error message to stderr and immediately terminate connection
			eprintln!("Invalid HTTP request detected. Dropping connection...");
			let mut response = HttpResponse::new(parent);
			response.status(HttpStatus::new(400).unwrap());
			response.end();
			parent.terminate_connection();
			return None;
		}

		// Else, start obtaining the HTTP method, target and version, terminating the connection in case of errors
		let Some(method) = HttpMethod::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP method detected. Dropping connection...");
			let mut response = HttpResponse::new(parent);
			response.status(HttpStatus::new(501).unwrap());
			response.end();
			parent.terminate_connection();
			return None;
		};
		let target = HttpTarget::new(splitted_first_line.next().unwrap());
		// Note: a HttpVersion structs will only check if the HTTP version is in the format "HTTP/{num}.{num}" and won't check if the major and minor revisions of the HTTP protocol exist. This check will occur later on our code
		let Some(http_version) = HttpVersion::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP version detected. Dropping connection...");
			let mut response = HttpResponse::new(parent);
			response.status(HttpStatus::new(400).unwrap());
			response.end();
			parent.terminate_connection();
			return None;
		};


		// Create a variable for storing HTTP headers
		let mut headers: Vec<HttpHeader> = Vec::new();

		// Obtain available HTTP headers
		loop {
			let line = read_line(&mut parent.stream);

			if line == String::from("") {
				break;
			}

			let Some(header) = HttpHeader::new(&line) else {
				eprintln!("Invalid HTTP header syntax detected. Dropping connection...");
				//terminate_connection(stream);
				return None;
			};

			headers.push(header);
		}

		Some(
			Self {
				method,
				target,
				version: http_version,
				headers,
			}
		)
	}
}

pub struct HttpResponse<'s> {
	parent: &'s mut HttpConnection,

	pub status: HttpStatus,
	pub version: HttpVersion,

	pub headers: Vec<HttpHeader>,
}

impl<'s> HttpResponse<'s> {
	pub fn new(parent: &'s mut HttpConnection) -> Self {
		Self {
			parent,
			status: HttpStatus::new(200).unwrap(),
			version: HttpVersion::new(VERSION).unwrap(),
			headers: Vec::new(),
		}
	}


	pub fn status(&mut self, status: HttpStatus) {
		self.status = status;
	}

	pub fn send<S>(self, message: S) where S: Into<String> {
		let message: String = message.into();

		// Send a HTTP status line response
		self.parent.stream.write(format!("{} {} \r\n", self.version, self.status).as_bytes()).unwrap();

		if !message.is_empty() {
			// Send a header indicating message length if message isn't empty
			self.parent.stream.write(format!("Content-Length: {}\r\n", message.len()).as_bytes()).unwrap();
		}

		// Loop through each header and write them to connection stream
		for header in &self.headers {
			self.parent.stream.write(format!("{}: {}\r\n", header.name, header.value).as_bytes()).unwrap();
		}

		// Send the response to the client (the CRLF before the response is to signal the beginning of message body)
		// If the message is empty, this will essentialy write "\r\n" to the stream, so it will be like there is no message body
		self.parent.stream.write(format!("\r\n{}", message).as_bytes()).unwrap();
	}

	pub fn end(self) {
		// Basically send an empty response
		self.send("");
	}
}