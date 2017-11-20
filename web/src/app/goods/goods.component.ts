import { Component, OnInit } from '@angular/core'

import { ApiService } from '../core'
import { Goods, JoinedGoods, SuppliedGoods } from '../shared'
import { MatDialog } from '@angular/material'
import { GoodsEditorDialogComponent } from './goods-editor-dialog.component'

@Component({
  templateUrl: './goods.component.html'
})
export class GoodsComponent implements OnInit {

  goodsArray: JoinedGoods[]

  constructor (private api: ApiService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.loadGoods()
  }

  loadGoods () {
    this.api.getGoods().subscribe(goods => this.goodsArray = goods)
  }

  openUpdateDialog (supplied_goods: SuppliedGoods) {
    let dialogRef = this.dialog.open(GoodsEditorDialogComponent, {
      data: supplied_goods
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.updateGoods(result as Goods)
            .subscribe(_ => {})
        }
      }
    )
  }

  openCreateDialog () {
    let id = Math.max.apply(Math, this.goodsArray.map(c => c.supplied_goods.id)) + 1

    let dialogRef = this.dialog.open(GoodsEditorDialogComponent, {
      data: {id}
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.createGoods(result as Goods)
            .subscribe(_ => {})
        }
      }
    )
  }

  deleteGoods (id: number) {
    this.api.deleteGoods(id)
      .subscribe(_ => {})
  }

}
