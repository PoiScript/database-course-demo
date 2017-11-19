import { NgModule } from '@angular/core'
import { HttpClientModule } from '@angular/common/http'
import { BrowserModule } from '@angular/platform-browser'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations'

import { AppComponent } from './app.component'
import { AppRoutingModule } from './app-routing.module'
import { CoreModule } from './core'
import { SharedModule } from './shared'

import { CustomerComponent, CustomerEditorDialogComponent } from './customer'
import { GoodsComponent } from './goods/goods.component'
import { PurchaseComponent } from './purchase/purchase.component'
import { ReceiptComponent } from './receipt/receipt.component'
import { StaffComponent, StaffEditorDialogComponent } from './staff'

@NgModule({
  declarations: [
    AppComponent,
    CustomerComponent,
    CustomerEditorDialogComponent,
    GoodsComponent,
    PurchaseComponent,
    ReceiptComponent,
    StaffComponent,
    StaffEditorDialogComponent
  ],
  entryComponents: [
    CustomerEditorDialogComponent,
    StaffEditorDialogComponent
  ],
  imports: [
    AppRoutingModule,
    BrowserModule,
    BrowserAnimationsModule,
    CoreModule,
    SharedModule,
    HttpClientModule
  ],
  bootstrap: [AppComponent]
})
export class AppModule {}
