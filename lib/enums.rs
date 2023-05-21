use std::fmt;

pub enum HttpStatus {
	OK,
	BadRequest
}

impl HttpStatus {
	/// Initialize a `HttpStatus` by passing a `usize` to it (for example, pass `200` to get a `HttpStatus::OK` variant)
	pub fn new(status: usize) -> Option<Self> {
		match status {
			200 => Some(Self::OK),
			400 => Some(Self::BadRequest),
			_ => None
		}
	}
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
			Self::OK => 200,
			Self::BadRequest => 400
		})
    }
}


/// A rather simple enum implementing an initialization method and the `Display` trait
pub enum HttpMethod {
	GET,
	HEAD
}

impl HttpMethod {
	/// Initialize a `HttpMethod` by passing a `&str` to it (for example, pass `"GET"` to get a `HttpMethod::GET` variant)
	pub fn new(method: &str) -> Option<Self> {
		match method {
			"GET" => Some(Self::GET),
			"HEAD" => Some(Self::HEAD),
			_ => None
		}
	}
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
			Self::GET => "GET",
			Self::HEAD => "HEAD"
		})
    }
}
