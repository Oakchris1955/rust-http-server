#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;
use std::{thread, time as std_time};

use time::macros::format_description;
use time::OffsetDateTime;

use crate::{Connection, Response, Status};

/// The type of the headers field in a [`Request`](crate::Request)
pub type Headers = HashMap<String, String>;

pub fn read_line(mut connection: &mut Connection) -> Option<String> {
    let mut temp_string = String::new();

    loop {
        // Attempt reading 1 byte from the TCPStream
        if let Some(bytes_read) = read_bytes(&mut connection, 1) {
            let char_read = bytes_read[0] as char;
            // If found CRLF (\r\n), return string (without CRLF)
            if char_read == '\n' {
                if temp_string.chars().last()? == '\r' {
                    temp_string.pop();
                    break;
                }
            }

            // Else, push char to output string
            temp_string.push(char_read);

            continue;
        }
    }

    Some(temp_string)
}

pub fn read_bytes(mut connection: &mut Connection, bytes_to_read: usize) -> Option<Vec<u8>> {
    let mut temp_vec = Vec::new();

    loop {
        // Allocate a u8 array with size of 1
        let mut temp_array: [u8; 1] = [0];

        // Attempt reading from the TCPStream
        if let Ok(bytes_read) = connection.stream.read(&mut temp_array) {
            if bytes_read > 0 {
                // If read more than 0 bytes, push the byte received to output vector
                temp_vec.push(temp_array[0]);

                // If read as many bytes as we want, break loop
                if temp_vec.len() == bytes_to_read {
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
        thread::sleep(std_time::Duration::from_millis(5));
    }

    Some(temp_vec)
}

pub fn parse_headers<S>(headers: S) -> Headers
where
    S: ToString,
{
    let headers: String = headers.to_string();
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
    S: ToString,
{
    let header: String = line.to_string();

    if let Some((name, mut value)) = header.split_once(":") {
        // Trim the value str from any whitespaces
        value = value.trim();
        headers.insert(name.to_lowercase(), value.to_string());

        Some(())
    } else {
        None
    }
}

/// Convert [`SystemTime`] to HTTP Date (RFC 9110, Section 5.6.7, IMF-fixdate definition)
pub fn format_time(time_to_format: SystemTime) -> String {
    OffsetDateTime::from(time_to_format)
        .format(format_description!(
            "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
        ))
        .unwrap()
}
