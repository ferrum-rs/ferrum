// Stdlib dependencies
#[macro_use] extern crate log;

// Third party packages
extern crate hyper;
extern crate futures;
extern crate typemap as tmap;
extern crate plugin;
extern crate num_cpus;
extern crate mime_guess;

#[cfg(test)]
extern crate mime;

// Request + Response
pub use request::Request;
//pub use response::Response;

// Middleware system
//pub use middleware::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler, Chain};

// Server
//pub use ferrum::*;

// Extensions
pub use typemap::TypeMap;

// Headers
pub use hyper::header;
pub use hyper::header::Headers;

// Expose `Pluggable` as `Plugin` so users can do `use iron::Plugin`.
pub use plugin::Pluggable as Plugin;

// Expose modifiers.
//pub use modifier::Set;

/// Re-exports from the `TypeMap` crate.
pub mod typemap {
    pub use tmap::{TypeMap, Key};
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

// Request utilities
pub mod request;

// Response utilities
pub mod response;