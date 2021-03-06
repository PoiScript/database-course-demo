import { Staff } from './staff.model'
import { Warehouse } from './warehouse.model'
import { Supplier } from './supplier.model'

export class Receipt {
  id: number
  date: string
  is_resolved: boolean
  warehouse_id: number
  responsible_staff: number
  supplied_goods_id: number
  supplied_goods_count: number
}

export class JoinedReceipt {
  warehouse: Warehouse
  receipt: Receipt
  staff: Staff
  supplier: Supplier
  goods_name: string
  goods_original_cost: number
  goods_sale_price: number
}
