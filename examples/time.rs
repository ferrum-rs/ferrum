extern crate time;
extern crate ferrum;

use ferrum::*;
use time::precise_time_ns;

struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, request: &mut Request) -> FerrumResult<()> {
        request.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, request: &mut Request, response: Response) -> FerrumResult<Response> {
        let delta = precise_time_ns() - *request.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(response)
    }
}

fn hello_world(_: &mut Request) -> FerrumResult<Response> {
    Ok(Response::new().with_content("Hello World", mime::TEXT_PLAIN))
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Ferrum::new(chain).http("localhost:3000").unwrap();
}
