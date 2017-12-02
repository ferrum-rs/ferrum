Ferrum
======

> An Extensible, Concurrent Web Framework for Rust.

Ferrum is a fork of the [Iron](https://github.com/iron/iron).

## Hello World Example

```rust
extern crate ferrum;

use ferrum::*;

fn main() {
    Ferrum::new(|_: &mut Request| {
        Ok(Response::new().with_content("Hello world!", mime::TEXT_PLAIN))
    }).http("localhost:3000").unwrap();
}
```

## Response Timer Example

```rust
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
```

## Overview

Ferrum is a high level web framework built in and for Rust, built on
[hyper](https://github.com/hyperium/hyper). Ferrum is designed to take advantage
of Rust's greatest features - its excellent type system and its principled
approach to ownership in both single threaded and multi threaded contexts.

Ferrum is highly concurrent and can scale horizontally on more machines behind a
load balancer or by running more threads on a more powerful machine. Ferrum
avoids the bottlenecks encountered in highly concurrent code by avoiding shared
writes and locking in the core framework.

## Philosophy

Ferrum is meant to be as extensible and pluggable as possible; Ferrum's core is
concentrated and avoids unnecessary features by leaving them to middleware and
plugins.

Middleware and Plugins are the main ways to extend Ferrum with new functionality.
Most extensions that would be provided by middleware in other web frameworks are
instead addressed by the much simpler Plugin system.

Plugins allow for lazily-evaluated, automatically cached extensions to Requests
and Responses, perfect for parsing, accessing, and otherwise lazily manipulating
an http connection.

Middleware are only used when it is necessary to modify the control flow of a
Request flow, hijack the entire handling of a Request, check an incoming
Request, or to do final post-processing. This covers areas such as routing,
mounting, static asset serving, final template rendering, authentication, and
logging.

Ferrum comes with only basic functionality for setting the status, body, and
various headers, and the infrastructure for creating plugins and middleware.
No plugins or middleware are bundled with Ferrum.

## Underlying HTTP Implementation

Ferrum is based on and uses [`hyper`](https://github.com/hyperium/hyper) as its
HTTP implementation, and lifts several types from it, including its header
representation, status, and other core HTTP types. It is usually unnecessary to
use `hyper`, `futures` and `mime` directly when using Ferrum, since Ferrum provides
a facade over those core facilities, but it is sometimes necessary to depend on it
as well.

<!--
FIXME: expand on when it is necessary to user hyper for serving, e.g. when doing HTTPS.
-->

## Installation

If you're using `Cargo`, just add Ferrum to your `Cargo.toml`:

```toml
[dependencies.ferrum]
version = "*"
```

## [Examples](/examples)

Check out the [examples](/examples) directory!

You can run an individual example using `cargo run --example example-name`.
Note that for benchmarking you should make sure to use the `--release` flag,
which will cause cargo to compile the entire toolchain with optimizations.
Without `--release` you will get truly sad numbers.

## License

MIT
