import { Component, Inject } from '@angular/core'
import { MAT_DIALOG_DATA } from '@angular/material'

import { SuppliedGoods } from '../shared'

@Component({
  templateUrl: './goods-editor-dialog.component.html'
})
export class GoodsEditorDialogComponent {

  constructor (@Inject(MAT_DIALOG_DATA) public data: SuppliedGoods) { }

}
