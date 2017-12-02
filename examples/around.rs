extern crate time;
extern crate ferrum;

use std::thread;

use ferrum::*;

enum LoggerMode {
    Silent,
    Tiny,
    Large
}

struct Logger {
    mode: LoggerMode
}

struct LoggerHandler<H: Handler> {
    logger: Logger, handler: H
}

impl Logger {
    fn new(mode: LoggerMode) -> Logger {
        Logger { mode }
    }

    fn log(&self, request: &Request, response: Result<&Response, &FerrumError>, time: u64) {
        match self.mode {
            LoggerMode::Silent => {},
            LoggerMode::Tiny => println!("-> {:?}\n<- {:?}\n{}", request, response, time),
            LoggerMode::Large => println!("Request: {:?}\nResponse: {:?}\nResponse-Time: {}", request, response, time)
        }
    }
}

impl<H: Handler> Handler for LoggerHandler<H> {
    fn handle(&self, request: &mut Request) -> FerrumResult<Response> {
        let entry = time::precise_time_ns();
        let response = self.handler.handle(request);

        self.logger.log(request, response.as_ref(), time::precise_time_ns() - entry);
        response
    }
}

impl AroundMiddleware for Logger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(LoggerHandler {
            logger: self,
            handler
        }) as Box<Handler>
    }
}

fn hello_world(_: &mut Request) -> FerrumResult<Response> {
    Ok(Response::new().with_content("Hello World!", mime::TEXT_PLAIN))
}

fn main() {
    println!("Servers listening on 2000, 3000, and 4000");

    thread::spawn(|| {
        Ferrum::new(
            Logger::new(LoggerMode::Silent).around(Box::new(hello_world))
        ).http("localhost:2000").unwrap();
    });

    thread::spawn(|| {
        Ferrum::new(
            Logger::new(LoggerMode::Tiny).around(Box::new(hello_world))
        ).http("localhost:3000").unwrap();
    });

    Ferrum::new(
        Logger::new(LoggerMode::Large).around(Box::new(hello_world))
    ).http("localhost:4000").unwrap();
}
