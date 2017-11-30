extern crate ferrum;

use ferrum::*;

fn main() {
    Ferrum::new(|_: &mut Request| {
        Ok(Response::new().with_content("Hello world!", mime::TEXT_PLAIN))
    }).http("localhost:3000").unwrap();
}
