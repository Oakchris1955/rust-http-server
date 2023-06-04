use std::fmt;

#[derive(PartialEq)]
#[non_exhaustive]
pub enum HttpStatus {
	OK,
	Created,
	Accepted,
	NoContent,

	BadRequest,
	NotFound,

	InternalError,
	NotImplemented
}

impl HttpStatus {
	/// Initialize a `HttpStatus` by passing a `usize` to it (for example, pass `200` to get a `HttpStatus::OK` variant)
	pub fn new(status: usize) -> Option<Self> {
		match status {
			200 => Some(Self::OK),
			201 => Some(Self::Created),
			202 => Some(Self::Accepted),
			204 => Some(Self::NoContent),

			400 => Some(Self::BadRequest),
			404 => Some(Self::NotFound),

			500 => Some(Self::InternalError),
			501 => Some(Self::NotImplemented),
			_ => None
		}
	}
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
			Self::OK => 200,
			Self::Created => 201,
			Self::Accepted => 202,
			Self::NoContent => 204,

			Self::BadRequest => 400,
			Self::NotFound => 404,

			Self::InternalError => 500,
			Self::NotImplemented => 501
		})
    }
}


/// A rather simple enum implementing an initialization method and the `Display` trait
#[derive(PartialEq, Clone)]
#[non_exhaustive]
pub enum HttpMethod {
	GET,
	HEAD,
	POST,
	PUT,
	DELETE
}

impl HttpMethod {
	/// Initialize a `HttpMethod` by passing a `&str` to it (for example, pass `"GET"` to get a `HttpMethod::GET` variant)
	pub fn new(method: &str) -> Option<Self> {
		match method {
			"GET" => Some(Self::GET),
			"HEAD" => Some(Self::HEAD),
			"POST" => Some(Self::POST),
			"PUT" => Some(Self::PUT),
			"DELETE" => Some(Self::DELETE),
			_ => None
		}
	}
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
			Self::GET => "GET",
			Self::HEAD => "HEAD",
			Self::POST => "POST",
			Self::PUT => "PUT",
			Self::DELETE => "DELETE"
		})
    }
}
