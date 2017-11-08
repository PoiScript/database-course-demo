#[macro_use]
extern crate nom;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate postgres;
extern crate hyper;

mod service;
mod database;
mod types;

fn main() {
  print!("hello world");
}
