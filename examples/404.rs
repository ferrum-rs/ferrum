extern crate ferrum;
extern crate env_logger;

use ferrum::*;

fn main() {
    env_logger::init().unwrap();

    let addr = "127.0.0.1:3000".parse().unwrap();
    Ferrum::new(|_: &mut Request| {
        Ok(Response::new().with_status(status::NotFound))
    }).http(&addr).unwrap();
}