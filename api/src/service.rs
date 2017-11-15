use futures;
use futures::future::Future;

use hyper;
use hyper::{Method, StatusCode};
use hyper::server::{Request, Response, Service};

use serde_json::to_string;

use database::Database;
use types::*;

pub struct ApiService {
  db: Database
}

impl ApiService {
  pub fn new(db: Database) -> ApiService {
    ApiService { db }
  }
}

impl Service for ApiService {
  // hook up server's types
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    match (req.method(), req.path()) {
      (&Method::Get, "/api/goods") => {
        let all = &self.db.get_all::<Goods>();
        let json = to_string(all).unwrap();
        Box::new(futures::future::ok(
          Response::new().with_body(json)
        ))
      }
      (&Method::Get, "/api/staff") => {
        let all = &self.db.get_all::<Staff>();
        let json = to_string(all).unwrap();
        Box::new(futures::future::ok(
          Response::new().with_body(json)
        ))
      }
      (&Method::Get, "/api/warehouse") => {
        let all = &self.db.get_all::<Warehouse>();
        let json = to_string(all).unwrap();
        Box::new(futures::future::ok(
          Response::new().with_body(json)
        ))
      }
      (&Method::Get, "/api/receipt") => {
        let all = &self.db.get_all::<Receipt>();
        let json = to_string(all).unwrap();
        Box::new(futures::future::ok(
          Response::new().with_body(json)
        ))
      }
      _ => Box::new(futures::future::ok(
        Response::new().with_status(StatusCode::NotFound)
      ))
    }
  }
}
