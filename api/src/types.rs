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

impl Query for Staff {
  fn create(&self, conn: &Connection) {
    &conn.execute(
      "INSERT INTO staff (id, tele, address) VALUES ($1, $2, $3)",
      &[&self.id, &self.tele, &self.address]
    ).unwrap();
  }

  fn get_all(conn: &Connection) -> Vec<Self> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, tele, address FROM staff", &[]).unwrap() {
      vec.push(Staff {
        id: row.get(0),
        tele: row.get(1),
        address: row.get(2)
      });
    }
    vec
  }

  fn get_by_id(conn: &Connection, id: i32) -> Self {
    let rows = &conn.query("SELECT tele, address FROM staff WHERE id = $1", &[&id]).unwrap();
    let first_row = rows.get(0);
    Staff {
      id: first_row.get(0),
      tele: first_row.get(1),
      address: first_row.get(2)
    }
  }

  fn update(&self, conn: &Connection) {
    &conn.execute(
      "UPDATE goods SET tele = $1, address = $2 WHERE id = $3",
      &[&self.tele, &self.address, &self.id]
    ).unwrap();
  }
}

pub struct Supplier {
  pub id: i32,
  pub zip: i32,
  pub tele: i32,
  pub contact: String,
  pub address: String
}


impl Query for Supplier {
  fn create(&self, conn: &Connection) {
    &conn.execute(
      "INSERT INTO supplier (id, zip, tele, contact, address) VALUES ($1, $2, $3, $4, $5)",
      &[&self.id, &self.zip, &self.tele, &self.contact, &self.address]
    ).unwrap();
  }

  fn get_all(conn: &Connection) -> Vec<Self> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, zip, tele, contact, address FROM supplier", &[]).unwrap() {
      vec.push(Supplier {
        id: row.get(0),
        zip: row.get(1),
        tele: row.get(2),
        contact: row.get(3),
        address: row.get(4)
      });
    }
    vec
  }

  fn get_by_id(conn: &Connection, id: i32) -> Self {
    let rows = &conn.query("SELECT zip, tele, contact, address FROM supplier WHERE id = $1", &[&id]).unwrap();
    let first_row = rows.get(0);
    Supplier {
      id: first_row.get(0),
      zip: first_row.get(1),
      tele: first_row.get(2),
      contact: first_row.get(3),
      address: first_row.get(4)
    }
  }

  fn update(&self, conn: &Connection) {
    &conn.execute(
      "UPDATE supplier SET zip = $1, tele = $2, contact = $3, address = $4 WHERE id = $5",
      &[&self.zip, &self.tele, &self.contact, &self.address, &self.id]
    ).unwrap();
  }
}

pub struct Warehouse {
  pub id: i32,
  pub name: String,
  pub address: String
}

impl Query for Warehouse {
  fn create(&self, conn: &Connection) {
    &conn.execute(
      "INSERT INTO warehouse (id, name, address) VALUES ($1, $2, $3)",
      &[&self.id, &self.name, &self.address]
    ).unwrap();
  }

  fn get_all(conn: &Connection) -> Vec<Self> {
    let mut vec = Vec::new();
    for row in &conn.query("SELECT id, name, address FROM warehouse", &[]).unwrap() {
      vec.push(Warehouse {
        id: row.get(0),
        name: row.get(1),
        address: row.get(2)
      });
    }
    vec
  }

  fn get_by_id(conn: &Connection, id: i32) -> Self {
    let rows = &conn.query("SELECT name, address FROM warehouse WHERE id = $1", &[&id]).unwrap();
    let first_row = rows.get(0);
    Warehouse {
      id: first_row.get(0),
      name: first_row.get(1),
      address: first_row.get(2)
    }
  }

  fn update(&self, conn: &Connection) {
    &conn.execute(
      "UPDATE warehouse SET name = $1, address = $2 WHERE id = $3",
      &[&self.name, &self.address, &self.id]
    ).unwrap();
  }
}