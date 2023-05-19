use std::net::{TcpListener, TcpStream, Shutdown};
use std::process::exit;

mod utils;
use utils::*;

fn handle_client(mut stream: TcpStream) {
	let peer_address = match stream.peer_addr() {
		Ok(sock_addr) => sock_addr.ip().to_string(),
		Err(_) => String::from("COULDN'T OBTAIN PEER ADDRESS")
	};
	println!("Received connection from {}", peer_address);


	let first_line = read_line(&mut stream);
	let mut splitted_first_line = first_line.split(' ');

	if splitted_first_line.clone().count() == 3 {
		println!("Method: {}\nPath: {}\nHTTP version: {}", splitted_first_line.next().unwrap(), splitted_first_line.next().unwrap(), splitted_first_line.next().unwrap());
	} else {
		eprintln!("Invalid HTTP request detected. Dropping connection...");
	}

	loop {
		match stream.shutdown(Shutdown::Both) {
			Ok(_) => break,
			Err(_) => ()
		}
	}
}

fn main() {
    let listener = TcpListener::bind("localhost:2300").unwrap_or_else(|err| {
		eprintln!("Couldn't initiate TCP server. Error message: {}", err);
		exit(1);
	});
	println!("Successfully initiated server");

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