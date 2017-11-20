import { Component } from '@angular/core'
import { UserService } from './core'

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {

  constructor (private userService: UserService) {}

  isLogged (): boolean {
    return this.userService.isLogged()
  }

}
