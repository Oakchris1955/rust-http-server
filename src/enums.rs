use std::fmt;

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
