use postgres::Connection;

use database::Query;

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
  fn create(&self, conn: &Connection) {
    &conn.execute(
      "INSERT INTO goods (id, name, sale_price) VALUES ($1, $2, $3)",
      &[&self.id, &self.name, &self.sale_price]
    ).unwrap();
  }

  fn get_all(conn: &Connection) -> Vec<Self> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, sale_price FROM goods", &[]).unwrap() {
      vec.push(Goods::new(row.get(0), row.get(1), row.get(2)));
    }
    vec
  }

  fn get_by_id(conn: &Connection, id: i32) -> Self {
    let rows = &conn.query("SELECT id, name, sale_price FROM goods WHERE id = $1", &[&id]).unwrap();
    let first_row = rows.get(0);
    Goods::new(
     first_row.get(0),
     first_row.get(1),
     first_row.get(2)
    )
  }

  fn update(&self, conn: &Connection) {
    &conn.execute(
      "UPDATE goods SET name = $1, sale_price = $2 WHERE id = $3",
      &[&self.name, &self.sale_price, &self.id]
    ).unwrap();
  }
}
