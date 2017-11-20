use postgres::Connection;
use postgres::rows::Row;

use error::Error;

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "role")]
pub enum Role {
  Manager,
  Leader
}

#[derive(Serialize)]
pub struct User {
  username: String,
  password: String,
  user_role: Role
}

impl User {
  fn from_row(row: Row) -> User {
    User { username: row.get(0), password: row.get(1), user_role: row.get(2) }
  }

  pub fn login(conn: &Connection, username: String, password: String) -> Result<User, Error> {
    let rows = &conn.query(
      "SELECT * FROM StoreUser WHERE username = $1 AND password = $2",
      &[&username, &password]
    )?;
    if (rows.is_empty()) {
      Err(Error::NotFound)
    } else {
      Ok(User::from_row(rows.get(0)))
    }
  }
}
