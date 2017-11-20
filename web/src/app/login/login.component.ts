import { Component } from '@angular/core'

import { ApiService, UserService } from '../core'
import { Router } from '@angular/router'

@Component({
  templateUrl: './login.component.html'
})
export class LoginComponent {

  username: string
  password: string

  constructor (private api: ApiService,
               private router: Router,
               private userService: UserService) {}

  login () {
    this.api.login(this.username, this.password)
      .subscribe(user => {
        this.userService.setUser(user)
        this.router.navigate(['receipt'])
      })
  }

}