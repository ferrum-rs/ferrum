use std::fmt;

use {Response};

pub use std::error::Error;
pub use hyper::Error as HyperError;
pub use hyper::error::Result as HyperResult;

/// The type of Errors inside and when using Ferrum.
///
/// `FerrumError` informs its receivers of two things:
///
/// * What went wrong
/// * What to do about it
///
/// The `error` field is responsible for informing receivers of which
/// error occured, and receivers may also modify the error field by layering
/// it (building up a cause chain).
///
/// The `response` field provides a tangible action to be taken if this error
/// is not otherwise handled.
#[derive(Debug)]
pub struct FerrumError {
    /// The underlying error
    ///
    /// This can be layered and will be logged at the end of an errored request.
    pub error: Box<Error + Send>,

    /// What to do about this error.
    ///
    /// This Response will be used when the error-handling flow finishes.
    pub response: Response
}

impl FerrumError {
    /// Create a new `FerrumError` from an error and a response.
    pub fn new<E: 'static + Error + Send>(err: E, response: Response) -> FerrumError {
        FerrumError {
            error: Box::new(err),
            response
        }
    }
}

impl fmt::Display for FerrumError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&*self.error, formatter)
    }
}

impl Error for FerrumError {
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&Error> {
        self.error.cause()
    }
}

