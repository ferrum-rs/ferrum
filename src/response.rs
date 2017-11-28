//! Ferrum's HTTP Response representation and associated methods.

use std::fmt::{self, Debug};
use std::mem::replace;

use typemap::{TypeMap, TypeMapInner};
use plugin::Extensible;
use hyper::{Body, HttpVersion};

use {Plugin, Header, Headers, StatusCode};

pub use hyper::Response as HyperResponse;

/// The response representation given to `Middleware`
pub struct Response {
    /// The response status-code.
    pub status: StatusCode,

    /// The headers of the response.
    pub headers: Headers,

    /// The body of the response.
    pub body: Option<Body>,

    /// A TypeMap to be used as an extensible storage for data
    /// associated with this Response.
    pub extensions: TypeMap<TypeMapInner>,
}

impl Response {
    /// Construct a blank Response
    pub fn new() -> Response {
        Response {
            status: Default::default(),
            headers: Headers::new(),
            body: None, // Start with no body.
            extensions: TypeMap::custom()
        }
    }

    /// Set the status and move the Response.
    ///
    /// Useful for the "builder-style" pattern.
    #[inline]
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Set a header and move the Response.
    ///
    /// Useful for the "builder-style" pattern.
    #[inline]
    pub fn with_header<H: Header>(mut self, header: H) -> Self {
        self.headers.set(header);
        self
    }

    /// Set the headers and move the Response.
    ///
    /// Useful for the "builder-style" pattern.
    #[inline]
    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    /// Set the body and move the Response.
    ///
    /// Useful for the "builder-style" pattern.
    #[inline]
    pub fn with_body<T: Into<Body>>(mut self, body: T) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl From<HyperResponse> for Response {
    fn from(mut from_response: HyperResponse) -> Response {
        Response {
            status: from_response.status(),
            headers: replace(from_response.headers_mut(), Headers::new()),
            body: if from_response.body_ref().is_some() { Some(from_response.body()) } else { None },
            extensions: TypeMap::custom()
        }
    }
}

impl From<Response> for HyperResponse {
    fn from(from_response: Response) -> HyperResponse {
        HyperResponse::new()
            .with_status(from_response.status)
            .with_headers(from_response.headers)
            .with_body(from_response.body.unwrap_or_default())
    }
}

impl Debug for Response {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "{} {}\n{}",
            HttpVersion::default(),
            self.status,
            self.headers
        )
    }
}

impl fmt::Display for Response {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, formatter)
    }
}

// Allow plugins to attach to responses.
impl Extensible<TypeMapInner> for Response {
    fn extensions(&self) -> &TypeMap<TypeMapInner> {
        &self.extensions
    }

    fn extensions_mut(&mut self) -> &mut TypeMap<TypeMapInner> {
        &mut self.extensions
    }
}

impl Plugin for Response {}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::header::{ContentType};
    use futures::stream::Stream;
    use futures::{future, Future};
    use mime;
    use std::str::from_utf8;

    #[test]
    fn test_create_response() {
        let response = Response::new();

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.headers, Headers::new());
        assert!(response.body.is_none());
    }

    #[test]
    fn test_response_from_hyper_response() {
        let mut headers = Headers::new();
        headers.set(ContentType(mime::TEXT_HTML));

        let response = Response::from(
            HyperResponse::new()
                .with_status(StatusCode::NotFound)
                .with_headers(headers.clone())
                .with_body("Error")
        );

        assert_eq!(response.status, StatusCode::NotFound);
        assert_eq!(response.headers, headers);
        assert!(response.body.is_some());

        let body = response.body.unwrap()
            .concat2()
            .and_then(|chunk| {
                future::ok(String::from(from_utf8(&chunk).unwrap()))
            })
            .wait().unwrap();

        assert_eq!(body, "Error");
    }

    #[test]
    fn test_hyper_response_from_response() {
        let mut headers = Headers::new();
        headers.set(ContentType(mime::TEXT_HTML));

        let response = HyperResponse::from(
            Response {
                status: StatusCode::NotFound,
                headers: headers.clone(),
                body: Some("Error".into()),
                extensions: TypeMap::custom()
            }
        );

        assert_eq!(response.status(), StatusCode::NotFound);
        assert_eq!(response.headers(), &headers);
        assert!(response.body_ref().is_some());

        let body = response.body()
            .concat2()
            .and_then(|chunk| {
                future::ok(String::from(from_utf8(&chunk).unwrap()))
            })
            .wait().unwrap();

        assert_eq!(body, "Error");
    }
}