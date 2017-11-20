import { Injectable } from '@angular/core'

import { User } from '../shared'

@Injectable()
export class UserService {

  user: User

  isLogged (): boolean {
    return !!this.user
  }

  setUser (user: User) {
    this.user = user
  }

  isLeader (): boolean {
    return this.user.user_role === 'Leader'
  }

}
