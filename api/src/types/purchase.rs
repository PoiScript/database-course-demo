use postgres::Connection;
use postgres::rows::Row;
use chrono::NaiveDateTime;

use database::Query;
use error::Error;

#[derive(Serialize, Deserialize)]
pub struct Purchase {
  id: i32,
  date: NaiveDateTime,
  is_resolved: bool,
  customer_id: i32,
  warehouse_id: i32,
  responsible_staff: i32,
  supplied_goods_id: i32,
  supplied_goods_count: i32,
}

impl Purchase {
  pub fn new(id: i32, date: NaiveDateTime, is_resolved: bool, customer_id: i32, warehouse_id: i32,
             responsible_staff: i32, supplied_goods_id: i32, supplied_goods_count: i32) -> Purchase {
    Purchase {
      id, date, is_resolved, customer_id, warehouse_id,
      responsible_staff, supplied_goods_id, supplied_goods_count
    }
  }

  fn from_row(row: Row) -> Purchase {
    Purchase::new(
      row.get(0), row.get(1), row.get(2), row.get(3),
      row.get(4), row.get(5), row.get(6), row.get(7)
    )
  }
}

impl Query for Purchase {
  fn create(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO purchase (id, date, is_resolved, customer_id, warehouse_id,
      responsible_staff, supplied_goods_id, supplied_goods_count)
      VALUES ($1, $2, $3, $4, $5, $6, $7)",
      &[
        &obj.id, &obj.date, &obj.is_resolved, &obj.customer_id, &obj.warehouse_id,
        &obj.responsible_staff, &obj.supplied_goods_id, &obj.supplied_goods_count
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

  fn update(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "UPDATE purchase SET date = $1, is_resolved = $2, customer_id = $3, warehouse_id = $4,
      responsible_staff = $5, supplied_goods_id = $6, supplied_goods_count = $7 WHERE id = $8",
      &[&obj.date, &obj.is_resolved, &obj.customer_id, &obj.warehouse_id,
        &obj.responsible_staff, &obj.supplied_goods_id, &obj.supplied_goods_count, &obj.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM purchase WHERE id = $1", &[&id])?;
    Ok(())
  }
}
