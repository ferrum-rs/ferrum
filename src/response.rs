//! Ferrum's HTTP Response representation and associated methods.

use std::fmt::{self, Debug};
use std::mem::replace;

use typemap::TypeMap;
use plugin::Extensible;
use hyper::{Body, HttpVersion};

use status::StatusCode;
use {Plugin, Headers};

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
    pub extensions: TypeMap,
}

impl Response {
    /// Construct a blank Response
    pub fn new() -> Response {
        Response {
            status: Default::default(),
            headers: Headers::new(),
            body: None, // Start with no body.
            extensions: TypeMap::new()
        }
    }
}

impl From<HyperResponse> for Response {
    fn from(mut from_response: HyperResponse) -> Response {
        Response {
            status: from_response.status(),
            headers: replace(from_response.headers_mut(), Headers::new()),
            body: if from_response.body_ref().is_some() { Some(from_response.body()) } else { None },
            extensions: TypeMap::new()
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
impl Extensible for Response {
    fn extensions(&self) -> &TypeMap {
        &self.extensions
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
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
                .with_body("Error".as_bytes().to_vec())
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
                body: Some("Error".as_bytes().to_vec().into()),
                extensions: TypeMap::new()
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