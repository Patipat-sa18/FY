import { Component, computed, inject, Signal } from '@angular/core'
import { MatButtonModule } from '@angular/material/button'
import { MatToolbarModule } from '@angular/material/toolbar'
import { PassportService } from '../_services/passport-service'
import { MatMenuModule } from '@angular/material/menu'
import { getAvatarUrl } from '../_helpers/util'
import { Router, RouterLink, RouterLinkActive } from "@angular/router"

@Component({
  selector: 'app-navbar',
  imports: [MatToolbarModule, MatButtonModule, MatMenuModule, RouterLink, RouterLinkActive],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',
})
export class Navbar {
  private _router = inject(Router)
  private _passport = inject(PassportService)
  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>

  constructor() {
    this.display_name = computed(() => this._passport.data()?.display_name)
    this.avatar_url = computed(() => this._passport.avatar())
  }

  logout() {
    this._passport.destroy()

    this._router.navigate(['/login'])
  }
}
