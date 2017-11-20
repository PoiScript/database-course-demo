import { NgModule } from '@angular/core'

import { ApiService } from './api.service'
import { UserService } from './user.service'

@NgModule({
  providers: [
    ApiService,
    UserService
  ]
})
export class CoreModule {}
