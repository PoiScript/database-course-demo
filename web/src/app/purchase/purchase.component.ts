import { Component, OnInit } from '@angular/core'
import { MatDialog } from '@angular/material'

import { JoinedPurchase, Purchase } from '../shared'
import { ApiService } from '../core'
import { PurchaseEditorDialogComponent } from './purchase-editor-dialog.component'

@Component({
  templateUrl: './purchase.component.html'
})
export class PurchaseComponent implements OnInit {

  purchases: JoinedPurchase[]

  constructor (private api: ApiService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.loadPurchases()
  }

  loadPurchases () {
    this.api.getPurchases()
      .subscribe(purchases => this.purchases = purchases)
  }

  openCreateDialog () {
    let id = Math.max.apply(Math, this.purchases.map(c => c.purchase.id)) + 1

    let dialogRef = this.dialog.open(PurchaseEditorDialogComponent, {
      data: {id}
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.createPurchase(result as Purchase)
            .subscribe(_ => {})
        }
      }
    )
  }

}
