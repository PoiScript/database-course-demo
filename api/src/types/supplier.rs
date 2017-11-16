use postgres::Connection;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Supplier {
  id: i32,
  zip: i32,
  tele: i32,
  name: String,
  contact: String,
  address: String
}

impl Supplier {
  fn new(id: i32, zip: i32, tele: i32, name: String, contact: String, address: String) -> Supplier {
    Supplier { id, zip, tele, name, contact, address }
  }
}

impl Query for Supplier {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO supplier (id, zip, tele, name, contact, address) VALUES ($1, $2, $3, $4, $5, $6)",
      &[&self.id, &self.zip, &self.tele, &self.name, &self.contact, &self.address]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, zip, tele, name, contact, address FROM supplier", &[])? {
      vec.push(Supplier::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4),
        row.get(5)
      ));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT zip, tele, contact, address FROM supplier WHERE id = $1", &[&id])?;
    let first_row = rows.get(0);
    Ok(Supplier::new(
      first_row.get(0),
      first_row.get(1),
      first_row.get(2),
      first_row.get(3),
      first_row.get(4),
      first_row.get(5)
    ))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE supplier SET zip = $1, tele = $2, name = $3, contact = $4, address = $5 WHERE id = $6",
      &[&self.zip, &self.tele, &self.name, &self.contact, &self.address, &self.id]
    )?;
    Ok(())
  }
}
