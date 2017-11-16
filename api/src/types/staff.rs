use postgres::Connection;

use database::Query;
use error::Error;

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "department")]
enum Department {
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
  fn new(id: i32, tele: i32, name: String, address: String, staff_department: Department) -> Staff {
    Staff { id, tele, name, address, staff_department }
  }
}

impl Query for Staff {
  fn create(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "INSERT INTO staff (id, tele, name, address, staff_department) VALUES ($1, $2, $3, $4, $5)",
      &[&self.id, &self.tele, &self.name, &self.address, &self.staff_department]
    )?;
    Ok(())
  }

  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, tele, name, address, staff_department FROM staff", &[])? {
      vec.push(Staff::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3),
        row.get(4))
      );
    }
    Ok(vec)
  }

  fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
    let rows = &conn.query("SELECT id, tele, name, address, staff_department FROM staff WHERE id = $1", &[&id])?;
    let first_row = rows.get(0);
    Ok(Staff::new(
      first_row.get(0),
      first_row.get(1),
      first_row.get(2),
      first_row.get(3),
      first_row.get(4)
    ))
  }

  fn update(&self, conn: &Connection) -> Result<(), Error> {
    &conn.execute(
      "UPDATE goods SET tele = $1, name = $2, address = $3, staff_department = $4 WHERE id = $5",
      &[&self.tele, &self.name, &self.address, &self.staff_department, &self.id]
    )?;
    Ok(())
  }
}
