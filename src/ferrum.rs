//! Exposes the `Ferrum` type, the main entrance point of the `Ferrum` library.

use std::net::ToSocketAddrs;
use std::time::Duration;
use std::io::{Error, ErrorKind};

use hyper::Body;
use hyper::server::{Http, Server as HyperServer};

use error::HyperResult;
use service::InitialService;
use middleware::Handler;

pub type Server<H> = HyperServer<InitialService<H>, Body>;

/// The primary entrance point to `Ferrum`, a `struct` to instantiate a new server.
///
/// `Ferrum` contains the `Handler` which takes a `Request` and produces a `Response`.
pub struct Ferrum<H>
    where H: Handler
{
    /// Ferrum contains a `Handler`, which it uses to create responses for client requests.
    pub handler: H,

    /// Controls the timeout for keep alive connections.
    ///
    /// The default is `true`.
    pub keep_alive: bool,

    /// Server timeout.
    pub timeout: Option<Duration>,

    /// The number of request handling threads.
    ///
    /// Defaults to `num_cpus`.
    pub num_threads: usize,
}

impl<H> Ferrum<H>
    where H: Handler
{
    /// Instantiate a new instance of `Ferrum`.
    ///
    /// This will create a new `Ferrum`, the base unit of the server, using the
    /// passed in `Handler`.
    pub fn new(handler: H) -> Ferrum<H> {
        Ferrum {
            handler,
            keep_alive: true,
            timeout: Some(Duration::from_secs(30)),
            num_threads: ::num_cpus::get(),
        }
    }

    /// Kick off the server process using the HTTP protocol.
    ///
    /// Call this once to begin listening for requests on the server.
    /// This consumes the Ferrum instance. This method will block
    /// the current thread executing the HTTP server.
    pub fn http<A>(self, addr: A) -> HyperResult<()>
        where A: ToSocketAddrs
    {
        let server = self.server(addr)?;//server.bind(&addr, InitialService::new(self.handler, Some(self.num_threads)))?;
        server.run()
    }

    /// Bind the provided `addr` and return a server ready to handle
    /// connections.
    pub fn server<A>(self, addr: A) -> HyperResult<Server<H>>
        where A: ToSocketAddrs
    {
        let addr = addr.to_socket_addrs()?
            .next()
            .ok_or(Error::new(ErrorKind::Other, "Empty addrs"))?;

        let mut server = Http::new();
        server.keep_alive(self.keep_alive);
        server.bind(&addr, InitialService::new(self.handler, Some(self.num_threads)))
    }
}
