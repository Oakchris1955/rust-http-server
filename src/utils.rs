use std::io::Read;
use std::net::{TcpStream, Shutdown};

pub fn read_line(stream: &mut TcpStream) -> String {
	let mut temp_string = String::new();

	loop {
		let mut temp_array: [u8; 1] = [0];

		if stream.read(&mut temp_array).is_ok() {
			let temp_char = char::from_u32(temp_array[0] as u32).unwrap();
			
			if temp_char == '\n' {
				if temp_string.chars().last().unwrap() == '\r' {
					temp_string.pop();
					break;
				}
			}

			temp_string.push(temp_char);
		}
	}

	temp_string
}

pub fn terminate_connection(stream: TcpStream) {
	loop {
		match stream.shutdown(Shutdown::Both) {
			Ok(_) => break,
			Err(_) => ()
		}
	}
}