import { Component, inject } from '@angular/core'
import { Router } from '@angular/router'
import { PassportService } from '../_services/passport-service'

@Component({
  selector: 'app-home',
  imports: [],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class Home {
  private _router = inject(Router)
  private _passport = inject(PassportService)

  constructor() {
    if (!this._passport.data())
      this._router.navigate(['/login'])
  }
}
