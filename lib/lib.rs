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

const VERSION: &str = "HTTP/1.1";

/// A custom HTTP method struct that extends [`HttpMethod`].
///
/// It include an `Any` field to allow the server to process a [`HttpRequest`] of any [`HttpMethod`]
pub enum HandlerHttpMethod {
    Specific(HttpMethod),
    Any,
}

/// The type of the callback function of a [`Handler`]
pub type HandlerCallback = fn(HttpRequest, HttpResponse);

/// The type of a request handler
pub type Handler = (HandlerHttpMethod, HandlerCallback);

/// The "heart" of the module; the server struct
///
/// It does everything: process requests, pass them to handlers, reject them if they are malformed
pub struct HttpServer {
    /// The hostname the server is listening to for requests
    pub hostname: String,
    /// The port the server is listening for requests
    pub port: u16,

    handlers: HashMap<String, Vec<Handler>>,
}

impl HttpServer {
    /// Initialize a [`HttpServer`] by passing a hostname and a port number
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
        self.append_handler(path.into(), HandlerHttpMethod::Any, handler);
    }

    /// Same as the [`on()`](`HttpServer::on()`) function, but processes only GET requests
    pub fn on_get<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(
            path.into(),
            HandlerHttpMethod::Specific(HttpMethod::GET),
            handler,
        );
    }

    /// Same as the [`on()`](`HttpServer::on()`) function, but processes only HEAD requests
    pub fn on_head<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(
            path.into(),
            HandlerHttpMethod::Specific(HttpMethod::HEAD),
            handler,
        );
    }

    /// Same as the [`on()`](`HttpServer::on()`) function, but processes only POST requests
    pub fn on_post<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(
            path.into(),
            HandlerHttpMethod::Specific(HttpMethod::POST),
            handler,
        );
    }

    /// Same as the [`on()`](`HttpServer::on()`) function, but processes only PUT requests
    pub fn on_put<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(
            path.into(),
            HandlerHttpMethod::Specific(HttpMethod::PUT),
            handler,
        );
    }

    /// Same as the [`on()`](`HttpServer::on()`) function, but processes only DELETE requests
    pub fn on_delete<S>(&mut self, path: S, handler: HandlerCallback)
    where
        S: Into<String>,
    {
        self.append_handler(
            path.into(),
            HandlerHttpMethod::Specific(HttpMethod::DELETE),
            handler,
        );
    }

    fn append_handler(
        &mut self,
        path: String,
        method: HandlerHttpMethod,
        handler: HandlerCallback,
    ) {
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
        let mut connection = HttpConnection::new(stream);

        let mut connection_open = true;
        while connection_open {
            let request = match HttpRequest::new(&mut connection) {
                Some(value) => value,
                None => {
                    eprintln!("Couldn't create new request for connection. Dropping connection...");
                    break;
                }
            };

            // Create a HttpResponse beforehand that will be used in case an error occurs
            let mut err_response = HttpResponse::new(&mut connection);

            // Before responding, check if the HTTP version of the request is supported (HTTP/1.1)
            if request.version != HttpVersion::new(VERSION).unwrap() {
                eprintln!(
                    "Expected HTTP version {}, found {}. Dropping connection...",
                    VERSION, request.version
                );
                err_response.status(HttpStatus::new(400).unwrap());
                err_response.end();
                break;
            }

            // Then check if a `Host` was sent, else respond with a 400 status code
            if request.version != HttpVersion::new(VERSION).unwrap() {
                eprintln!("Expected 'Host' header, found nothing. Dropping connection...");
                err_response.status(HttpStatus::new(400).unwrap());
                err_response.end();
                break;
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
            if let Some(handlers) = self.handlers.get(&request.target.absolute_path) {
                for handler in handlers {
                    match &handler.0 {
                        HandlerHttpMethod::Specific(method) => {
                            if request.method == *method {
                                (handler.1)(request.clone(), HttpResponse::new(&mut connection))
                            }
                        }
                        HandlerHttpMethod::Any => {
                            (handler.1)(request.clone(), HttpResponse::new(&mut connection))
                        }
                    }
                }
            } else {
                // Otherwise, respond with a HTTP 404 Not Found status
                err_response.status(HttpStatus::new(404).unwrap());
                err_response.end();
                break;
            }
        }

        connection.terminate_connection()
    }
}

/// A struct representing a HTTP connection between a client and the server
pub struct HttpConnection {
    /// The address of the peer client (if known)
    pub peer_address: io::Result<SocketAddr>,

    stream: TcpStream,
}

impl HttpConnection {
    /// Create a new [`HttpConnection`] from a [`TcpStream`]
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
    /// Note: the [`HttpConnection`] struct shouldn't be used after this function returns
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
pub struct HttpRequest {
    /// The request's method
    pub method: HttpMethod,
    /// The target URL of the method
    pub target: HttpTarget,
    /// The HTTP version the client supports
    pub version: HttpVersion,

    /// A Vec containing a list of the headers of the [`HttpRequest`]
    pub headers: Vec<HttpHeader>,
}

impl HttpRequest {
    /// Create a new [`HttpRequest`] from a [`HttpConnection`]
    pub fn new(parent: &mut HttpConnection) -> Option<Self> {
        // Begin by reading the first line
        let first_line = read_line(&mut parent.stream);
        // Then split it by whitespace
        let mut splitted_first_line = first_line.split_whitespace();

        // Create a HttpResponse beforehand that will be used in case an error occurs
        let mut err_response = HttpResponse::new(parent);

        // Check if the resulting slices aren't three in number (as they should be)
        if splitted_first_line.clone().count() != 3 {
            // If yes, print an error message to stderr and immediately terminate connection
            eprintln!("Invalid HTTP request detected. Dropping connection...");
            err_response.status(HttpStatus::new(400).unwrap());
            err_response.end();
            return None;
        }

        // Else, start obtaining the HTTP method, target and version, terminating the connection in case of errors
        let Some(method) = HttpMethod::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP method detected. Dropping connection...");
			err_response.status(HttpStatus::new(501).unwrap());
			err_response.end();
			return None;
		};
        let target = HttpTarget::new(splitted_first_line.next().unwrap());
        // Note: a HttpVersion structs will only check if the HTTP version is in the format "HTTP/{num}.{num}" and won't check if the major and minor revisions of the HTTP protocol exist. This check will occur later on our code
        let Some(http_version) = HttpVersion::new(splitted_first_line.next().unwrap()) else {
			eprintln!("Invalid HTTP version detected. Dropping connection...");
			err_response.status(HttpStatus::new(400).unwrap());
			err_response.end();
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
pub struct HttpResponse<'s> {
    parent: &'s mut HttpConnection,

    /// The HTTP status code of the response
    pub status: HttpStatus,
    /// The HTTP version of the response
    pub version: HttpVersion,

    /// A Vec containing the headers of the response
    pub headers: Vec<HttpHeader>,
}

impl<'s> HttpResponse<'s> {
    /// Create a new [`HttpResponse`]
    pub fn new(parent: &'s mut HttpConnection) -> Self {
        Self {
            parent,
            status: HttpStatus::new(200).unwrap(),
            version: HttpVersion::new(VERSION).unwrap(),
            headers: Vec::new(),
        }
    }

    /// CHange the [`HttpStatus`] of the response
    pub fn status(&mut self, status: HttpStatus) {
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
