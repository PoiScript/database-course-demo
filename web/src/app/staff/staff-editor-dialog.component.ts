import { Component, Inject } from '@angular/core'
import { MAT_DIALOG_DATA } from '@angular/material'

import { Staff } from '../shared'

@Component({
  templateUrl: './staff-editor-dialog.component.html'
})
export class StaffEditorDialogComponent {

  constructor (@Inject(MAT_DIALOG_DATA) public data: Staff) { }

}