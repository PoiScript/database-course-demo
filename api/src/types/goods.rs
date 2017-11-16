use postgres::Connection;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Goods {
  id: i32,
  name: String,
  sale_price: f32
}

impl Goods {
  fn new(id: i32, name: String, sale_price: f32) -> Goods {
    Goods { id, name, sale_price }
  }
}

impl Query for Goods {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO goods (id, name, sale_price) VALUES ($1, $2, $3)",
      &[&self.id, &self.name, &self.sale_price]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, sale_price FROM goods", &[])? {
      vec.push(Goods::new(row.get(0), row.get(1), row.get(2)));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, name, sale_price FROM goods WHERE id = $1", &[&id])?;
    let first_row = rows.get(0);
    Ok(Goods::new(
      first_row.get(0),
      first_row.get(1),
      first_row.get(2)
    ))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE goods SET name = $1, sale_price = $2 WHERE id = $3",
      &[&self.name, &self.sale_price, &self.id]
    )?;
    Ok(())
  }
}
