//! Ferrum's HTTP Request representation and associated methods.

use std::mem;
use std::net::SocketAddr;
use std::fmt::{self, Debug};

use hyper::{Body, HttpVersion, Uri};

use typemap::{TypeMap, TypeMapInner};
use plugin::Extensible;

pub use hyper::server::Request as HyperRequest;

use {Plugin, Headers, Method};

pub mod uri;
pub use self::uri::*;

/// The `Request` given to all `Middleware`.
///
/// Stores all the properties of the client's request plus
/// an `TypeMap` for data communication between middleware.
pub struct Request {
    /// The requested URI.
    pub uri: Uri,

    /// The URI path segments collection
    pub uri_path_segments: Vec<String>,

    /// The request method.
    pub method: Method,

    /// The version of the HTTP protocol used.
    pub version: HttpVersion,

    /// The originating address of the request. Some underlying transports
    /// may not have a socket address, such as Unix Sockets.
    pub remote_addr: Option<SocketAddr>,

    /// The request headers.
    pub headers: Headers,

    /// The request body.
    pub body: Option<Body>,

    /// Extensible storage for data passed between middleware.
    pub extensions: TypeMap<TypeMapInner>,

    _p: (),
}

impl Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Request {{")?;
        writeln!(f, "    uri: {:?}", self.uri)?;
        writeln!(f, "    method: {:?}", self.method)?;
        writeln!(f, "    version: {:?}", self.version)?;
        writeln!(f, "    remote_addr: {:?}", self.remote_addr)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl Request {
    /// Create a request from an HyperRequest.
    ///
    /// This constructor consumes the HyperRequest.
    pub fn new(request: HyperRequest) -> Request {
        #[allow(deprecated)]
        let remote_addr = request.remote_addr();
        let (method, uri, version, headers, body) = request.deconstruct();

        let uri_path_segments = uri.decoded_path_segments();

        Request {
            uri,
            uri_path_segments,
            method,
            version,
            remote_addr,
            headers,
            body: Some(body),
            extensions: TypeMap::custom(),
            _p: (),
        }
    }

    pub fn take_body(&mut self) -> Body {
        let body = mem::replace(&mut self.body, None);
        body.unwrap_or_default()
    }

    #[cfg(test)]
    pub fn stub() -> Request {
        use std::net::ToSocketAddrs;
        use std::str::FromStr;

        let uri = Uri::from_str("http://www.rust-lang.org").unwrap();
        let uri_path_segments = uri.decoded_path_segments();

        Request {
            uri,
            uri_path_segments,
            method: Method::Get,
            version: HttpVersion::Http11,
            remote_addr: Some("localhost:3000".to_socket_addrs().unwrap().next().unwrap()),
            headers: Headers::new(),
            body: None,
            extensions: TypeMap::custom(),
            _p: (),
        }
    }
}

// Allow plugins to attach to requests.
impl Extensible<TypeMapInner> for Request {
    fn extensions(&self) -> &TypeMap<TypeMapInner> {
        &self.extensions
    }

    fn extensions_mut(&mut self) -> &mut TypeMap<TypeMapInner> {
        &mut self.extensions
    }
}

impl Plugin for Request {}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::ToSocketAddrs;
    use std::str::FromStr;

    #[test]
    fn test_create_request() {
        let uri = Uri::from_str("http://www.rust-lang.org/foo/bar").unwrap();
        let request = Request::new(
            HyperRequest::new(Method::Get, uri.clone())
        );

        assert_eq!(request.uri, uri);
        assert_eq!(request.uri_path_segments, vec!["foo".to_string(), "bar".to_string()]);
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.version, HttpVersion::default());
        assert_eq!(request.remote_addr, None);
        assert_eq!(request.headers, Headers::new());
    }

    #[test]
    fn test_create_request_stub() {
        let uri = Uri::from_str("http://www.rust-lang.org").unwrap();
        let addr = "127.0.0.1:3000".to_socket_addrs().unwrap().next().unwrap();
        let request = Request::stub();

        assert_eq!(request.uri, uri);
        assert_eq!(request.uri_path_segments, vec!["".to_string()]);
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.version, HttpVersion::default());
        assert_eq!(request.remote_addr.unwrap(), addr);
        assert_eq!(request.headers, Headers::new());
    }
}