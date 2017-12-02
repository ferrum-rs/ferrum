extern crate ferrum;

use std::env;

use ferrum::*;
use ferrum::header::{ContentLength, ContentType};

// All these variants do the same thing, with more or less options for customization.

fn variant1(_: &mut Request) -> FerrumResult<Response> {
    Ok(Response::new().with_content("{}", mime::APPLICATION_JSON))
}

fn variant2(_: &mut Request) -> FerrumResult<Response> {
    let content_type: mime::Mime = "application/json".parse().unwrap();
    Ok(Response::new().with_content("{}", content_type))
}

fn variant3(_: &mut Request) -> FerrumResult<Response> {
    Ok(
        Response::new()
            .with_status(StatusCode::Ok)
            .with_body("{}")
            .with_header(ContentLength("{}".len() as u64))
            .with_header(ContentType::json())
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let variant_index = if args.len() > 1 { args[1].parse().unwrap() } else { 1 };
    let handler = match variant_index {
        1 => variant1,
        2 => variant2,
        3 => variant3,
        _ => panic!("No such variant"),
    };
    println!("Using variant{}", variant_index);
    Ferrum::new(handler).http("localhost:3000").unwrap();
}
