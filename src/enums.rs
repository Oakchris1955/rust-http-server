use std::fmt;

pub enum HttpMethod {
	GET,
	HEAD
}

impl HttpMethod {
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
