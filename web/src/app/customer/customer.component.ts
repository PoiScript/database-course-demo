import { Component, OnInit } from '@angular/core'
import { MatDialog } from '@angular/material'

import { ApiService, UserService } from '../core'
import { Customer } from '../shared'
import { CustomerEditorDialogComponent } from './customer-editor-dialog.component'

@Component({
  templateUrl: './customer.component.html'
})
export class CustomerComponent implements OnInit {

  customers: Customer[]

  constructor (private api: ApiService,
               private userService: UserService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.loadCustomers()
  }

  isLeader (): boolean {
    return this.userService.isLeader()
  }

  loadCustomers () {
    this.api.getCustomers().subscribe(customers => this.customers = customers)
  }

  openUpdateDialog (customer: Customer) {
    let dialogRef = this.dialog.open(CustomerEditorDialogComponent, {
      data: customer
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.updateCustomer(result as Customer)
            .subscribe(_ => {})
        }
      }
    )
  }

  openCreateDialog () {
    let id = Math.max.apply(Math, this.customers.map(c => c.id)) + 1

    let dialogRef = this.dialog.open(CustomerEditorDialogComponent, {
      data: {id}
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.createCustomer(result as Customer)
            .subscribe(_ => {})
        }
      }
    )
  }

  deleteCustomer (id: number) {
    this.api.deleteCustomer(id)
      .subscribe(_ => {})
  }

}
