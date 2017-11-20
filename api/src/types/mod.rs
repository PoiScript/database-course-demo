pub mod user;
pub mod goods;
pub mod staff;
pub mod receipt;
pub mod customer;
pub mod purchase;
pub mod supplier;
pub mod warehouse;
pub mod supplied_goods;

pub use self::user::User;
pub use self::goods::Goods;
pub use self::staff::Staff;
pub use self::receipt::Receipt;
pub use self::customer::Customer;
pub use self::purchase::Purchase;
pub use self::supplier::Supplier;
pub use self::warehouse::Warehouse;
pub use self::supplied_goods::SuppliedGoods;

use error::Error;
use postgres::Connection;

#[derive(Deserialize)]
pub struct Id {
  pub id: i32
}

#[derive(Deserialize)]
pub struct LoginRequest {
  pub username: String,
  pub password: String
}

#[derive(Serialize)]
pub struct JoinedPurchase {
  customer: Customer,
  warehouse: Warehouse,
  purchase: Purchase,
  staff: Staff,
  goods_name: String,
  goods_original_cost: f32,
  goods_sale_price: f32
}

impl JoinedPurchase {
  pub fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query(
      "SELECT *
        FROM purchase AS p, customer AS c, warehouse AS w, staff AS s, (
          SELECT s.id AS id, name, original_cost, sale_price
          FROM supplied_goods AS s , goods AS g
          WHERE s.goods_id = g.id
        ) as g
        WHERE p.customer_id = c.id
        AND p.warehouse_id = w.id
        AND p.responsible_staff = s.id
        AND p.supplied_goods_id = g.id", &[]
    )? {
      vec.push(JoinedPurchase {
        purchase: Purchase::new(
          row.get(0), row.get(1),
          row.get(2), row.get(3),
          row.get(4), row.get(5),
          row.get(6), row.get(7)
        ),
        customer: Customer::new(
          row.get(8), row.get(13),
          row.get(10), row.get(9),
          row.get(11), row.get(12)
        ),
        warehouse: Warehouse::new(
          row.get(14), row.get(15),
          row.get(16), row.get(17)
        ),
        staff: Staff::new(
          row.get(18), row.get(20),
          row.get(19), row.get(21),
          row.get(22)
        ),
        goods_name: row.get(24),
        goods_original_cost: row.get(25),
        goods_sale_price: row.get(26)
      });
    }
    Ok(vec)
  }
}

#[derive(Serialize, Deserialize)]
pub struct JoinedReceipt {
  warehouse: Warehouse,
  receipt: Receipt,
  staff: Staff,
  supplier: Supplier,
  goods_name: String,
  goods_original_cost: f32,
  goods_sale_price: f32,
}

impl JoinedReceipt {
  pub fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query(
      "SELECT *
      FROM warehouse AS w, receipt AS r, staff AS s, (
        SELECT sg.id AS sg_id, g.name, original_cost, sale_price,
        s.id, s.zip, s.tele, s.name, s.contact, s.address
        FROM supplied_goods AS sg , goods AS g, supplier AS s
        WHERE sg.goods_id = g.id AND sg.supplier_id = s.id
      ) as g
      WHERE r.warehouse_id = w.id
      AND r.responsible_staff = s.id
      AND r.supplied_goods_id = g.sg_id", &[]
    )? {
      vec.push(JoinedReceipt {
        warehouse: Warehouse::new(
          row.get(0), row.get(1),
          row.get(2), row.get(3)
        ),
        receipt: Receipt::new(
          row.get(4), row.get(5), row.get(6),
          row.get(7), row.get(8),
          row.get(9), row.get(10)
        ),
        staff: Staff::new(
          row.get(11), row.get(13),
          row.get(12), row.get(14),
          row.get(15)
        ),
        goods_name: row.get(17),
        goods_original_cost: row.get(18),
        goods_sale_price: row.get(19),
        supplier: Supplier::new(
          row.get(20), row.get(21),
          row.get(22), row.get(23),
          row.get(24), row.get(25)
        )
      });
    }
    Ok(vec)
  }
}

#[derive(Serialize)]
pub struct JoinedSuppliedGoods {
  goods: Goods,
  supplier: Supplier,
  supplied_goods: SuppliedGoods
}

impl JoinedSuppliedGoods {
  pub fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query(
      "SELECT *
      FROM goods AS g, supplier AS s, supplied_goods AS sg
      WHERE sg.goods_id = g.id
      AND sg.supplier_id = s.id", &[]
    )? {
      vec.push(JoinedSuppliedGoods {
        goods: Goods::new(
          row.get(0), row.get(1), row.get(2)
        ),
        supplier: Supplier::new(
          row.get(3), row.get(8),
          row.get(5), row.get(4),
          row.get(6), row.get(7)
        ),
        supplied_goods: SuppliedGoods::new(
          row.get(9), row.get(10),
          row.get(11), row.get(12)
        )
      });
    }
    Ok(vec)
  }
}

#[derive(Serialize)]
pub struct JoinedStaff {
  staff: Staff,
  receipt_total: f64
}

impl JoinedStaff {
  pub fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query(
      "SELECT s.id, s.tele, s.name, s.address, s.staff_department, COALESCE(SUM(total), 0)
      FROM staff AS s
      LEFT JOIN (
        SELECT r.responsible_staff, supplied_goods_count * original_cost as total
        FROM receipt AS r, supplied_goods AS g WHERE r.supplied_goods_id = g.id
      ) AS r ON s.id = r.responsible_staff
      GROUP BY s.id", &[]
    )? {
      vec.push(JoinedStaff {
        staff: Staff::new(
          row.get(0), row.get(1),
          row.get(2), row.get(3),
          row.get(4)
        ),
        receipt_total: row.get(5)
      });
    }
    Ok(vec)
  }
}
