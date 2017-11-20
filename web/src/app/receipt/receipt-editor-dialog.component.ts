import { Component, Inject } from '@angular/core'
import { MAT_DIALOG_DATA } from '@angular/material'

import { Receipt } from '../shared'

@Component({
  templateUrl: './receipt-editor-dialog.component.html'
})
export class ReceiptEditorDialogComponent {

  constructor (@Inject(MAT_DIALOG_DATA) public data: Receipt) { }

}