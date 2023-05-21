use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process::exit;

mod utils;
use utils::*;

mod enums;
pub use enums::*;

mod structs;
pub use structs::*;

const VERSION: &str = "HTTP/1.1";

pub struct HttpServer {
	pub hostname: String,
	pub port: u16
}

impl HttpServer {
	pub fn new<S, N>(hostname: S, port:N) -> Self where S: Into<String>, N: Into<u16> {
		Self {
			hostname: hostname.into(),
			port: port.into()
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

	fn handle_connection(&self, mut stream: TcpStream) {
		// Obtain peer address (if possible) and log it to stdout
		let peer_address = match stream.peer_addr() {
			Ok(sock_addr) => sock_addr.ip().to_string(),
			Err(_) => String::from("COULDN'T OBTAIN PEER ADDRESS")
		};
		println!("Received connection from {}", peer_address);


		// Begin by reading the first line
		let first_line = read_line(&mut stream);
		// Then split it by whitespace
		let mut splitted_first_line = first_line.split_whitespace();

		// Check if the resulting slices aren't three in number (as they should be)
		if splitted_first_line.clone().count() != 3 {
			// If yes, print an error message to stderr and immediately terminate connection
			eprintln!("Invalid HTTP request detected. Dropping connection...");
			terminate_connection(stream);
			return;
		}

		// Else, start obtaining the HTTP method, target and version, terminating the connection in case of errors
		let Some(method) = HttpMethod::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP method detected. Dropping connection...");
			terminate_connection(stream);
			return;
		};
		let target = splitted_first_line.next().unwrap();
		// Note: a HttpVersion structs will only check if the HTTP version is in the format "HTTP/{num}.{num}" and won't check if the major and minor revisions of the HTTP protocol exist. This check will occur later on our code
		let Some(http_version) = HttpVersion::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP version detected. Dropping connection...");
			terminate_connection(stream);
			return;
		};

		// Print a message to stdout about the HTTP request (meant for debugging, will be removed in the near future)
		println!("Method: {}\nPath: {}\nHTTP version: {}", method, target, http_version);


		// Create a variable for storing HTTP headers
		let mut headers: Vec<HttpHeader> = Vec::new();

		// Obtain available HTTP headers
		loop {
			let line = read_line(&mut stream);

			if line == String::from("") {
				break;
			}

			let Some(header) = HttpHeader::new(&line) else {
				eprintln!("Invalid HTTP header syntax detected. Dropping connection...");
				terminate_connection(stream);
				return;
			};

			headers.push(header);
		}

		// Before responding, check if the HTTP version of the request is supported (HTTP/1.1)
		if http_version != HttpVersion::new(VERSION).unwrap() {
			eprintln!("Expected HTTP version {}, found {}. Dropping connection...", VERSION, http_version);
			terminate_connection(stream);
			return;
		}

		// Send a HTTP 200 OK response
		stream.write(format!("{} 200 \r\n", http_version).as_bytes()).unwrap();

		// Send a response to the client (the CRLF before the response is to signal the beginning of message body)
		stream.write(format!("\r\n{}", target).as_bytes()).unwrap();

		// Once done, terminate the connection
		terminate_connection(stream);
	}
}
