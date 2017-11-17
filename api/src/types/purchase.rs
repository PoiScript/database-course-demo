use postgres::Connection;
use postgres::rows::Row;
use chrono::NaiveDateTime;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Purchase {
  id: i32,
  date: NaiveDateTime,
  is_resolved: i32,
  customer_id: i32,
  warehouse_id: i32,
  responsible_staff: i32,
  supplied_goods_id: i32,
  supplied_goods_count: i32,
}

impl Purchase {
  fn new(id: i32, date: NaiveDateTime, is_resolved: i32, customer_id: i32, warehouse_id: i32,
         responsible_staff: i32, supplied_goods_id: i32, supplied_goods_count: i32) -> Purchase {
    Purchase {
      id,
      date,
      is_resolved,
      customer_id,
      warehouse_id,
      responsible_staff,
      supplied_goods_id,
      supplied_goods_count
    }
  }

  fn from_row(row: Row) -> Purchase {
    Purchase::new(
      row.get(1),
      row.get(2),
      row.get(3),
      row.get(4),
      row.get(5),
      row.get(6),
      row.get(7),
      row.get(8)
    )
  }
}

impl Query for Purchase {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO purchase (id, date, is_resolved, customer_id, warehouse_id,
      responsible_staff, supplied_goods_id, supplied_goods_count)
      VALUES ($1, $2, $3, $4, $5, $6, $7)",
      &[
        &self.id,
        &self.date,
        &self.is_resolved,
        &self.customer_id,
        &self.warehouse_id,
        &self.responsible_staff,
        &self.supplied_goods_id,
        &self.supplied_goods_count
      ]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id,
      date, is_resolved, customer_id, warehouse_id, responsible_staff,
      supplied_goods_id, supplied_goods_count FROM purchase", &[])? {
      vec.push(Purchase::from_row(row));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id,
      date, is_resolved, customer_id, warehouse_id, responsible_staff,
      supplied_goods_id, supplied_goods_count FROM purchase WHERE id = $1", &[&id])?;
    Ok(Purchase::from_row(rows.get(0)))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE purchase date = $1 is_resolved = $2 customer_id = $3 warehouse_id = $4
      responsible_staff = $5 supplied_goods_id = $6 supplied_goods_count = $7 WHERE id = $8",
      &[&self.date, &self.is_resolved, &self.customer_id, &self.warehouse_id,
        &self.responsible_staff, &self.supplied_goods_id, &self.supplied_goods_count, &self.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM purchase WHERE id = $1", &[&id])?;
    Ok(())
  }
}
