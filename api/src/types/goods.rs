use postgres::Connection;
use postgres::rows::Row;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Goods {
  id: i32,
  name: String,
  sale_price: f32
}

impl Goods {
  pub fn new(id: i32, name: String, sale_price: f32) -> Goods {
    Goods { id, name, sale_price }
  }

  fn from_row(row: Row) -> Goods {
    Goods::new(
      row.get(0),
      row.get(1),
      row.get(2)
    )
  }
}

impl Query for Goods {
  fn create(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO goods (id, name, sale_price) VALUES ($1, $2, $3)",
      &[&obj.id, &obj.name, &obj.sale_price]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, sale_price FROM goods", &[])? {
      vec.push(Goods::from_row(row));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, name, sale_price FROM goods WHERE id = $1", &[&id])?;
    Ok(Goods::from_row(rows.get(0)))
  }

  fn update(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "UPDATE goods SET name = $1, sale_price = $2 WHERE id = $3",
      &[&obj.name, &obj.sale_price, &obj.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM goods WHERE id = $1", &[&id])?;
    Ok(())
  }
}
