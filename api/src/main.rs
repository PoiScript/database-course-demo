extern crate hyper;
extern crate chrono;
extern crate futures;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;

mod service;
mod database;
mod types;
mod error;

use std::rc::Rc;
use hyper::server::Http;

use database::Database;
use service::ApiService;

fn main() {
  env_logger::init().expect("error/env-logger");

  let addr = "0.0.0.0:3000".parse().unwrap();
  let server = Http::new().bind(&addr, || {
    let db = Rc::new(Database::new("postgres://poi@localhost:5432/store"));
    Ok(ApiService::new(db))
  }).unwrap();
  server.run().unwrap();
}
