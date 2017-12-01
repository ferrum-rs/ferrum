extern crate ferrum;

use ferrum::*;
use ferrum::header::{ContentType, UserAgent};

struct DefaultContentType;

impl AfterMiddleware for DefaultContentType {
    // This is run for every requests, AFTER all handlers have been executed
    fn after(&self, _: &mut Request, mut response: Response) -> FerrumResult<Response> {
        if response.headers.get::<ContentType>() == None {
            // Set a standard header
            response.headers.set(ContentType(mime::TEXT_PLAIN));
        }
        Ok(response)
    }
}

fn info(request: &mut Request) -> FerrumResult<Response> {
    // Get a header using a standard ferrum headers
    let user_agent = match request.headers.get::<UserAgent>() {
        Some(user_agent) => format!("User Agent: {}\n", user_agent),
        None => "No User Agent\n".to_string(),
    };
    // Get a non-standard header using the raw header
    let x_forwarded_for = match request.headers.get_raw("X-Forwarded-For") {
        Some(proxies) => format!("Proxies: {}\n", std::str::from_utf8(&proxies[0]).unwrap()),
        None => "No proxy\n".to_string(),
    };
    let body = format!("{}{}\n", user_agent, x_forwarded_for);

    Ok(Response::new().with_body(body))
}

fn main() {
    let mut chain = Chain::new(info);
    chain.link_after(DefaultContentType);
    Ferrum::new(chain).http(("localhost", 3000)).unwrap();
}
