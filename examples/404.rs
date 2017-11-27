extern crate ferrum;

//use ferrum::prelude::*;
//use ferrum::status;
use ferrum::*;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    Ferrum::new(|_: &mut Request| {
        Ok(Response::new().with_status(status::NotFound))
    }).http(&addr).unwrap();
}