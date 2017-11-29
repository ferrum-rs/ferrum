extern crate ferrum;
extern crate time;

use std::error::Error;
use std::fmt::{self, Debug};

use ferrum::*;

struct ErrorHandler;
struct ErrorProducer;

#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, formatter)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &*self.0
    }
}

impl Handler for ErrorHandler {
    fn handle(&self, _: &mut Request) -> FerrumResult<Response> {
        // This is never called!
        //
        // If a BeforeMiddleware returns an error through Err(...),
        // and it is not handled by a subsequent BeforeMiddleware in
        // the chain, the main handler is not invoked.
        Ok(Response::new())
    }
}

impl BeforeMiddleware for ErrorProducer {
    fn before(&self, _: &mut Request) -> FerrumResult<()> {
        Err(FerrumError::new(
            StringError("Error".to_string()),
            Some(Response::new().with_status(StatusCode::BadRequest))
        ))
    }
}

fn main() {
    // Handler is attached here.
    let mut chain = Chain::new(ErrorHandler);

    // Link our error maker.
    chain.link_before(ErrorProducer);

    Ferrum::new(chain).http("localhost:3000").unwrap();
}
