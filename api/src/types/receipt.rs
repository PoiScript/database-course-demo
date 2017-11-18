use postgres::Connection;
use postgres::rows::Row;
use chrono::NaiveDateTime;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Receipt {
  id: i32,
  date: NaiveDateTime,
  is_resolved: bool,
  warehouse_id: i32,
  responsible_staff: i32,
  supplied_goods_id: i32,
  supplied_goods_count: i32,
}

impl Receipt {
  fn new(id: i32, date: NaiveDateTime, is_resolved: bool, warehouse_id: i32,
         responsible_staff: i32, supplied_goods_id: i32, supplied_goods_count: i32) -> Receipt {
    Receipt {
      id,
      date,
      is_resolved,
      warehouse_id,
      responsible_staff,
      supplied_goods_id,
      supplied_goods_count
    }
  }

  fn from_row(row: Row) -> Receipt {
    Receipt::new(
      row.get(0),
      row.get(1),
      row.get(2),
      row.get(3),
      row.get(4),
      row.get(5),
      row.get(6),
    )
  }
}

impl Query for Receipt {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO receipt (id, date, is_resolved, warehouse_id,
      responsible_staff, supplied_goods_id, supplied_goods_count)
      VALUES ($1, $2, $3, $4, $5, $6, $7)",
      &[
        &self.id,
        &self.date,
        &self.is_resolved,
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
      date, is_resolved, warehouse_id, responsible_staff,
      supplied_goods_id, supplied_goods_count FROM receipt", &[])? {
      vec.push(Receipt::from_row(row));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id,
      date, is_resolved, warehouse_id, responsible_staff,
      supplied_goods_id, supplied_goods_count FROM receipt WHERE id = $1", &[&id])?;
    Ok(Receipt::from_row(rows.get(0)))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE receipt date = $1
      is_resolved = $2 warehouse_id = $3 responsible_staff = $4
      supplied_goods_id = $5 supplied_goods_count = $6 WHERE id = $7",
      &[&self.date, &self.is_resolved, &self.warehouse_id, &self.responsible_staff,
        &self.supplied_goods_id, &self.supplied_goods_count, &self.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM receipt WHERE id = $1", &[&id])?;
    Ok(())
  }
}
