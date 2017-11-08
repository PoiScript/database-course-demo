use futures;
use futures::future::Future;

use hyper;
use hyper::{Method, StatusCode};
use hyper::server::{Request, Response, Service};

use database::Database;

pub struct ApiService {
  db: Database
}

impl Service for ApiService {
  // hook up server's types
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let mut response = Response::new();

    match (req.method(), req.path()) {
      _ => response.set_status(StatusCode::NotFound)
    };

    Box::new(futures::future::ok(response))
  }
}
