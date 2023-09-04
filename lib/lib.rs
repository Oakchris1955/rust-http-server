#![warn(missing_docs)]

//! **Note:** The library is still in early Alpha/Beta
//!
//! A lightweight server library for the HTTP/1.1 protocol
//!
//! The aim of this crate is to create a library both easy to use and fast in intercepting incoming connections
//!
//! # Quick start
//!
//! ```
//! # #[cfg(feature = "safe")]
//! # extern crate oak_http_server;
//! // Library imports
//! use oak_http_server::{Server, Status};
//!
//! fn main() {
//!     // Save server hostname and port as variables
//!     let hostname = "localhost";
//!     let port: u16 = 2300;
//!     
//!     // Create a new HTTP server instance (must be mutable since appending handlers to the Server struct modifies its fields)
//!     let mut server = Server::new(hostname, port);
//!
//!     // The following path handler responds to each response to the "/ping" path with "Pong!"
//!     server.on("/ping", |_request, response| response.end_with("Pong!"));
//!     
//!     // The following path handler responds only to GET requests on the "\headers" path
//!     // and returns a list of the headers supplied in the corresponding HTTP request
//!     server.on_get("/headers", |request, response| {
//!         response.end_with(format!(
//!	            "Your browser sent the following headers with the request:\n{}",
//!	            request
//!	                .headers
//!                 .iter()
//!	                .map(|(name, value)| format!("{}: {}\n", name, value))
//!	                .collect::<String>(),
//!         ))
//!     });
//!
//!    // Start the HTTP server. The provided closure/callback function will be called
//!    // when a connection listener has been successfully established.
//!    // Once this function is run, the server will begin listening to incoming HTTP requests
//!    # #[cfg(not)]
//!    server.start(|| {
//!        println!("Successfully initiated server");
//!    });
//! }
//! ```

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::process::exit;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

mod utils;
use utils::*;
pub use utils::{Headers, FORBIDDEN_HEADERS};

mod enums;
pub use enums::*;

mod structs;
pub use structs::*;

pub mod handlers;

const VERSION: Version = Version { major: 1, minor: 1 };

/// A custom HTTP method struct that extends [`Method`].
///
/// It includes an `Any` field to allow the server to process a [`Request`] of any [`Method`]
///
/// There is also a `Directory` field so that the user can create custom URL parsers for a directory or use the ones provided by the library.
pub enum HandlerMethod {
    /// Represents a directory handler. Will be run whether the user requests a target that is part of this directory. Also, it is the last handler type in terms of priority
    Directory,
    /// A handler that will be run only when a specific [`Method`] is made at the corresponding target
    Specific(Method),
    /// Like the [`Specific`](HandlerMethod::Specific) variant, but will run for any type of request
    Any,
}

/// The type of the callback function of a [`Handler`]
pub type HandlerCallback = dyn Fn(Request, Response) + Send + Sync;

/// The type of a request handler
pub type Handler = (HandlerMethod, Arc<HandlerCallback>);

/// Represents a collection of HTTP cookies
pub type Cookies = HashMap<String, String>;

/// The "heart" of the module; the server struct
///
/// It does everything: process requests, pass them to handlers, reject them if they are malformed
pub struct Server {
    /// The hostname the server is listening to for requests
    pub hostname: String,
    /// The port the server is listening for requests
    pub port: u16,

    handlers: HashMap<String, Vec<Handler>>,
}

impl Server {
    /// Initialize a [`Server`] by passing a hostname and a port number
    pub fn new<S, N>(hostname: S, port: N) -> Self
    where
        S: ToString,
        N: Into<u16>,
    {
        Self {
            hostname: hostname.to_string(),
            port: port.into(),

            handlers: HashMap::new(),
        }
    }

    /// Start the server and make it process incoming connections
    pub fn start(self, callback: fn()) {
        // Initiate a TCP Listener at localhost port 2300 (port and IP address are subject to change)
        let listener = TcpListener::bind(format!("{}:{}", self.hostname, self.port))
            .unwrap_or_else(|err| {
                eprintln!("Couldn't initiate TCP server. Error message: {}", err);
                exit(1);
            });

        // Arc is basically a pointer that can be shared safely between different threads through cloning
        let shared_self = Arc::new(self);

        callback();

        // For each incoming connection request, accept connection and pass control of connection to "handle_connection" function
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Clone the Arc and move it to the new thread
                    let self_clone = shared_self.clone();
                    thread::spawn(move || self_clone.handle_connection(stream));
                }
                Err(e) => {
                    eprintln!("Failed to establish a new connection. Error message: {}", e);
                }
            }
        }
    }

    /// Append a function handler that will be called on any request in a specific path
    pub fn on<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(path.to_string(), HandlerMethod::Any, handler);
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only GET requests
    pub fn on_get<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(
            path.to_string(),
            HandlerMethod::Specific(Method::GET),
            handler,
        );
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only HEAD requests
    pub fn on_head<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(
            path.to_string(),
            HandlerMethod::Specific(Method::HEAD),
            handler,
        );
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only POST requests
    pub fn on_post<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(
            path.to_string(),
            HandlerMethod::Specific(Method::POST),
            handler,
        );
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only PUT requests
    pub fn on_put<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(
            path.to_string(),
            HandlerMethod::Specific(Method::PUT),
            handler,
        );
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only DELETE requests
    pub fn on_delete<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(
            path.to_string(),
            HandlerMethod::Specific(Method::DELETE),
            handler,
        );
    }

    /// Append a directory handler that will be called on any request in a specific path
    pub fn on_directory<S, H>(&mut self, path: S, handler: H)
    where
        S: ToString,
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.append_handler(path.to_string(), HandlerMethod::Directory, handler);
    }

    fn append_handler<H>(&mut self, path: String, method: HandlerMethod, handler: H)
    where
        H: Fn(Request, Response) + Send + Sync + 'static,
    {
        match self.handlers.get_mut(&path) {
            Some(handlers) => {
                handlers.push((method, Arc::new(handler)));
            }
            None => {
                self.handlers
                    .insert(path, vec![(method, Arc::new(handler))]);
            }
        };
    }

    fn handle_connection(&self, stream: TcpStream) {
        let mut connection = Connection::new(stream);

        'connection_loop: while !connection.close {
            let mut request = match Request::new(&mut connection) {
                Some(value) => value,
                None => {
                    eprintln!("Couldn't create new request for connection. Dropping connection...");
                    break 'connection_loop;
                }
            };

            // Update connection fields
            connection.inactive_since = SystemTime::now();
            connection.requests_received += 1;

            // Check whether the max amount of requests this connection can process has been reached
            if connection.requests_received > connection.max_requests {
                // We just straight up close the connection. Chek this: https://stackoverflow.com/a/46365730/
                break 'connection_loop;
            }

            // Before responding, check if the HTTP version of the request is supported (HTTP/1.1)
            'version_check: {
                // If the major revision is different, send 505 HTTP Version Not Supported
                if request.version.major != VERSION.major {
                    Response::quick(&mut connection, Status::HttpVersionNotSupported);
                // If not, check the minor revision
                } else {
                    // If it is greater than the supported one, send 400 Bad Request
                    if request.version.minor > VERSION.minor {
                        Response::quick(&mut connection, Status::BadRequest);
                    // If it is less, send 426 Upgrade Required
                    } else if request.version.minor < VERSION.minor {
                        Response::quick(&mut connection, Status::UpgradeRequired);
                    // Otherwise, break from this code block
                    } else {
                        break 'version_check;
                    }
                }

                // Lastly, if the code is still running, print an error message and break the connection loop
                eprintln!(
                    "Expected HTTP version {}, found {}. Dropping connection...",
                    VERSION, request.version
                );
                break 'connection_loop;
            }

            // Then check if a `Host` was sent, else respond with a 400 status code
            if !request.headers.contains_key("host") {
                eprintln!("Expected 'Host' header, found nothing. Dropping connection...");
                Response::quick(&mut connection, Status::BadRequest);
                break 'connection_loop;
            }

            // The handle_header function returns a Status if an Error occurs, which is then sent to client
            if let Err(status) = self.handle_headers(&mut connection, &request.headers) {
                Response::quick(&mut connection, status);
                break 'connection_loop;
            };

            // If everything is alright, check if an appropriate handler exists for this request
            if let Some(handlers) = self.handlers.get(&request.target.full_url()) {
                for handler in handlers {
                    match &handler.0 {
                        HandlerMethod::Specific(method) => {
                            if request.method == *method {
                                (handler.1)(request, Response::new(&mut connection))
                            }
                            continue 'connection_loop;
                        }
                        HandlerMethod::Any => {
                            (handler.1)(request, Response::new(&mut connection));
                            continue 'connection_loop;
                        }
                        _ => (),
                    }
                }
            } else {
                let full_url = request.target.full_url();
                let mut path_sections = full_url.split("/");
                path_sections.next();

                let mut path_string = String::new();

                for section in path_sections {
                    path_string.push_str(&format!("/{}", section));

                    if let Some(handlers) = self.handlers.get(&path_string) {
                        if let Some(handler) = handlers
                            .iter()
                            .find(|handler| matches!(handler.0, HandlerMethod::Directory))
                        {
                            (request.target.target_path, request.target.relative_path) = (
                                path_string.clone(),
                                request
                                    .target
                                    .relative_path
                                    .split_at(path_string.len())
                                    .1
                                    .to_string(),
                            );

                            (handler.1)(request, Response::new(&mut connection));
                            continue 'connection_loop;
                        }
                    }
                }
            }

            // Otherwise, respond with a HTTP 404 Not Found status
            Response::quick(&mut connection, Status::NotFound);
            break 'connection_loop;
        }

        connection.terminate_connection()
    }

    fn handle_headers(&self, connection: &mut Connection, headers: &Headers) -> Result<(), Status> {
        // Process headers (.to_lowercase() because headers are case-insensitive)
        for (name, value) in headers
            .iter()
            .map(|(name, value)| (name.to_lowercase(), value.to_lowercase()))
        {
            let value = value.as_str();
            match name.as_str() {
                "connection" => match value {
                    "close" => connection.close = true,
                    // According to Mozilla Web Docs (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection),
                    // any value other than "close" is treated as "keep-alive" in HTTP/1.1 connections
                    _ => (),
                },
                "keep-alive" => {
                    // Parse the header value
                    for parameter in value.split(",") {
                        if let Some((param_name, param_value)) = parameter.split_once("=") {
                            if let Ok(param_int_value) = param_value.parse::<usize>() {
                                // Match the header parameters
                                // Note: the server will process them only if they are in logical limits
                                // For example, the server won't allow a timeout of more than the default one (which is 60 seconds)
                                match param_name {
                                    "timeout" => {
                                        if param_int_value <= connection.timeout.as_secs() as usize
                                        {
                                            connection.timeout =
                                                Duration::from_secs(param_int_value as u64)
                                        }
                                    }
                                    "max" => {
                                        if param_int_value <= 20 {
                                            connection.max_requests = param_int_value
                                        }
                                    }
                                    _ => (),
                                }

                                continue;
                            }
                        }

                        // Otherwise, if an error occured while parsing, send a HTTP 400 Bad Request response code
                        return Err(Status::BadRequest);
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}

/// A struct representing a HTTP connection between a client and the server
pub struct Connection {
    /// The address of the peer client (if known)
    pub peer_address: io::Result<SocketAddr>,

    /// Whether to close the connection or not
    close: bool,
    /// Connection timeout (in seconds)
    timeout: Duration,
    /// Max number of requests that can be received before closing the connection
    max_requests: usize,

    /// How long this connection hasn't received a HTTP request
    inactive_since: SystemTime,
    /// The number of requests received in this connection
    requests_received: usize,

    /// The [`TcpStream`] from which to read and write data
    stream: TcpStream,
}

impl Connection {
    /// Create a new [`Connection`] from a [`TcpStream`]
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

            close: false,
            timeout: Duration::from_secs(60),
            max_requests: 5,

            inactive_since: SystemTime::now(),
            requests_received: 0,

            stream,
        }
    }

    /// Terminates the connection between the client and the server
    ///
    /// Note: the [`Connection`] struct shouldn't be used after this function returns
    pub fn terminate_connection(&self) {
        loop {
            match self.stream.shutdown(Shutdown::Both) {
                Ok(_) => break,
                Err(_) => (),
            }
        }
    }
}

/// A HTTP request
#[derive(Clone)]
pub struct Request {
    /// The request's method
    pub method: Method,
    /// The target URL of the method
    pub target: Target,
    /// The HTTP version the client supports
    pub version: Version,

    /// The body of the request (if any)
    pub body: Vec<u8>,

    /// A type alias of a Hashmap containing a list of the headers of the [`Request`]
    /// Please note that in order to comply with RFC 9110, Section 5.1-3 ("Field names are case-insensitive..."),
    /// the header names are all in lowercase. Their value, however, is left intact
    pub headers: Headers,

    /// A list of the [`Cookies`] the client sent alongside this [`Request`]
    pub cookies: Cookies,
}

impl Request {
    /// Create a new [`Request`] from a [`Connection`]
    pub fn new(mut parent: &mut Connection) -> Option<Self> {
        // Begin by reading the first line
        let first_line = read_line(&mut parent)?;
        // Then split it by whitespace
        let splitted_first_line = first_line.split_whitespace().collect::<Vec<_>>();

        let (method, target, version) = match &splitted_first_line[..] {
            // Check if the resulting slices aren't three in number (as they should be)
            &[method, target, version] => {
                // If not, try parsing the HTTP method, target and version, and terminate the connection if any error occur
                let Some(method) = Method::new(method) else {
                    eprintln!("Invalid HTTP method detected. Dropping connection...");
                    Response::quick(parent, Status::NotImplemented);
                    return None;
                };
                let target = Target::new(target);
                // Note: a HTTP version struct will only check if the HTTP version is in the format "HTTP/{num}.{num}" and won't check if the major and minor revisions of the HTTP protocol exist. This check will occur later on our code                let Some(version) = Version::new(version) else {
                let Some(version) = Version::new(version)else{
                    eprintln!("Invalid HTTP version detected. Dropping connection...");
                    Response::quick(parent, Status::BadRequest);
                    return None;
                };

                (method, target, version)
            }
            _ => {
                // If yes, print an error message to stderr and immediately terminate connection
                eprintln!("Invalid HTTP request detected. Dropping connection...");
                Response::quick(parent, Status::BadRequest);
                return None;
            }
        };

        // Create a variable for storing HTTP headers
        let mut headers: Headers = Headers::new();

        // Obtain available HTTP headers
        loop {
            let line = read_line(&mut parent)?;

            if line == String::new() {
                break;
            }

            if parse_header_line(&mut headers, line).is_none() {
                eprintln!("Invalid HTTP header syntax detected. Dropping connection...");
                return None;
            };
        }

        // Parse cookies from Cookie header
        let mut cookies: Cookies = HashMap::new();

        if let Some(cookies_header) = headers.get("cookie") {
            for cookie in cookies_header.split("; ") {
                if let Some((name, value)) = cookie.split_once("=") {
                    cookies.insert(name.to_string(), value.to_string());
                }
            }
        }

        // Allocate an empty vector for the request body
        let mut body = Vec::new();

        // Check if the HTTP request has some body
        // (for example, when the Content-Type header is set to multipart/form-data or application/x-www-form-urlencoded)
        if let Some(transfer_encoding) = headers.get("transfer-encoding") {
            match transfer_encoding.as_str() {
                "chunked" => {
                    loop {
                        let length_line = read_line(&mut parent)?;
                        let (chunk_length, _) = length_line.split_once(";")?;
                        let chunk_length = usize::from_str_radix(chunk_length, 16).ok()?;

                        if chunk_length != 0 {
                            let chunk_body = read_bytes(&mut parent, chunk_length + 2)?;
                            body.extend_from_slice(&chunk_body[..&chunk_body.len() - 2]);
                        } else {
                            // Remove CRLF from stream
                            read_bytes(&mut parent, 2);
                            break;
                        }
                    }

                    // Ignore the trailers
                    while read_line(&mut parent)?.len() != 0 {}
                }
                _ => {
                    Response::quick(parent, Status::BadRequest);
                    return None;
                }
            }
        } else if let Some(content_length) = headers.get("content-length") {
            if let Ok(content_length) = content_length.parse::<usize>() {
                if let Some(request_body) = utils::read_bytes(&mut parent, content_length) {
                    body = request_body
                } else {
                    Response::quick(parent, Status::InternalServerError);
                    return None;
                }
            } else {
                Response::quick(parent, Status::BadRequest);
                return None;
            }
        }

        Some(Self {
            method,
            target,
            version,
            body,
            headers,
            cookies,
        })
    }
}

/// A HTTP response for the server to reply to the client
pub struct Response<'s> {
    parent: &'s mut Connection,

    /// The HTTP status code of the response
    pub status: Status,
    /// The HTTP version of the response
    pub version: Version,
    // Whether we have already sent the status line or not
    sent_status: bool,

    /// A type alias of a Hashmap containing the headers of the response
    headers: Headers,
    // Whether we have already sent the headers or not
    sent_headers: bool,

    /// A list of the Cookies to send
    cookies: HashSet<Cookie>,
}

impl<'s> Response<'s> {
    /// Create a new [`Response`]
    pub fn new(parent: &'s mut Connection) -> Self {
        Self {
            parent,
            status: Status::OK,
            version: VERSION,
            sent_status: false,
            headers: [
                // Set some default headers
                (String::from("Transfer-Encoding"), String::from("chunked")),
                (String::from("Date"), format_time(SystemTime::now())),
            ]
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .into(),
            sent_headers: false,
            cookies: HashSet::new(),
        }
    }

    /// Send an empty response with a specified [`Status`]
    fn quick(connection: &'s mut Connection, status: Status) {
        let mut response = Self::new(connection);
        response.status(status);
        response.end()
    }

    /// Change the [`Status`] of the response
    pub fn status(&mut self, status: Status) {
        if !self.sent_status {
            self.status = status;
        }
    }

    /// Set a new cookie
    pub fn set_cookie(&mut self, cookie: Cookie) {
        // .replace() inserts the provided cookie to the cookies HashSet
        // and removes any cookie with the same name (in order to prevent cookies with the same name being sent)
        self.cookies.replace(cookie);
    }

    /// Set a new header
    pub fn set_header<S>(&mut self, name: S, value: S)
    where
        S: ToString,
    {
        let name = name.to_string();

        // Don't push the header to the internal header field if it is a forbidden one
        if !FORBIDDEN_HEADERS
            .iter()
            .map(|item| item.to_lowercase())
            .any(|item| item == name.to_lowercase())
        {
            self.headers.insert(name, value.to_string());
        }
    }

    /// Set multiple new headers
    pub fn set_headers(&mut self, other_headers: Headers) {
        other_headers
            .iter()
            .for_each(|(name, value)| self.set_header(name, value))
    }

    /// Get a reference to the internal [`Headers`]
    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }

    // Send a HTTP status line response
    fn send_status(&mut self) {
        if !self.sent_status {
            self.parent
                .stream
                .write(format!("{} {}\r\n", self.version, self.status).as_bytes())
                .unwrap();
            self.sent_status = true;
        }
    }

    // Send headers
    fn send_headers(&mut self) {
        if !self.sent_headers {
            // Invoke send_status function
            self.send_status();

            // Loop through each header and write them to connection stream
            for (name, value) in &self.headers {
                self.parent
                    .stream
                    .write(format!("{}: {}\r\n", name, value).as_bytes())
                    .unwrap();
            }

            // Send the cookies
            self.cookies.iter().for_each(|cookie| {
                self.parent
                    .stream
                    .write(format!("Set-Cookie: {}\n", cookie).as_bytes())
                    .unwrap();
            });

            // Send CRLF indicating that no more headers will be received
            self.parent.stream.write(b"\r\n").unwrap();
            self.sent_headers = true;
        }
    }

    fn send_chunk(&mut self, chunk_data: Vec<u8>) {
        // Check if there are any data to actually send
        // According to RFC 2616, Section 3.6.1, second paragraph, a chunk can't have a length of 0, unless it is the last chunk
        if !chunk_data.is_empty() {
            // Invoke send_headers function
            self.send_headers();

            // Send chunk size
            self.parent
                .stream
                .write(format!("{:x}\r\n", chunk_data.len()).as_bytes())
                .unwrap();

            // Send chunk data
            self.parent.stream.write(&chunk_data).unwrap();
            self.parent.stream.write(b"\r\n").unwrap();
        }
    }

    fn end_chunked(&mut self) {
        // Invoke send_headers function
        self.send_headers();

        // Send last-chunk, followed by CRLF
        self.parent.stream.write(b"0\r\n\r\n").unwrap();
    }

    /// Send some data to the connection
    pub fn send<S>(&mut self, message: S)
    where
        S: ToString,
    {
        // Turn String to u8 vector
        let message: Vec<u8> = message.to_string().as_bytes().to_vec();

        // Send message
        self.send_chunk(message);
    }

    /// End the response (consumes it)
    pub fn end(mut self) {
        // An alias to self.end_chunked()
        self.end_chunked();
    }

    /// End the response with some data (calls [`Response.send`](#method.send), then [`Response.end`](#method.end))
    pub fn end_with<S>(mut self, message: S)
    where
        S: ToString,
    {
        self.send(message);
        self.end();
    }
}
