import { Component, OnInit } from '@angular/core'
import { MatDialog } from '@angular/material'

import { ApiService } from '../core'
import { JoinedStaff, Staff } from '../shared'
import { StaffEditorDialogComponent } from './staff-editor-dialog.component'

@Component({
  templateUrl: './staff.component.html'
})
export class StaffComponent implements OnInit {

  staffs: JoinedStaff[]

  constructor (private api: ApiService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.api.getStaffs()
      .subscribe(staffs => this.staffs = staffs)
  }

  openUpdateDialog (staff: Staff) {
    let dialogRef = this.dialog.open(StaffEditorDialogComponent, {
      data: staff
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          console.log(result)
          this.api.updateStaff(result as Staff)
            .subscribe(_ => {})
        }
      }
    )
  }

}
