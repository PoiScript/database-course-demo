import { NgModule } from '@angular/core'
import { CommonModule } from '@angular/common'
import { FormsModule } from '@angular/forms'
import {
  MatIconModule,
  MatCardModule,
  MatListModule,
  MatInputModule,
  MatSelectModule,
  MatButtonModule,
  MatDialogModule,
  MatSidenavModule,
  MatToolbarModule,
  MatExpansionModule,
  MatFormFieldModule,
} from '@angular/material'

@NgModule({
  imports: [
    CommonModule
  ],
  exports: [
    CommonModule,
    FormsModule,
    MatIconModule,
    MatCardModule,
    MatListModule,
    MatInputModule,
    MatSelectModule,
    MatButtonModule,
    MatDialogModule,
    MatSidenavModule,
    MatToolbarModule,
    MatExpansionModule,
    MatFormFieldModule
  ]
})
export class SharedModule {}
