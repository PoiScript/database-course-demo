import { Component, OnInit } from '@angular/core'
import { MatDialog } from '@angular/material'

import { ApiService, UserService } from '../core'
import { JoinedStaff, Staff } from '../shared'
import { StaffEditorDialogComponent } from './staff-editor-dialog.component'

@Component({
  templateUrl: './staff.component.html'
})
export class StaffComponent implements OnInit {

  staffs: JoinedStaff[]

  constructor (private api: ApiService,
               private userService: UserService,
               public dialog: MatDialog) { }

  ngOnInit () {
    this.loadStaffs()
  }

  isLeader (): boolean {
    return this.userService.isLeader()
  }

  loadStaffs() {
    this.api.getStaffs().subscribe(staffs => this.staffs = staffs)
  }

  openUpdateDialog (staff: Staff) {
    let dialogRef = this.dialog.open(StaffEditorDialogComponent, {
      data: staff
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.updateStaff(result as Staff)
            .subscribe(_ => {})
        }
      }
    )
  }

  openCreateDialog () {
    let id = Math.max.apply(Math, this.staffs.map(s => s.staff.id)) + 1

    let dialogRef = this.dialog.open(StaffEditorDialogComponent, {
      data: {id}
    })

    dialogRef.afterClosed().subscribe(
      result => {
        if (result) {
          this.api.createStaff(result as Staff)
            .subscribe(_ => {})
        }
      }
    )
  }

  deleteStaff (id: number) {
    this.api.deleteStaff(id)
      .subscribe(_ => {})
  }

}
