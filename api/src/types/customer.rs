use postgres::Connection;
use postgres::rows::Row;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Customer {
  id: i32,
  zip: i32,
  tele: i32,
  name: String,
  contact: String,
  address: String
}

impl Customer {
  pub fn new(id: i32, zip: i32, tele: i32, name: String, contact: String, address: String) -> Customer {
    Customer { id, zip, tele, name, contact, address }
  }

  fn from_row(row: Row) -> Customer {
    Customer::new(
      row.get(0),
      row.get(1),
      row.get(2),
      row.get(3),
      row.get(4),
      row.get(5),
    )
  }
}

impl Query for Customer {
  fn create(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO customer (id, zip, tele, name, contact, address) VALUES ($1, $2, $3, $4, $5, $6)",
      &[&obj.id, &obj.zip, &obj.tele, &obj.name, &obj.contact, &obj.address]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, zip, tele, name, contact, address FROM customer", &[])? {
      vec.push(Customer::from_row(row));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, name, sale_price FROM customer WHERE id = $1", &[&id])?;
    Ok(Customer::from_row(rows.get(0)))
  }

  fn update(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "UPDATE customer SET zip = $1, tele = $2, name = $3, contact = $4, address = $5 WHERE id = $6",
      &[&obj.zip, &obj.tele, &obj.name, &obj.contact, &obj.address, &obj.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM customer WHERE id = $1", &[&id])?;
    Ok(())
  }
}
