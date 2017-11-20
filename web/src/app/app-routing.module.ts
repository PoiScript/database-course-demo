import { Injectable, NgModule } from '@angular/core'
import { CanActivate, Router, RouterModule, Routes } from '@angular/router'

import { CustomerComponent } from './customer'
import { GoodsComponent } from './goods'
import { PurchaseComponent } from './purchase'
import { ReceiptComponent } from './receipt'
import { StaffComponent } from './staff'
import { LoginComponent } from './login/login.component'
import { UserService } from './core'

@Injectable()
export class AuthGuard implements CanActivate {

  constructor (private userService: UserService,
               private router: Router) {}

  canActivate () {
    if (this.userService.isLogged()) {
      return true
    }
    this.router.navigate(['/login'])
    return false
  }
}

const routes: Routes = [
  {path: '', redirectTo: 'receipt', pathMatch: 'full'},
  {path: 'login', component: LoginComponent},
  {path: 'customer', component: CustomerComponent, canActivate: [AuthGuard]},
  {path: 'goods', component: GoodsComponent, canActivate: [AuthGuard]},
  {path: 'purchase', component: PurchaseComponent, canActivate: [AuthGuard]},
  {path: 'receipt', component: ReceiptComponent, canActivate: [AuthGuard]},
  {path: 'staff', component: StaffComponent, canActivate: [AuthGuard]}
]

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
  providers: [AuthGuard]
})
export class AppRoutingModule {}
