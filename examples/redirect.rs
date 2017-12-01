extern crate ferrum;

use ferrum::*;

fn main() {
    Ferrum::new(move |_: &mut Request | {
        Ok(Response::new_redirect("http://rust-lang.org"))
    }).http("localhost:3000").unwrap();
}

