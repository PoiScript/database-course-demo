import { NgModule } from '@angular/core'
import { HttpClientModule } from '@angular/common/http'
import { BrowserModule } from '@angular/platform-browser'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations'

import { AppComponent } from './app.component'
import { AppRoutingModule } from './app-routing.module'
import { CoreModule } from './core'
import { SharedModule } from './shared'

import { CustomerComponent, CustomerEditorDialogComponent } from './customer'
import { GoodsComponent, GoodsEditorDialogComponent } from './goods'
import { PurchaseComponent, PurchaseEditorDialogComponent } from './purchase'
import { ReceiptComponent, ReceiptEditorDialogComponent } from './receipt'
import { StaffComponent, StaffEditorDialogComponent } from './staff'

@NgModule({
  declarations: [
    AppComponent,
    CustomerComponent,
    CustomerEditorDialogComponent,
    GoodsComponent,
    GoodsEditorDialogComponent,
    PurchaseComponent,
    PurchaseEditorDialogComponent,
    ReceiptComponent,
    ReceiptEditorDialogComponent,
    StaffComponent,
    StaffEditorDialogComponent
  ],
  entryComponents: [
    GoodsEditorDialogComponent,
    StaffEditorDialogComponent,
    PurchaseEditorDialogComponent,
    CustomerEditorDialogComponent,
    ReceiptEditorDialogComponent
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
