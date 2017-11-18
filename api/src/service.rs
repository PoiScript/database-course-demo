use futures::future;
use futures::future::Future;

use hyper;
use hyper::{Method, StatusCode};
use hyper::server::{Request, Response, Service};

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
      (&Method::Get, path) => {
        Box::new(
          (match path {
            "/api/goods" => self.db.join_supplied_goods(),
            "/api/customer" => self.db.get_all::<Customer>(),
            "/api/purchase" => self.db.join_purchase(),
            "/api/receipt" => self.db.join_receipt(),
            "/api/staff" => self.db.get_all::<Staff>(),
            _ => Box::new(future::ok(r#"{"error":{"code":"error/not-found"}}"#.to_string()))
          })
            .or_else(|e| {
              error!("{:?}", e);
              Ok(e.json())
            })
            .and_then(|s| Ok(Response::new().with_body(s)))
        )
      }
      _ => {
        Box::new(future::ok(
          Response::new().with_status(StatusCode::NotFound)
        ))
      }
    }
  }
}
