import { Supplier } from './supplier.model'

export class Goods {
  name: String
  sale_price: Number
}

export class SuppliedGoods {
  id: Number;
  original_cost: Number;
}

export class JoinedGoods {
  goods: Goods;
  supplier: Supplier;
  supplied_goods: SuppliedGoods;
}
