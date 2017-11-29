extern crate ferrum;
extern crate env_logger;

use ferrum::*;

fn main() {
    env_logger::init().unwrap();

    Ferrum::new(|_: &mut Request| {
        Ok(Response::new().with_status(StatusCode::NotFound))
    }).http("127.0.0.1:3000").unwrap();
}