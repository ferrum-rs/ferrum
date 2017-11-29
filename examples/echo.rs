// An example that echoes the body of the request back as the response.
//
// Shows how to read the request body with error handling and how to return a
// response. See `helper_macros` example for a different way to handle errors.

extern crate futures;
extern crate ferrum;

use futures::Future;
use futures::stream::Stream;

use ferrum::*;

fn echo(request: &mut Request) -> FerrumResult<Response> {
    Ok(match request.method {

        Method::Post => {
            let body = request.take_body().concat2().wait().unwrap();
            Response::new()
                .with_status(StatusCode::Ok)
                .with_body(body)
        },

        _ => Response::new()
            .with_status(StatusCode::Ok)
            .with_body("Try POSTing data"),
    })
}

fn main() {
    Ferrum::new(echo).http("127.0.0.1:3000").unwrap();
}
