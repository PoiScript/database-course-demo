import { Customer } from './customer.model'
import { Warehouse } from './warehouse.model'
import { Staff } from './staff.model'

export class Purchase {
  id: number
  date: string
  is_resolved: boolean
  customer_id: number
  warehouse_id: number
  responsible_staff: number
  supplied_goods_id: number
  supplied_goods_count: number
}

export class JoinedPurchase {
  customer: Customer
  warehouse: Warehouse
  purchase: Purchase
  staff: Staff
  goods_name: string
  goods_original_cost: number
  goods_sale_price: number
}
