#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Read;
use std::{thread, time};

use crate::{Connection, Response, Status};

pub type Headers = HashMap<String, String>;

pub fn read_line(mut connection: &mut Connection) -> Option<String> {
    let mut temp_string = String::new();

    loop {
        // Attempt reading 1 byte from the TCPStream
        if let Some(string_read) = read_bytes(&mut connection, 1) {
            // If found CRLF (\r\n), return string (without CRLF)
            if string_read == "\n" {
                if temp_string.chars().last()? == '\r' {
                    temp_string.pop();
                    break;
                }
            }

            // Else, push single-character-string to output string
            temp_string.push_str(&string_read);

            continue;
        }
    }

    Some(temp_string)
}

pub fn read_bytes(mut connection: &mut Connection, bytes_to_read: usize) -> Option<String> {
    let mut temp_string = String::new();

    loop {
        // Allocate a u8 array with size of 1
        let mut temp_array: [u8; 1] = [0];

        // Attempt reading from the TCPStream
        if let Ok(bytes_read) = connection.stream.read(&mut temp_array) {
            if bytes_read > 0 {
                // If read more than 0 bytes, turn the u8 received into a char
                let temp_char = char::from_u32(temp_array[0] as u32)?;

                // Push char to output string
                temp_string.push(temp_char);

                // If read as many bytes as we want, break loop
                if temp_string.len() == bytes_to_read {
                    break;
                }

                continue;
            }
        }

        // If an error occured, wait for some time before re-reading from stream
        // Also, check whether the request timeout has been reached
        if connection.inactive_since.elapsed().ok()? > connection.timeout {
            Response::quick(&mut connection, Status::RequestTimeout);
            return None;
        }

        // Note: This sleep call saves a ton of resources
        thread::sleep(time::Duration::from_millis(5));
    }

    Some(temp_string)
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
