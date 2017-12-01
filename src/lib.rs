#[macro_use] extern crate log;
pub extern crate hyper;
pub extern crate futures;
extern crate futures_cpupool;
extern crate typemap as tmap;
extern crate unsafe_any as uany;
extern crate plugin;
extern crate num_cpus;
extern crate mime_guess;
pub extern crate mime;

/// Request + Response
pub use request::Request;
pub use response::Response;

/// Middleware system
pub use middleware::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler, Chain};

/// Server
pub use ferrum::*;

/// Extensions
pub use typemap::TypeMap;

/// Headers
pub use hyper::header;
pub use hyper::header::Headers;
pub use hyper::header::Header;

/// Status Codes
pub use hyper::{Method, StatusCode, Uri};

/// Expose `Pluggable` as `Plugin` so users can do `use iron::Plugin`.
pub use plugin::Pluggable as Plugin;

/// Errors
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

// Publicized to show the documentation
pub mod middleware;

/// Request utilities
pub mod request;

/// Response utilities
pub mod response;

pub mod content;

mod service;

mod ferrum;