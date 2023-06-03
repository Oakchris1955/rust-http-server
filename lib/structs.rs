use std::fmt;

/// A rather simple struct implementing an complicated initialization method and the `Display trait`
#[derive(PartialEq, Clone)]
pub struct HttpVersion {
	pub major: usize,
	pub minor: usize
}

impl HttpVersion {
	/// Initialize a `HttpVersion` by passing a string to it in the format `HTTP/{int}.{int}`
	pub fn new(version: &str) -> Option<Self> {
		if version.len() >= 5 {
			if &version[0..4] == "HTTP" && &version[4..5] == "/" {
				let version_split = &mut version[5..].split(".");
				if version_split.clone().count() == 2 {
					let parse_int = |option_input: Option<&str>| -> Option<usize> {
						let Some(string_num) = option_input else {
							return None;
						};

						let Ok(number) = string_num.parse::<usize>() else {
							return None;
						};

						Some(number)
					};

					let Some(major) = parse_int(version_split.next()) else {return None};
					let Some(minor) = parse_int(version_split.next()) else {return None};

					return Some(Self {
						major,
						minor
					})
				}
			}
		}

		None
	}
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/{}.{}", self.major, self.minor)
    }
}

/// A rather generalized HTTP header struct
#[derive(Clone)]
pub struct HttpHeader {
	pub name: String,
	pub value: String
}

impl HttpHeader {
	pub fn new(header: &str) -> Option<Self> {
		let Some((name, value)) = header.split_once(": ") else {
			return None;
		};

		Some(Self {
			name: name.to_string(),
			value: value.to_string()
		})
	}
}

impl fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Clone)]
pub struct HttpTarget {
	pub absolute_path: String,
	pub queries: Vec<QueryParameter>
}

impl HttpTarget {
	pub fn new<S>(target: S) -> Self where S: Into<String> {
		let target_string: String = Self::decode_url(target.into());

		let (absolute_path, queries_str) = target_string.split_once('?').unwrap_or((&target_string, ""));

		let mut queries = Vec::new();

		if !queries_str.is_empty() {

			let queries_split = queries_str.split("&");

			for query_str in queries_split {
				if let Some((name, value)) = query_str.split_once("=") {
					queries.push(
						QueryParameter {
							name: name.to_string(),
							value: value.to_string()
						}
					)
				}
			};

		}

		Self {
			absolute_path: absolute_path.to_string(),
			queries
		}
	}

	fn decode_url(encoded_url: String) -> String {
		let mut url_iterator = encoded_url.split("%");
		
		[url_iterator.next().unwrap().to_string(),
			url_iterator.map(|str_to_decode| {

				if str_to_decode.len() >= 2 {
					if str_to_decode[..2].chars().all(|char_to_check| {
						char_to_check.is_digit(16)
					}) {
						let mut concatenated_string = String::new();
						concatenated_string.push(char::from_u32(u32::from_str_radix(&str_to_decode[..2], 16).unwrap()).unwrap());
						concatenated_string.push_str(&str_to_decode[2..]);
						return concatenated_string;
					}
				}

				str_to_decode.to_string()
			}).collect::<Vec<String>>().join("")
		].join("")
	}
}

impl fmt::Display for HttpTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.absolute_path, {
			let mut queries_string = self.queries.iter().map(|query| {
				let mut query_string = query.to_string();
				query_string.push('&');

				query_string
			}).collect::<String>();

			if !queries_string.is_empty() {
				queries_string.insert_str(0, "?");
				queries_string.pop();
			}
			
			queries_string
		})
    }
}

#[derive(Debug, Clone)]
pub struct QueryParameter {
	pub name: String,
	pub value: String
}

impl fmt::Display for QueryParameter {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.name, self.value)
    }
}