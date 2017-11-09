use postgres::Connection;

use database::Query;

pub struct Goods {
  pub id: i32,
  pub name: String,
  pub supplier: i32,
  pub price: f32
}

impl Query for Goods {
  fn create(&self, conn: &Connection) {
    &conn.execute(
      "INSERT INTO goods (id, name, supplier, price) VALUES ($1, $2, $3, $4)",
      &[&self.id, &self.name, &self.supplier, &self.price]
    ).unwrap();
  }

  fn get_all(conn: &Connection) -> Vec<Self> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, supplier, price FROM goods", &[]).unwrap() {
      vec.push(Goods {
        id: row.get(0),
        name: row.get(1),
        supplier: row.get(2),
        price: row.get(3)
      });
    }
    vec
  }

  fn get_by_id(conn: &Connection, id: i32) -> Self {
    let rows = &conn.query("SELECT id, name, supplier, price FROM goods WHERE id = $1", &[&id]).unwrap();
    let first_row = rows.get(0);
    Goods {
      id: first_row.get(0),
      name: first_row.get(1),
      supplier: first_row.get(2),
      price: first_row.get(3)
    }
  }

  fn update(&self, conn: &Connection) {
    &conn.execute(
      "UPDATE goods SET name = $1, supplier = $2, price = $3 WHERE id = $4",
      &[&self.name, &self.supplier, &self.price, &self.id]
    ).unwrap();
  }
}

pub struct Purchase {
  pub id: i32,
  pub goods: Vec<(i32, i32)>,
  pub resolved: bool
}

pub struct Receipt {
  pub id: i32,
  pub goods: Vec<(i32, i32)>,
  pub resolved: bool
}

pub struct Staff {
  pub id: i32,
  pub tele: i32,
  pub address: String
}

pub struct Supplier {
  pub id: i32,
  pub zip: i32,
  pub tele: i32,
  pub contact: String,
  pub address: String
}

pub struct Warehouse {
  pub id: i32,
  pub name: String,
  pub address: String
}
