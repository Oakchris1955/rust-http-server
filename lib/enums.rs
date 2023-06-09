use std::fmt;

/// A HTTP status to include in a [`Response`](crate::Response)
#[derive(PartialEq)]
#[non_exhaustive]
pub enum Status {
    /// `200 OK`
    OK,
    /// `201 Created`
    Created,
    /// `202 Accepted`
    Accepted,
    /// `203 No Content`
    NoContent,

    /// `400 Bad Request`
    BadRequest,
    /// `404 Not Found`
    NotFound,

    /// `500 Internal Server Error`
    InternalError,
    /// `501 Not Implemented`
    NotImplemented,
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
    /// fn main() {
    /// 	// Generate a new HTTP Status instance (in our case, Status::OK)
    /// 	let status: Option<HttpStatus> = Status::new(200);
    ///
    /// 	assert_eq!(status, Some(Status::OK));
    /// }
    /// ```
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
            _ => None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OK => 200,
                Self::Created => 201,
                Self::Accepted => 202,
                Self::NoContent => 204,

                Self::BadRequest => 400,
                Self::NotFound => 404,

                Self::InternalError => 500,
                Self::NotImplemented => 501,
            }
        )
    }
}

/// A HTTP method that is provided by the client
#[derive(PartialEq, Clone)]
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
