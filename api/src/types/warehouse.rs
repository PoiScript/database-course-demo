use postgres::Connection;
use postgres::rows::Row;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Warehouse {
  id: i32,
  name: String,
  address: String,
  responsible_staff: i32
}

impl Warehouse {
  pub fn new(id: i32, name: String, address: String, responsible_staff: i32) -> Warehouse {
    Warehouse { id, name, address, responsible_staff }
  }

  fn from_row(row: Row) -> Warehouse {
    Warehouse::new(
      row.get(0),
      row.get(1),
      row.get(2),
      row.get(3)
    )
  }
}

impl Query for Warehouse {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO warehouse (id, name, address, responsible_staff) VALUES ($1, $2, $3, $4)",
      &[&self.id, &self.name, &self.address, &self.responsible_staff]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, address, responsible_staff FROM warehouse", &[])? {
      vec.push(Warehouse::from_row(row));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, name, address, responsible_staff FROM warehouse WHERE id = $1", &[&id])?;
    Ok(Warehouse::from_row(rows.get(0)))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE warehouse SET name = $1, address = $2, responsible_staff = $3 WHERE id = $4",
      &[&self.name, &self.address, &self.responsible_staff, &self.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM warehouse WHERE id = $1", &[&id])?;
    Ok(())
  }
}
