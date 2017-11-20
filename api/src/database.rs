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

  pub fn create<T: Query>(&self, obj: T) -> Result<(), Error> {
    T::create(&self.conn, obj)
  }

  pub fn get_all<T: Query + 'static>(&self) -> Result<String, Error> {
    T::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
  }

  pub fn update<T: Query>(&self, obj: T) -> Result<(), Error> {
    T::update(&self.conn, obj)
  }

  pub fn delete_by_id<T: Query + 'static>(&self, id: i32) -> Result<(), Error> {
    T::delete_by_id(&self.conn, id)
  }

  pub fn join_purchase(&self) -> Result<String, Error> {
    JoinedPurchase::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
  }

  pub fn join_supplied_goods(&self) -> Result<String, Error> {
    JoinedSuppliedGoods::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
  }

  pub fn join_receipt(&self) -> Result<String, Error> {
    JoinedReceipt::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
  }

  pub fn join_staff(&self) -> Result<String, Error> {
    JoinedStaff::get_all(&self.conn)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
  }

  pub fn login(&self, req: LoginRequest) -> Result<String, Error> {
    User::login(&self.conn, req.username, req.password)
      .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
  }
}

pub trait Query where Self: Sized + Serialize {
  fn create(conn: &Connection, obj: Self) -> Result<(), Error>;
  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error>;
  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error>;
  fn update(conn: &Connection, obj: Self) -> Result<(), Error>;
  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error>;
}
