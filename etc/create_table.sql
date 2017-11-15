CREATE TYPE department AS ENUM (
    'Selling',
    'Warehouseman',
    'Purchasing'
);

CREATE TABLE Staff (
  id INTEGER PRIMARY KEY,
  name CHAR(20) NOT NULL,
  tele INTEGER NOT NULL,
  address CHAR(40) NOT NULL,
  staff_department department NOT NULL
);

CREATE TABLE Customer (
  id INTEGER PRIMARY KEY,
  name CHAR(20) NOT NULL,
  tele INTEGER NOT NULL,
  contact CHAR(20) NOT NULL,
  address CHAR(40) NOT NULL,
  zip INTEGER NOT NULL
);

CREATE TABLE Goods (
  id INTEGER PRIMARY KEY,
  name CHAR(40) NOT NULL,
  sale_price REAL NOT NULL
);

CREATE TABLE Supplier (
  id INTEGER PRIMARY KEY,
  name CHAR(40) NOT NULL,
  tele INTEGER NOT NULL,
  contact CHAR(20) NOT NULL,
  address CHAR(40) NOT NULL,
  zip INTEGER NOT NULL
);

CREATE TABLE Supplied_Goods (
  id INTEGER PRIMARY KEY,
  original_cost REAL NOT NULL,
  goods_id INTEGER NOT NULL REFERENCES Goods(id),
  supplier_id INTEGER NOT NULL REFERENCES Supplier(id)
);

CREATE TABLE Warehouse (
  id INTEGER PRIMARY KEY,
  name CHAR(20) NOT NULL,
  address CHAR(40) NOT NULL,
  responsible_staff INTEGER NOT NULL REFERENCES Staff(id)
);

CREATE TABLE Receipt (
  id INTEGER PRIMARY KEY,
  date TIMESTAMP NOT NULL,
  is_resolved BOOLEAN NOT NULL,
  warehouse_id INTEGER NOT NULL REFERENCES Warehouse(id),
  responsible_staff INTEGER NOT NULL REFERENCES Staff(id),
  supplied_goods_id INTEGER NOT NULL REFERENCES Supplied_Goods(id),
  supplied_goods_count INTEGER NOT NULL
);

CREATE TABLE Purchase (
  id INTEGER PRIMARY KEY,
  date TIMESTAMP NOT NULL,
  is_resolved BOOLEAN NOT NULL,
  customer_id INTEGER NOT NULL REFERENCES Customer(id),
  warehouse_id INTEGER NOT NULL REFERENCES Warehouse(id),
  responsible_staff INTEGER NOT NULL REFERENCES Staff(id),
  supplied_goods_id INTEGER NOT NULL REFERENCES Supplied_Goods(id),
  supplied_goods_count INTEGER NOT NULL
);
