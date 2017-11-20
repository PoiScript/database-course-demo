import { Component, OnInit } from '@angular/core'
import { MatDialog } from '@angular/material'

import { ApiService, UserService } from '../core'
import { JoinedReceipt, Receipt } from '../shared'
import { ReceiptEditorDialogComponent } from './receipt-editor-dialog.component'

@Component({
  templateUrl: './receipt.component.html'
})
export class ReceiptComponent implements OnInit {

  receipts: JoinedReceipt[]

  constructor (private api: ApiService,
               private userService: UserService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.loadReceipts()
  }

  isLeader (): boolean {
    return this.userService.isLeader()
  }

  loadReceipts () {
    this.api.getReceipts()
      .subscribe(receipts => this.receipts = receipts)
  }

  openCreateDialog () {
    let id = Math.max.apply(Math, this.receipts.map(c => c.receipt.id)) + 1

    let dialogRef = this.dialog.open(ReceiptEditorDialogComponent, {
      data: {id}
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.createReceipt(result as Receipt)
            .subscribe(_ => {})
        }
      }
    )
  }

}
