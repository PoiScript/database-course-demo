use postgres::{Connection, TlsMode};

pub struct Database {
  conn: Connection
}

impl Database {
  pub fn new(url: &str) -> Database {
    Database { conn: Connection::connect(url, TlsMode::None).unwrap() }
  }

  pub fn create<T: Query>(&self, obj: T) {
    obj.create(&self.conn)
  }

  pub fn get_by_id<T: Query>(&self, id: i32) -> T {
    T::get_by_id(&self.conn, id)
  }

  pub fn get_all<T: Query>(&self) -> Vec<T> {
    T::get_all(&self.conn)
  }
}

pub trait Query where Self: Sized {
  fn create(&self, conn: &Connection);
  fn get_by_id(conn: &Connection, id: i32) -> Self;
  fn get_all(conn: &Connection) -> Vec<Self>;
  fn update(&self, conn: &Connection);
}
