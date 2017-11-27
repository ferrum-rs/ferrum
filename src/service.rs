use std::io;
use std::sync::Arc;

use hyper::server::{NewService, Service};
use hyper::header::ContentLength;
use futures::{future, Future};
use futures_cpupool::{CpuPool, CpuFuture};

use request::{Request, HyperRequest};
use response::HyperResponse;
use error::{FerrumError, HyperError};
use middleware::Handler;
use status::StatusCode;

pub struct InitialService<H>
    where H: Handler
{
    pub handler: Arc<H>,
    pub thread_pool: Arc<CpuPool>,
}

impl<H> InitialService<H>
    where H: Handler
{
    pub fn new(handler: H, thread_pool_size: Option<usize>) -> InitialService<H> {
        let thread_pool = if let Some(size) = thread_pool_size {
            CpuPool::new(size)
        } else {
            CpuPool::new_num_cpus()
        };

        InitialService {
            handler: Arc::new(handler),
            thread_pool: Arc::new(thread_pool),
        }
    }
}

impl<H> Clone for InitialService<H>
    where H: Handler
{
    fn clone(&self) -> Self {
        InitialService {
            handler: self.handler.clone(),
            thread_pool: self.thread_pool.clone(),
        }
    }
}

impl<H> NewService for InitialService<H>
    where H: Handler
{
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = HyperError;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl<H> Service for InitialService<H>
    where H: Handler
{
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = HyperError;
    type Future = CpuFuture<Self::Response, Self::Error>;

    fn call(&self, request: Self::Request) -> Self::Future {
        let mut request = Request::new(request);
        let handler = self.handler.clone();

        self.thread_pool.spawn_fn(move || {
            let handle_result = match handler.handle(&mut request) {
                Ok(response) => Box::new(future::ok(response)),
                Err(err) => Box::new(future::err(err))
            };
            Box::new(handle_result
                .and_then(move |response| {
                    future::ok(HyperResponse::from(response))
                })
                .or_else(move |error| {
                    future::ok(HyperResponse::from(error))
                })
            )
        })
    }
}

impl From<FerrumError> for HyperResponse {
    fn from(error: FerrumError) -> HyperResponse {
        let error = format!("ERROR: {}", error);
        HyperResponse::new()
            .with_header(ContentLength(error.len() as u64))
            .with_body(error)
            .with_status(StatusCode::InternalServerError)
    }
}
