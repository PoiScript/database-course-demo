import { Component, Inject } from '@angular/core'
import { MAT_DIALOG_DATA } from '@angular/material'

import { Customer } from '../shared'

@Component({
  templateUrl: './customer-editor-dialog.component.html'
})
export class CustomerEditorDialogComponent {

  constructor (@Inject(MAT_DIALOG_DATA) public data: Customer) { }

}