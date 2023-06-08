use std::{collections::HashMap, fmt};

/// The HTTP version of a request or a response
#[derive(PartialEq, Clone)]
pub struct HttpVersion {
    pub major: usize,
    pub minor: usize,
}

impl HttpVersion {
    /// Initialize a [`HttpVersion`] by passing a [`&str`] or [`String`] to it in the format `HTTP/{major}.{minor}`. The corresponding numbers `major` and `minor` represent a HTTP version and are stored in the struct's fields.
    ///
    /// If for whatever reason this function fails to parse the [`&str`] provided into a [`HttpVersion`], it will return [`None`].
    ///
    /// If the [`&str`] provided is parsed successfully, then the function will return a [`Some`] value containing a [`HttpVersion`] struct
    ///
    /// # Example
    ///
    /// ```
    /// fn main() {
    /// 	let version = HttpVersion::new("HTTP/1.1").unwrap(); // Unwrap the `Some` value the `new` function returns
    /// 	println!("{}", version); // Prints "HTTP/1.1" in the console
    /// }
    /// ```
    pub fn new<S>(version: S) -> Option<Self>
    where
        S: Into<String>,
    {
        let version = version.into();

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

                    return Some(Self { major, minor });
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

/// Represents a HTTP header
#[derive(Clone)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    /// Parses a [`&str`] or [`String`] in the following format: "{header name}: {header value}" into a [`HttpHeader`]
    pub fn new<S>(header: S) -> Option<Self>
    where
        S: Into<String>,
    {
        let header = header.into();

        let Some((name, value)) = header.split_once(": ") else {
			return None;
		};

        Some(Self {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

impl fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// Represents a HTTP URL (named `HttpTarget` for formality reasons)
#[derive(Clone)]
pub struct HttpTarget {
    /// Stores the URL path, according to RFC 3986
    pub absolute_path: String,
    /// A HashMap with a String key representing the query value and a String value representing the query value (query is defined in RFC 3986 as well)
    pub queries: HashMap<String, String>,
}

impl HttpTarget {
    /// Parses a [`&str`] or [`String`] into a [`HttpTarget`]
    pub fn new<S>(target: S) -> Self
    where
        S: Into<String>,
    {
        let target_string: String = Self::decode_url(target.into());

        let (absolute_path, queries_str) = target_string
            .split_once('?')
            .unwrap_or((&target_string, ""));

        let mut queries = HashMap::new();

        if !queries_str.is_empty() {
            let queries_split = queries_str.split("&");

            for query_str in queries_split {
                if let Some((name, value)) = query_str.split_once("=") {
                    queries.insert(name.to_string(), value.to_string());
                }
            }
        }

        Self {
            absolute_path: absolute_path.to_string(),
            queries,
        }
    }

    fn decode_url(encoded_url: String) -> String {
        let mut url_iterator = encoded_url.split("%");

        [
            url_iterator.next().unwrap().to_string(),
            url_iterator
                .map(|str_to_decode| {
                    if str_to_decode.len() >= 2 {
                        if str_to_decode[..2]
                            .chars()
                            .all(|char_to_check| char_to_check.is_digit(16))
                        {
                            let mut concatenated_string = String::new();
                            concatenated_string.push(
                                char::from_u32(
                                    u32::from_str_radix(&str_to_decode[..2], 16).unwrap(),
                                )
                                .unwrap(),
                            );
                            concatenated_string.push_str(&str_to_decode[2..]);
                            return concatenated_string;
                        }
                    }

                    str_to_decode.to_string()
                })
                .collect::<Vec<String>>()
                .join(""),
        ]
        .join("")
    }
}

impl fmt::Display for HttpTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.absolute_path, {
            let mut queries_string = self
                .queries
                .iter()
                .map(|(name, value)| format!("{}: {}&", name, value))
                .collect::<String>();

            if !queries_string.is_empty() {
                queries_string.insert_str(0, "?");
                queries_string.pop();
            }

            queries_string
        })
    }
}
