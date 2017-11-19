import { Component, OnInit } from '@angular/core'
import { MatDialog } from '@angular/material'

import { ApiService } from '../core'
import { Customer } from '../shared'
import { CustomerEditorDialogComponent } from './customer-editor-dialog.component'

@Component({
  templateUrl: './customer.component.html'
})
export class CustomerComponent implements OnInit {

  customers: Customer[]

  constructor (private api: ApiService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.api.getCustomers()
      .subscribe(customers => this.customers = customers)
  }

  openUpdateDialog (customer: Customer) {
    let dialogRef = this.dialog.open(CustomerEditorDialogComponent, {
      data: customer
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          console.log(result)
          this.api.updateCustomer(result as Customer)
            .subscribe(_ => {})
        }
      }
    )
  }

}
