#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;

pub type Headers = HashMap<String, String>;

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

pub fn parse_headers<S>(headers: S) -> Headers
where
    S: Into<String>,
{
    let headers: String = headers.into();
    let mut temp_hashmap: Headers = HashMap::new();

    for header in headers.split("\r\n") {
        if let Some((name, mut value)) = header.split_once(":") {
            // Trim the value str from any whitespaces
            value = value.trim();
            temp_hashmap.insert(name.to_string(), value.to_string());
        }
    }

    temp_hashmap
}

pub fn parse_header_line<S>(headers: &mut Headers, line: S) -> Option<()>
where
    S: Into<String>,
{
    let header: String = line.into();

    if let Some((name, mut value)) = header.split_once(":") {
        // Trim the value str from any whitespaces
        value = value.trim();
        headers.insert(name.to_string(), value.to_string());

        Some(())
    } else {
        None
    }
}
