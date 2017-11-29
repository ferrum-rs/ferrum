// This example shows how to create a basic router that maps url to different handlers.
// If you're looking for real routing middleware, check https://github.com/ferrum-rs/ferrum_router

extern crate ferrum;

use std::collections::HashMap;

use ferrum::*;

struct Router {
    // Routes here are simply matched with the uri path.
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new()
        }
    }

    fn add_route<H>(&mut self, path: &str, handler: H)
        where H: Handler
    {
        self.routes.insert(path.to_string(), Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, request: &mut Request) -> FerrumResult<Response> {
        match self.routes.get(request.uri.path()) {
            Some(handler) => handler.handle(request),
            None => Ok(Response::new().with_status(StatusCode::NotFound).with_body("Not found"))
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.add_route("/hello", |_: &mut Request| {
        Ok(Response::new().with_status(StatusCode::Ok).with_body("Hello world !"))
    });

    router.add_route("/hello/again", |_: &mut Request| {
       Ok(Response::new().with_status(StatusCode::Ok).with_body("Hello again !"))
    });

    router.add_route("/error", |_: &mut Request| {
       Ok(Response::new().with_status(StatusCode::BadRequest).with_body("Bad request"))
    });

    Ferrum::new(router).http("localhost:3000").unwrap();
}
