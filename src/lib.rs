// Stdlib dependencies
#[macro_use] extern crate log;

// Third party packages
extern crate hyper;
extern crate futures;
extern crate futures_cpupool;
extern crate typemap as tmap;
extern crate unsafe_any as uany;
extern crate plugin;
extern crate num_cpus;
extern crate mime_guess;

#[cfg(test)]
extern crate mime;

// Request + Response
pub use request::Request;
pub use response::Response;

// Middleware system
//pub use middleware::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler, Chain};

// Server
pub use ferrum::*;

// Extensions
pub use typemap::TypeMap;

// Headers
pub use hyper::header;
pub use hyper::header::Headers;
pub use hyper::header::Header;

// Expose `Pluggable` as `Plugin` so users can do `use iron::Plugin`.
pub use plugin::Pluggable as Plugin;

// Errors
pub use error::Error;
pub use error::FerrumError;

/// Ferrum's error type and associated utilities.
pub mod error;

/// The Result alias used throughout Iron and in clients of Iron.
pub type FerrumResult<T> = Result<T, FerrumError>;

/// Re-exports from the `TypeMap` crate.
pub mod typemap {
    pub use plugin::typemap::{TypeMap, Key};
    pub use uany::UnsafeAny;
    pub type TypeMapInner = UnsafeAny + Send + Sync;
}

/// Status Codes
pub mod status {
    pub use hyper::StatusCode;
    pub use hyper::StatusCode::*;
}

/// HTTP Methods
pub mod method {
    pub use hyper::Method;
    pub use hyper::Method::*;
}

// Publicized to show the documentation
pub mod middleware;

// Request utilities
pub mod request;

// Response utilities
pub mod response;

mod service;

mod ferrum;