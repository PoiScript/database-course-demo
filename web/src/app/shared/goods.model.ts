import { Supplier } from './supplier.model'

export class Goods {
  id: number
  name: string
  sale_price: number
}

export class SuppliedGoods {
  id: number
  goods_id: number
  supplier_id: number
  original_cost: number
}

export class JoinedGoods {
  goods: Goods
  supplier: Supplier
  supplied_goods: SuppliedGoods
}
