use postgres::Connection;

use database::Query;

#[derive(Serialize, Deserialize)]
pub struct Warehouse {
  id: i32,
  name: String,
  address: String,
  responsible_staff: i32,
}

impl Warehouse {
  fn new(id: i32, name: String, address: String, responsible_staff: i32) -> Warehouse {
    Warehouse { id, name, address, responsible_staff }
  }
}

impl Query for Warehouse {
  fn create(&self, conn: &Connection) {
    &conn.execute(
      "INSERT INTO warehouse (id, name, address, responsible_staff) VALUES ($1, $2, $3, $4)",
      &[&self.id, &self.name, &self.address, &self.responsible_staff]
    ).unwrap();
  }

  fn get_all(conn: &Connection) -> Vec<Self> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, address, responsible_staff FROM warehouse", &[]).unwrap() {
      vec.push(Warehouse::new(
        row.get(0),
        row.get(1),
        row.get(2),
        row.get(3)
      ));
    }
    vec
  }

  fn get_by_id(conn: &Connection, id: i32) -> Self {
    let rows = &conn.query("SELECT id, name, address, responsible_staff FROM warehouse WHERE id = $1", &[&id]).unwrap();
    let first_row = rows.get(0);
    Warehouse::new(
      first_row.get(0),
      first_row.get(1),
      first_row.get(2),
      first_row.get(3)
    )
  }

  fn update(&self, conn: &Connection) {
    &conn.execute(
      "UPDATE warehouse SET name = $1, address = $2, responsible_staff = $3 WHERE id = $4",
      &[&self.name, &self.address, &self.responsible_staff, &self.id]
    ).unwrap();
  }
}
