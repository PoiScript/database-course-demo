pub mod goods;
pub mod staff;
pub mod receipt;
pub mod customer;
pub mod purchase;
pub mod supplier;
pub mod warehouse;
pub mod supplied_goods;

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

#[derive(Serialize, Deserialize)]
pub struct JoinedPurchase {
  pub customer: Customer,
  pub warehouse: Warehouse,
  pub purchase: Purchase
}

impl JoinedPurchase {
  pub fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query(
      "SELECT *
      FROM customer AS c, warehouse AS w, purchase AS p
      WHERE p.customer_id = c.id
      AND p.warehouse_id = w.id", &[]
    )? {
      vec.push(JoinedPurchase {
        customer: Customer::new(
          row.get(0), row.get(5),
          row.get(2), row.get(1),
          row.get(3), row.get(4)
        ),
        warehouse: Warehouse::new(
          row.get(6), row.get(7),
          row.get(8), row.get(9)
        ),
        purchase: Purchase::new(
          row.get(10), row.get(11),
          row.get(12), row.get(13),
          row.get(14), row.get(15),
          row.get(16), row.get(17)
        )
      });
    }
    Ok(vec)
  }
}

#[derive(Serialize, Deserialize)]
pub struct JoinedReceipt {
  warehouse: Warehouse,
  receipt: Receipt
}

impl JoinedReceipt {
  pub fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
    let mut vec = Vec::new();
    for row in &conn.query(
      "SELECT *
      FROM warehouse AS w, receipt AS r
      WHERE r.warehouse_id = r.id", &[]
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
        )
      });
    }
    Ok(vec)
  }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct JoinedStaff {
  staff: Staff,
  receipt_total: i32,
  purchase_total: i32
}
