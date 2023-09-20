use std::{
    collections::HashMap,
    fmt, hash,
    time::{Duration, SystemTime},
};

use crate::utils::format_time;

/// The HTTP version of a request or a response
#[derive(PartialEq, Clone)]
pub struct Version {
    /// The major revision number of the HTTP version
    pub major: usize,
    /// The minor revision number of the HTTP version
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
    /// # use oak_http_server::Version;
    ///
    /// fn main() {
    /// 	let version = Version::new("HTTP/1.1").unwrap(); // Unwrap the `Some` value the `new` function returns
    /// 	println!("{}", version); // Prints "HTTP/1.1" in the console
    /// }
    /// ```
    pub fn new<S>(version: S) -> Option<Self>
    where
        S: ToString,
    {
        let version = version.to_string();

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

/// Represents a HTTP URL (named [`Target`] for formality reasons)
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
        S: ToString,
    {
        let target_string: String = Self::decode_url(target.to_string());

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

/// Controls whether or not a cookie is sent with cross-site requests
#[derive(PartialEq, Eq, Clone)]
pub enum SameSite {
    /// The browser sends the cookie only for same-site requests
    Strict,
    /// The cookie is not sent on cross-site requests,
    /// but is sent when a user is navigating to the origin site from an external site.
    ///
    /// This is the default behaviour
    Lax,
    /// The browser sends the cookie with both cross-site and same-site requests.
    ///
    /// The [`Secure`](Cookie::secure) attribute must also be set when setting this value
    None,
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Strict => "Strict",
                Self::Lax => "Lax",
                Self::None => "None",
            }
        )
    }
}

/// Represents a HTTP cookie
///
/// This struct also implements some functions that allow some form of function chaining.
///
/// For example:
/// ```
/// # #[cfg(feature = "safe")]
/// # extern crate oak_http_server;
/// // Library imports
/// use oak_http_server::Cookie;
///
/// fn main() {
///     // Create a new cookie
///     let mut cookie = Cookie::new("my_cookie", "some-value");
///
///     // Set the HTTP-Only attribute
///     cookie.set_http_only(true)
///     // Set the path attribute
///           .set_path("/some/path");
///
///     // Cookie doesn't implement Debug, but it implements Eq and PartialEq
///     // For this reason, we use assert!() instead of assert_eq!()
///     assert!(
///         cookie
///             == Cookie {
///                 name: String::from("my_cookie"),
///                 value: String::from("some-value"),
///                 domain: None,
///                 expires: None,
///                 http_only: true,
///                 path: Some(String::from("/some/path")),
///                 same_site: None,
///                 secure: false,
///             }
///     )
/// }
/// ```
#[derive(Clone)]
pub struct Cookie {
    /// The name of the cookie
    pub name: String,
    /// The value of the cookie
    pub value: String,

    /// Defines the host to which the cookie will be sent.
    ///
    /// Only the current domain can be set as the value, or a domain of a higher order, unless it is a public suffix.
    /// Setting the domain will make the cookie available to it, as well as to all its subdomains.
    pub domain: Option<String>,
    /// Indicates when the cookie is gonna expire. If None, this is a session cookie.
    pub expires: Option<SystemTime>,
    /// Forbids JavaScript from accessing the cookie, for example, through the `Document.cookie` property.
    pub http_only: bool,
    /// Indicates the path that **must** exist in the requested URL for the browser to send the cookie
    pub path: Option<String>,
    /// Check [`SameSite`] documentation
    pub same_site: Option<SameSite>,
    /// Indicates that the cookie is sent to the server only when a request is made with the `https:` scheme (except on `localhost`)
    pub secure: bool,
}

impl Cookie {
    /// Create a new [`Cookie`]
    pub fn new<S>(name: S, value: S) -> Self
    where
        S: ToString,
    {
        Self {
            name: Self::replace_with_whitespace(name.to_string(), false),
            value: Self::replace_with_whitespace(value.to_string(), true),
            domain: None,
            expires: None,
            http_only: false,
            path: None,
            same_site: None,
            secure: false,
        }
    }

    fn replace_with_whitespace(mut string: String, is_value: bool) -> String {
        // According to RFC 6265, Section 4.1.1, a cookie value MAY be surrounded by double quotes
        // The if-statement below check for that and removes them if they exist
        if string.chars().clone().count() >= 2 && is_value {
            if string.chars().next().unwrap() == '\"' && string.chars().last().unwrap() == '\"' {
                let mut chars = string.chars();
                chars.next();
                chars.next_back();

                string = chars.as_str().to_string();
            }
        }

        // Replace illegal characters according to RFC 6265, Section 4.1.1,
        string
            .chars()
            .map(|x| match x {
                ' ' | '\t' | '\"' | ',' | ';' | '/' => '_',
                '(' | ')' | '<' | '>' | '@' | '\\' | '[' | ']' | '?' | '=' | '{' | '}' => {
                    if is_value {
                        x
                    } else {
                        '_'
                    }
                }
                _ => x,
            })
            .collect()
    }

    /// Change cookie domain chain function
    pub fn set_domain(&mut self, domain: String) -> &mut Self {
        self.domain = Some(domain);
        self
    }

    /// Set cookie expiration chain function
    pub fn set_expires(&mut self, expires: SystemTime) -> &mut Self {
        self.expires = Some(expires);
        self
    }

    /// Set cookie HttpOnly attribute chain function
    pub fn set_http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = http_only;
        self
    }

    /// Set cookie path chain function
    pub fn set_path<S>(&mut self, path: S) -> &mut Self
    where
        S: ToString,
    {
        self.path = Some(path.to_string());
        self
    }

    /// Set cookie SameSite attribute chain function
    pub fn set_same_site(&mut self, same_site: SameSite) -> &mut Self {
        self.same_site = Some(same_site);
        self
    }

    /// Set cookie Secure attribute
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }
}

impl PartialEq for Cookie {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Cookie {}

impl hash::Hash for Cookie {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}{}", self.name, self.value, {
            // Create an attributes hashmap and an output string
            let mut attributes: HashMap<String, Option<String>> = HashMap::new();
            let mut output: String = String::from("; ");

            // Push struct fields to hashmap accordingly
            if let Some(domain) = &self.domain {
                attributes.insert(String::from("Domain"), Some(domain.clone()));
            }
            if let Some(expires) = &self.expires {
                attributes.insert(String::from("Expires"), Some(format_time(expires.clone())));
                attributes.insert(
                    String::from("Max-Age"),
                    Some(
                        expires
                            .duration_since(SystemTime::now())
                            // We assume that the reason this failed is because expires < Duration::now()
                            // So, we set the Max-Age to zero, which means that this cookie immediately expires
                            .unwrap_or(Duration::ZERO)
                            .as_secs()
                            .to_string(),
                    ),
                );
            }
            if self.http_only {
                attributes.insert(String::from("HttpOnly"), None);
            }
            if let Some(path) = &self.path {
                attributes.insert(String::from("Path"), Some(path.clone()));
            }
            if let Some(same_site) = &self.same_site {
                attributes.insert(String::from("Same-Site"), Some(same_site.to_string()));
                if same_site == &SameSite::None {
                    attributes.insert(String::from("Secure"), None);
                }
            }
            if self.secure {
                attributes.insert(String::from("Secure"), None);
            }

            // Loop through the hashmap
            attributes.iter().for_each(|(name, value)| {
                if let Some(value) = value {
                    // If there is some value, push it to the output string alongside the field name
                    output.push_str(&format!("{}={}", name, value))
                } else {
                    // Otherwise, push just the field name
                    output.push_str(&format!("{}", name))
                }

                // Push attribute seperator
                output.push_str("; ")
            });

            // Remove trailing attribute seperator
            output.pop();
            output.pop();

            // Return output strings
            output
        })
    }
}
