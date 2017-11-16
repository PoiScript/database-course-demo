use postgres::Connection;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct SuppliedGoods {
  id: i32,
  original_cost: f32,
  goods_id: i32,
  supplier_id: i32
}

impl SuppliedGoods {
  fn new(id: i32, original_cost: f32, goods_id: i32, supplier_id: i32) -> SuppliedGoods {
    SuppliedGoods { id, original_cost, goods_id, supplier_id }
  }
}

impl Query for SuppliedGoods {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO supplied_goods (id, original_cost, goods_id, supplier_id) VALUES ($1, $2, $3, $4)",
      &[&self.id, &self.original_cost, &self.goods_id, &self.supplier_id]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, original_cost, goods_id, supplier_id FROM supplied_goods", &[])? {
      vec.push(SuppliedGoods::new(row.get(0), row.get(1), row.get(2), row.get(3)));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, original_cost, goods_id, supplier_id FROM supplied_goods WHERE id = $1", &[&id])?;
    let first_row = rows.get(0);
    Ok(SuppliedGoods::new(
      first_row.get(0),
      first_row.get(1),
      first_row.get(2),
      first_row.get(3),
    ))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE supplied_goods SET original_cost = $1 goods_id = $2 supplier_id = $3 WHERE id = $4",
      &[&self.original_cost, &self.goods_id, &self.supplier_id, &self.id]
    )?;
    Ok(())
  }
}
