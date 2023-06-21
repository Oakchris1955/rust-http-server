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
//!     server.on("/ping", |_request, response| response.send("Pong!"));
//!     
//!     // The following path handler responds only to GET requests on the "\headers" path
//!     // and returns a list of the headers supplied in the corresponding HTTP request
//!     server.on_get("/headers", |request, response| {
//!         response.send(format!(
//!	            "Your browser sent the following headers with the request:\n{}",
//!	            request
//!	                .headers
//!                 .iter()
//!	                .map(|header| header.to_string() + "\n")
//!	                .collect::<String>(),
//!         ))
//!     });
//!
//!    // Start the HTTP server. The provided closure/callback function will be called
//!    // when a connection listener has been successfully established.
//!    // Once this function is run, the server will begin listening to incoming HTTP requests
//!    # #[cfg(not(test))]
//!    server.start(|| {
//!        println!("Successfully initiated server");
//!    });
//! }
//! ```

use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::process::exit;

mod utils;
use utils::*;

mod enums;
pub use enums::*;

mod structs;
pub use structs::*;

//pub mod handlers;

const VERSION: &str = "HTTP/1.1";

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
    /// Like the [`Specific`] variant, but will run for any type of request
    Any,
}

/// The type of the callback function of a [`Handler`]
pub type HandlerCallback = fn(Request, Response);

/// The type of a request handler
pub type Handler = (HandlerMethod, HandlerCallback);

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
        S: Into<String>,
        N: Into<u16>,
    {
        Self {
            hostname: hostname.into(),
            port: port.into(),

            handlers: HashMap::new(),
        }
    }

    /// Start the server and make it process incoming connections
    pub fn start(&self, callback: fn()) {
        // Initiate a TCP Listener at localhost port 2300 (port and IP address are subject to change)
        let listener = TcpListener::bind(format!("{}:{}", self.hostname, self.port))
            .unwrap_or_else(|err| {
                eprintln!("Couldn't initiate TCP server. Error message: {}", err);
                exit(1);
            });

        callback();

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

    /// Append a function handler that will be called on any request in a specific path
    pub fn on<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(path.into(), HandlerMethod::Any, handler);
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only GET requests
    pub fn on_get<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(path.into(), HandlerMethod::Specific(Method::GET), handler);
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only HEAD requests
    pub fn on_head<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(path.into(), HandlerMethod::Specific(Method::HEAD), handler);
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only POST requests
    pub fn on_post<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(path.into(), HandlerMethod::Specific(Method::POST), handler);
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only PUT requests
    pub fn on_put<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(path.into(), HandlerMethod::Specific(Method::PUT), handler);
    }

    /// Same as the [`on()`](`Server::on()`) function, but processes only DELETE requests
    pub fn on_delete<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(
            path.into(),
            HandlerMethod::Specific(Method::DELETE),
            handler,
        );
    }

    /// Append a directory handler that will be called on any request in a specific path
    pub fn on_directory<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(path.into(), HandlerMethod::Directory, handler);
    }

    fn append_handler(&mut self, path: String, method: HandlerMethod, handler: HandlerCallback) {
        match self.handlers.get_mut(&path) {
            Some(handlers) => {
                handlers.push((method, handler));
            }
            None => {
                self.handlers.insert(path, vec![(method, handler)]);
            }
        };
    }

    fn handle_connection(&self, stream: TcpStream) {
        let mut connection = Connection::new(stream);

        let mut connection_open = true;

        'connection_loop: while connection_open {
            let mut request = match Request::new(&mut connection) {
                Some(value) => value,
                None => {
                    eprintln!("Couldn't create new request for connection. Dropping connection...");
                    break 'connection_loop;
                }
            };

            // Create a HTTP response beforehand that will be used in case an error occurs
            let mut err_response = Response::new(&mut connection);

            // Before responding, check if the HTTP version of the request is supported (HTTP/1.1)
            if request.version != Version::new(VERSION).unwrap() {
                eprintln!(
                    "Expected HTTP version {}, found {}. Dropping connection...",
                    VERSION, request.version
                );
                err_response.status(Status::new(400).unwrap());
                err_response.end();
                break 'connection_loop;
            }

            // Then check if a `Host` was sent, else respond with a 400 status code
            if request.version != Version::new(VERSION).unwrap() {
                eprintln!("Expected 'Host' header, found nothing. Dropping connection...");
                err_response.status(Status::new(400).unwrap());
                err_response.end();
                break 'connection_loop;
            }

            // Process headers and print them in while doing so
            for header in request.headers.iter() {
                match header.name.as_str() {
                    "Connection" => match header.value.as_str() {
                        "close" => connection_open = false,
                        _ => (),
                    },
                    _ => (),
                }
            }

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
            err_response.status(Status::new(404).unwrap());
            err_response.end();
            break 'connection_loop;
        }

        connection.terminate_connection()
    }
}

/// A struct representing a HTTP connection between a client and the server
pub struct Connection {
    /// The address of the peer client (if known)
    pub peer_address: io::Result<SocketAddr>,

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

    /// A Vec containing a list of the headers of the [`Request`]
    pub headers: Vec<Header>,
}

impl Request {
    /// Create a new [`Request`] from a [`Connection`]
    pub fn new(parent: &mut Connection) -> Option<Self> {
        // Begin by reading the first line
        let first_line = read_line(&mut parent.stream);
        // Then split it by whitespace
        let mut splitted_first_line = first_line.split_whitespace();

        // Create a HTTP response beforehand that will be used in case an error occurs
        let mut err_response = Response::new(parent);

        // Check if the resulting slices aren't three in number (as they should be)
        if splitted_first_line.clone().count() != 3 {
            // If yes, print an error message to stderr and immediately terminate connection
            eprintln!("Invalid HTTP request detected. Dropping connection...");
            err_response.status(Status::new(400).unwrap());
            err_response.end();
            return None;
        }

        // Else, start obtaining the HTTP method, target and version, terminating the connection in case of errors
        let Some(method) = Method::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP method detected. Dropping connection...");
			err_response.status(Status::new(501).unwrap());
			err_response.end();
			return None;
		};
        let target = Target::new(splitted_first_line.next().unwrap());
        // Note: a HTTP version struct will only check if the HTTP version is in the format "HTTP/{num}.{num}" and won't check if the major and minor revisions of the HTTP protocol exist. This check will occur later on our code
        let Some(http_version) = Version::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP version detected. Dropping connection...");
			err_response.status(Status::new(400).unwrap());
			err_response.end();
			return None;
		};

        // Create a variable for storing HTTP headers
        let mut headers: Vec<Header> = Vec::new();

        // Obtain available HTTP headers
        loop {
            let line = read_line(&mut parent.stream);

            if line == String::from("") {
                break;
            }

            let Some(header) = Header::new(&line) else {
				eprintln!("Invalid HTTP header syntax detected. Dropping connection...");
				return None;
			};

            headers.push(header);
        }

        Some(Self {
            method,
            target,
            version: http_version,
            headers,
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

    /// A Vec containing the headers of the response
    pub headers: Vec<Header>,
}

impl<'s> Response<'s> {
    /// Create a new [`Response`]
    pub fn new(parent: &'s mut Connection) -> Self {
        Self {
            parent,
            status: Status::new(200).unwrap(),
            version: Version::new(VERSION).unwrap(),
            headers: Vec::new(),
        }
    }

    /// CHange the [`Status`] of the response
    pub fn status(&mut self, status: Status) {
        self.status = status;
    }

    /// Send the response along with a message (consumes the response)
    pub fn send<S>(self, message: S)
    where
        S: Into<String>,
    {
        let message: String = message.into();

        // Send a HTTP status line response
        self.parent
            .stream
            .write(format!("{} {} \r\n", self.version, self.status).as_bytes())
            .unwrap();

        if !message.is_empty() {
            // Send a header indicating message length if message isn't empty
            self.parent
                .stream
                .write(format!("Content-Length: {}\r\n", message.len()).as_bytes())
                .unwrap();
        }

        // Loop through each header and write them to connection stream
        for header in &self.headers {
            self.parent
                .stream
                .write(format!("{}: {}\r\n", header.name, header.value).as_bytes())
                .unwrap();
        }

        // Send the response to the client (the CRLF before the response is to signal the beginning of message body)
        // If the message is empty, this will essentialy write "\r\n" to the stream, so it will be like there is no message body
        self.parent
            .stream
            .write(format!("\r\n{}", message).as_bytes())
            .unwrap();
    }

    /// Send an empty response (consumes it)
    pub fn end(self) {
        // Basically send an empty response
        self.send("");
    }
}
