import { Component, Inject } from '@angular/core'
import { MAT_DIALOG_DATA } from '@angular/material'

import { Purchase } from '../shared'

@Component({
  templateUrl: './purchase-editor-dialog.component.html'
})
export class PurchaseEditorDialogComponent {

  constructor (@Inject(MAT_DIALOG_DATA) public data: Purchase) { }

}
