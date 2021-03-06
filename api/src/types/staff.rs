use postgres::Connection;
use postgres::rows::Row;

use database::Query;
use error::Error;

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "department")]
pub enum Department {
  Selling,
  Warehouseman,
  Purchasing
}

#[derive(Serialize, Deserialize)]
pub struct Staff {
  id: i32,
  tele: i32,
  name: String,
  address: String,
  staff_department: Department
}

impl Staff {
  pub fn new(id: i32, tele: i32, name: String, address: String, staff_department: Department) -> Staff {
    Staff { id, tele, name, address, staff_department }
  }

  fn from_row(row: Row) -> Staff {
    Staff::new(
      row.get(0), row.get(1), row.get(2),
      row.get(3), row.get(4)
    )
  }
}

impl Query for Staff {
  fn create(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO staff (id, tele, name, address, staff_department) VALUES ($1, $2, $3, $4, $5)",
      &[&obj.id, &obj.tele, &obj.name, &obj.address, &obj.staff_department]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, tele, name, address, staff_department FROM staff", &[])? {
      vec.push(Staff::from_row(row));
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, tele, name, address, staff_department FROM staff WHERE id = $1", &[&id])?;
    Ok(Staff::from_row(rows.get(0)))
  }

  fn update(conn: &Connection, obj: Self) -> Result<(), Error> {
    &conn.execute(
      "UPDATE staff SET tele = $1, name = $2, address = $3, staff_department = $4 WHERE id = $5",
      &[&obj.tele, &obj.name, &obj.address, &obj.staff_department, &obj.id]
    )?;
    Ok(())
  }

  fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error> {
    &conn.execute("DELETE FROM staff WHERE id = $1", &[&id])?;
    Ok(())
  }
}
