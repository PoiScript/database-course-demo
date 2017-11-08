use postgres::{Connection, TlsMode};

pub struct Database {
  conn: Connection
}

impl Database {
  pub fn new(url: &str) -> Database {
    Database { conn: Connection::connect(url, TlsMode::None).unwrap() }
  }
}
