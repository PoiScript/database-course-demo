#[macro_use]
extern crate nom;
extern crate hyper;
extern crate chrono;
extern crate futures;
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

fn main() {
  let addr = "0.0.0.0:3000".parse().unwrap();
  let server = hyper::server::Http::new().bind(&addr, || {
    let db = database::Database::new("postgres://poi@localhost:5432/store");
    Ok(service::ApiService::new(db))
  }).unwrap();
  server.run().unwrap();
}
