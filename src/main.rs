use std::net::{TcpListener, TcpStream};
use std::process::exit;

mod utils;
use utils::*;

mod enums;
use enums::*;

mod structs;
use structs::*;


/// A function to handle incoming client connections
fn handle_client(mut stream: TcpStream) {
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

	// Once done, terminate the connection
	terminate_connection(stream);
}

fn main() {
	// Initiate a TCP Listener at localhost port 2300 (port and IP address are subject to change)
    let listener = TcpListener::bind("localhost:2300").unwrap_or_else(|err| {
		eprintln!("Couldn't initiate TCP server. Error message: {}", err);
		exit(1);
	});
	println!("Successfully initiated server");

	// For each incoming connection request, accept connection and pass control of connection to "handle_client" function
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
				eprintln!("Failed to establish a new connection. Error message: {}", e);
			}
        }
    }
}