use std::{collections::HashMap, fmt};

/// The HTTP version of a request or a response
#[derive(PartialEq, Clone)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
}

impl Version {
    /// Initialize a [`Version`] by passing a [`&str`] or [`String`] to it in the format `HTTP/{major}.{minor}`. The corresponding numbers `major` and `minor` represent a HTTP version and are stored in the struct's fields.
    ///
    /// If for whatever reason this function fails to parse the [`&str`] provided into a [`Version`], it will return [`None`].
    ///
    /// If the [`&str`] provided is parsed successfully, then the function will return a [`Some`] value containing a [`Version`] struct
    ///
    /// # Example
    ///
    /// ```
    /// fn main() {
    /// 	let version = Version::new("HTTP/1.1").unwrap(); // Unwrap the `Some` value the `new` function returns
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

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/{}.{}", self.major, self.minor)
    }
}

/// Represents a HTTP header
#[derive(Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    /// Parses a [`&str`] or [`String`] in the following format: "{header name}: {header value}" into a [`Header`]
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

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// Represents a HTTP URL (named `HttpTarget` for formality reasons)
#[derive(Clone)]
pub struct Target {
    /// Contains the path of the current handler (Empty by default. Modified by the server before being passed to a handler). Primarily used by directory handlers.
    ///
    /// For example, if a directory handler is assigned at path `\www\etc` and the client attempts to access `\www\etc\main.txt`,
    /// this field's String's contents  will be `\www\etc` and the [relative path](Self::relative_path) will be equal to `\main.txt`
    pub target_path: String,
    /// Check the [target path](Self::target_path) documentation
    pub relative_path: String,
    /// A HashMap with a String key representing the query value and a String value representing the query value (query is defined in RFC 3986 as well)
    pub queries: HashMap<String, String>,
}

impl Target {
    /// Parses a [`&str`] or [`String`] into a [`Target`]
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
            target_path: String::new(),
            relative_path: absolute_path.to_string(),
            queries,
        }
    }

    /// Returns the URL path, according to RFC 3986
    pub fn full_url(&self) -> String {
        format!("{}{}", &self.target_path, &self.relative_path)
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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.full_url(), {
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
