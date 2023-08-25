use std::fmt;

/// A HTTP status to include in a [`Response`](crate::Response)
#[derive(PartialEq, Debug, Clone)]
#[non_exhaustive]
pub enum Status {
    /// `100 Continue`
    Continue,
    /// `101 Switching Protocols`
    SwitchingProtocols,
    /// `102 Processing`
    Processing,
    /// `103 Early Hints`
    EarlyHints,

    /// `200 OK`
    OK,
    /// `201 Created`
    Created,
    /// `202 Accepted`
    Accepted,
    /// `203 Non-Authoritative Information`
    NonAuthoritativeInformation,
    /// `204 No Content`
    NoContent,
    /// `205 Reset Content`
    ResetContent,
    /// `206 Partial Content`
    PartialContent,
    /// `207 Multi-Status`
    MultiStatus,
    /// `208 Already Reported`
    AlreadyReported,
    /// `226 IM Used`
    ImUsed,

    /// `300 Multiple Choices`
    MultipleChoices,
    /// `301 Moved Permanently`
    MovedPermanently,
    /// `302 Found`
    Found,
    /// `303 See Other`
    SeeOther,
    /// `304 Not Modified`
    NotModified,
    /// `305 Use Proxy`
    UseProxy,
    /// `307 Temporary Redirect`
    TemporaryRedirect,
    /// `308 Permanent Redirect`
    PermanentRedirect,

    /// `400 Bad Request`
    BadRequest,
    /// `401 Unauthorized`
    Unauthorized,
    /// `402 Payment Required`
    PaymentRequired,
    /// `403 Forbidden`
    Forbidden,
    /// `404 Not Found`
    NotFound,
    /// `405 Method Not Allowed`
    MethodNotAllowed,
    /// `406 Not Acceptable`
    NotAcceptable,
    /// `407 Proxy Authentication Required`
    ProxyAuthenticationRequired,
    /// `408 Request Timeout`
    RequestTimeout,
    /// `409 Conflict`
    Conflict,
    /// `410 Gone`
    Gone,
    /// `411 Length Required`
    LengthRequired,
    /// `412 Precondition Failed`
    PreconditionFailed,
    /// `413 Content Too Large`
    ContentTooLarge,
    /// `414 URI Too Long`
    UriTooLong,
    /// `415 Unsupported Media Type`
    UnsupportedMediaType,
    /// `416 Range Not Satisfiable`
    RangeNotSatisfiable,
    /// `417 Expectation Failed`
    ExpectationFailed,
    /// `418 I'm a teapot`
    ImATeapot,
    /// `421 Misdirected Request`
    MisdirectedRequest,
    /// `422 Unprocessable Content`
    UnprocessableContent,
    /// `423 Locked`
    Locked,
    /// `424 Failed Dependency`
    FailedDependency,
    /// `425 Too Early`
    TooEarly,
    /// `426 Upgrade Required`
    UpgradeRequired,
    /// `428 Precondition Required`
    PreconditionRequired,
    /// `429 Too Many Requests`
    TooManyRequests,
    /// `431 Request Header Fields Too Large`
    RequestHeaderFieldsTooLarge,
    /// `451 Unavailable For Legal Reasons`
    UnavailableForLegalReasons,

    /// `500 Internal Server Error`
    InternalServerError,
    /// `501 Not Implemented`
    NotImplemented,
    /// `502 Bad Gateway`
    BadGateway,
    /// `503 Service Unavailable`
    ServiceUnavailable,
    /// `504 Gateway Timeout`
    GatewayTimeout,
    /// `505 HTTP Version Not Supported`
    HttpVersionNotSupported,
    /// `506 Variant Also Negotiates`
    VariantAlsoNegotiates,
    /// `507 Insufficient Storage`
    InsufficientStorage,
    /// `508 Loop Detected`
    LoopDetected,
    /// `511 Network Authentication Required`
    NetworkAuthenticationRequired,

    /// A HTTP status code not defined here (whether official or not).
    ///
    /// Please note that the default status text of this is blank, so make sure to use
    /// the [`Self::other_text`] function to set it if you know an enum variant is [`Other`](Self::Other)
    Other(usize, String),
}

impl Status {
    /// Returns an [`Option`] containing [`Status`] by passing a [`usize`] corresponding to the HTTP status code to it
    ///
    /// If the status provided is a valid HTTP status, this function will evaluate to [`Some`] containing [`Self`]
    ///
    /// If the status provided isn't valid or implemented yet, this function will return [`None`]
    ///
    /// # Example
    ///
    /// ```
    /// # use oak_http_server::Status;
    /// #
    /// fn main() {
    /// 	// Generate a new HTTP Status instance (in our case, Status::OK)
    /// 	let status: Status = Status::OK;
    ///
    /// 	assert_eq!(status, Status::OK);
    /// }
    /// ```
    pub fn new(status: usize) -> Self {
        Self::from(status)
    }

    /// If enum variant is [`Other`](Self::Other), change it's status text
    pub fn other_text<S>(self, text: S) -> Self
    where
        S: ToString,
    {
        match self {
            Self::Other(status_code, _) => Self::Other(status_code, text.to_string()),
            _ => self,
        }
    }

    /// Get the corresponding status text for [`Self`]
    pub fn get_status_text(&self) -> String {
        match self {
            Self::Continue => String::from("Continue"),
            Self::SwitchingProtocols => String::from("Switching Protocols"),
            Self::Processing => String::from("Processing"),
            Self::EarlyHints => String::from("Early Hints"),

            Self::OK => String::from("OK"),
            Self::Created => String::from("Created"),
            Self::Accepted => String::from("Accepted"),
            Self::NonAuthoritativeInformation => String::from("Non-Authoritative Information"),
            Self::NoContent => String::from("No Content"),
            Self::ResetContent => String::from("Reset Content"),
            Self::PartialContent => String::from("Partial Content"),
            Self::MultiStatus => String::from("Multi-Status"),
            Self::AlreadyReported => String::from("Already Reported"),
            Self::ImUsed => String::from("IM Used"),

            Self::MultipleChoices => String::from("Multiple Choices"),
            Self::MovedPermanently => String::from("Moved Permanently"),
            Self::Found => String::from("Found"),
            Self::SeeOther => String::from("See Other"),
            Self::NotModified => String::from("Not Modified"),
            Self::UseProxy => String::from("Use Proxy"),
            Self::TemporaryRedirect => String::from("Temporary Redirect"),
            Self::PermanentRedirect => String::from("Permanent Redirect"),

            Self::BadRequest => String::from("Bad Request"),
            Self::Unauthorized => String::from("Unauthorized"),
            Self::PaymentRequired => String::from("Payment Required"),
            Self::Forbidden => String::from("Forbidden"),
            Self::NotFound => String::from("Not Found"),
            Self::MethodNotAllowed => String::from("Method Not Allowed"),
            Self::NotAcceptable => String::from("Not Acceptable"),
            Self::ProxyAuthenticationRequired => String::from("Proxy Authentication Required"),
            Self::RequestTimeout => String::from("Request Timeout"),
            Self::Conflict => String::from("Conflict"),
            Self::Gone => String::from("Gone"),
            Self::LengthRequired => String::from("Length Required"),
            Self::PreconditionFailed => String::from("Precondition Failed"),
            Self::ContentTooLarge => String::from("Content Too Large"),
            Self::UriTooLong => String::from("URI Too Long"),
            Self::UnsupportedMediaType => String::from("Unsupported Media Type"),
            Self::RangeNotSatisfiable => String::from("Range Not Satisfiable"),
            Self::ExpectationFailed => String::from("Expectation Failed"),
            Self::ImATeapot => String::from("I'm a teapot"),
            Self::MisdirectedRequest => String::from("Misdirected Request"),
            Self::UnprocessableContent => String::from("Unprocessable Content"),
            Self::Locked => String::from("Locked"),
            Self::FailedDependency => String::from("Failed Dependency"),
            Self::TooEarly => String::from("Too Early"),
            Self::UpgradeRequired => String::from("Upgrade Required"),
            Self::PreconditionRequired => String::from("Precondition Required"),
            Self::TooManyRequests => String::from("Too Many Requests"),
            Self::RequestHeaderFieldsTooLarge => String::from("Request Header Fields Too Large"),
            Self::UnavailableForLegalReasons => String::from("Unavailable For Legal Reasons"),

            Self::InternalServerError => String::from("Internal Server Error"),
            Self::NotImplemented => String::from("Not Implemented"),
            Self::BadGateway => String::from("Bad Gateway"),
            Self::ServiceUnavailable => String::from("Service Unavailable"),
            Self::GatewayTimeout => String::from("Gateway Timeout"),
            Self::HttpVersionNotSupported => String::from("HTTP Version Not Supported"),
            Self::VariantAlsoNegotiates => String::from("Variant Also Negotiates"),
            Self::InsufficientStorage => String::from("Insufficient Storage"),
            Self::LoopDetected => String::from("Loop Detected"),
            Self::NetworkAuthenticationRequired => String::from("Network Authentication Required"),

            Self::Other(_, status_text) => status_text.clone(),
        }
    }
}

impl From<usize> for Status {
    fn from(value: usize) -> Self {
        let integer = value.into();

        match integer {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            102 => Self::Processing,
            103 => Self::EarlyHints,

            200 => Self::OK,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            207 => Self::MultiStatus,
            208 => Self::AlreadyReported,
            226 => Self::ImUsed,

            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            305 => Self::UseProxy,
            307 => Self::TemporaryRedirect,
            308 => Self::PermanentRedirect,

            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::ContentTooLarge,
            414 => Self::UriTooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            418 => Self::ImATeapot,
            421 => Self::MisdirectedRequest,
            422 => Self::UnprocessableContent,
            423 => Self::Locked,
            424 => Self::FailedDependency,
            425 => Self::TooEarly,
            426 => Self::UpgradeRequired,
            428 => Self::PreconditionRequired,
            429 => Self::TooManyRequests,
            431 => Self::RequestHeaderFieldsTooLarge,
            451 => Self::UnavailableForLegalReasons,

            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HttpVersionNotSupported,
            506 => Self::VariantAlsoNegotiates,
            507 => Self::InsufficientStorage,
            508 => Self::LoopDetected,
            511 => Self::NetworkAuthenticationRequired,

            _ => Self::Other(integer, String::new()),
        }
    }
}

impl From<&usize> for Status {
    fn from(value: &usize) -> Self {
        value.clone().into()
    }
}

impl Into<usize> for Status {
    fn into(self) -> usize {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,

            Self::OK => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::ImUsed => 226,

            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,

            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::ContentTooLarge => 413,
            Self::UriTooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableContent => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,

            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HttpVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NetworkAuthenticationRequired => 511,

            Self::Other(status_code, _) => status_code,
        }
    }
}

impl Into<usize> for &Status {
    fn into(self) -> usize {
        self.clone().into()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_code: usize = self.into();
        let status_text = self.get_status_text();
        write!(
            f,
            "{}{}{}",
            status_code,
            if !status_text.is_empty() { " " } else { "" },
            status_text
        )
    }
}

/// A HTTP method that is provided by the client
#[derive(PartialEq, Clone, Debug)]
#[non_exhaustive]
pub enum Method {
    /// The `GET` method requests a representation of the specified resource.
    /// Requests using `GET` should only retrieve data.
    GET,
    /// The `HEAD` method asks for a response identical to a `GET` request, but without the response body.
    HEAD,
    /// The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    POST,
    /// The `PUT` method replaces all current representations of the target resource with the request payload.
    PUT,
    /// The `DELETE` method deletes the specified resource.
    DELETE,
}

impl Method {
    /// Returns an [`Option`] containing [`Method`] by passing a [`&str`] or [`String`] corresponding to a HTTP method
    ///
    /// If the method provided is a valid HTTP method, this function will evaluate to [`Some`] containing [`Self`]
    ///
    /// If the method provided isn't valid or implemented yet, this function will return [`None`]
    ///
    /// # Example
    ///
    /// ```
    /// # use oak_http_server::Method;
    ///
    /// fn main() {
    /// 	// Create a new HTTP Method instance (in our case, Method::GET)
    /// 	let method: Option<Method> = Method::new("GET");
    ///
    /// 	assert_eq!(method, Some(Method::GET));
    /// }
    /// ```
    pub fn new<S>(method: S) -> Option<Self>
    where
        S: Into<String>,
    {
        match method.into().as_str() {
            "GET" => Some(Self::GET),
            "HEAD" => Some(Self::HEAD),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            _ => None,
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GET => "GET",
                Self::HEAD => "HEAD",
                Self::POST => "POST",
                Self::PUT => "PUT",
                Self::DELETE => "DELETE",
            }
        )
    }
}
