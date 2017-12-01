extern crate ferrum;

use ferrum::*;

fn main() {
    Ferrum::new(|_: &mut Request| {
        Ok(Response::new().with_status(StatusCode::NotFound))
    }).http("localhost:3000").unwrap();
}