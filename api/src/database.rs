use futures;
use futures::Future;

use postgres::{Connection, TlsMode};

use serde::Serialize;
use serde_json::to_string;

use error::Error;
use types::*;

pub struct Database {
  conn: Connection
}

impl Database {
  pub fn new(url: &str) -> Database {
    Database { conn: Connection::connect(url, TlsMode::None).unwrap() }
  }

  pub fn create<T: Query>(&self, obj: T) -> Box<Future<Item=(), Error=Error>> {
    Box::new(futures::done(obj.create(&self.conn)))
  }

  pub fn get_by_id<T: Query + 'static>(&self, id: i32) -> Box<Future<Item=String, Error=Error>> {
    Box::new(futures::done(T::get_by_id(&self.conn, id)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
    ))
  }

  pub fn get_all<T: Query + 'static>(&self) -> Box<Future<Item=String, Error=Error>> {
    Box::new(futures::done(T::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
    ))
  }

  pub fn delete_by_id<T: Query + 'static>(&self, id: i32) -> Box<Future<Item=(), Error=Error>> {
    Box::new(futures::done(T::delete_by_id(&self.conn, id)))
  }

  pub fn join_purchase(&self) -> Box<Future<Item=String, Error=Error>> {
    Box::new(futures::done(JoinedPurchase::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
    ))
  }

  pub fn join_supplied_goods(&self) -> Box<Future<Item=String, Error=Error>> {
    Box::new(futures::done(JoinedSuppliedGoods::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
    ))
  }

  pub fn join_receipt(&self) -> Box<Future<Item=String, Error=Error>> {
    Box::new(futures::done(JoinedReceipt::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
    ))
  }
}

pub trait Query where Self: Sized + Serialize {
  fn create(&self, conn: &Connection) -> Result<(), Error>;
  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error>;
  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error>;
  fn update(&self, conn: &Connection) -> Result<(), Error>;
  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error>;
}
