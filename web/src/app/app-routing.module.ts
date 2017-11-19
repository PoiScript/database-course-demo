import { NgModule } from '@angular/core'
import { RouterModule, Routes } from '@angular/router'

import { CustomerComponent } from './customer/customer.component'
import { GoodsComponent } from './goods/goods.component'
import { PurchaseComponent } from './purchase/purchase.component'
import { ReceiptComponent } from './receipt/receipt.component'
import { StaffComponent } from './staff/staff.component'

const routes: Routes = [
  {path: 'customer', component: CustomerComponent},
  {path: 'goods', component: GoodsComponent},
  {path: 'purchase', component: PurchaseComponent},
  {path: 'receipt', component: ReceiptComponent},
  {path: 'staff', component: StaffComponent}
]

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})

export class AppRoutingModule {}
