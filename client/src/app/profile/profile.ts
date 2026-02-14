import { Component, computed, inject, Signal, signal } from '@angular/core'
import { getAvatarUrl } from '../_helpers/util'
import { PassportService } from '../_services/passport-service'
import { MatDialog } from '@angular/material/dialog'
import { UploadImg } from '../_dialogs/upload-img/upload-img'
import { UserService } from '../_services/user-service'
import { MatIconModule } from '@angular/material/icon'
import { ProfileStats } from '../_models/profile-stats'
import { AsyncPipe } from '@angular/common'
import { FormsModule } from '@angular/forms'

@Component({
  selector: 'app-profile',
  imports: [MatIconModule, AsyncPipe, FormsModule],
  templateUrl: './profile.html',
  styleUrl: './profile.scss',
})
export class Profile {
  avatar_url: Signal<string>
  display_name: Signal<string>
  private _passport = inject(PassportService)
  private _dialog = inject(MatDialog)
  private _user = inject(UserService)

  stats = signal<ProfileStats | null>(null)

  isEditing = signal(false)
  newUsername = ''

  constructor() {
    this.avatar_url = computed(() => this._passport.avatar())
    this.display_name = computed(() => this._passport.data()?.display_name || 'Guest')
    this.loadStats()
  }

  toggleEdit() {
    this.newUsername = this.display_name()
    this.isEditing.update(v => !v)
  }

  async saveName() {
    if (!this.newUsername || this.newUsername === this.display_name()) {
      this.isEditing.set(false)
      return
    }

    const error = await this._user.updateUsername(this.newUsername)
    if (error) {
      console.error(error)
      // Todo: show error
    } else {
      this._passport.updateDisplayName(this.newUsername)
      this.isEditing.set(false)
    }
  }

  async loadStats() {
    const stats = await this._user.getProfileStats()
    this.stats.set(stats)
  }

  openDialog() {
    const ref = this._dialog.open(UploadImg)
    ref.afterClosed().subscribe(async file => {
      if (file) {
        const error = await this._user.uploadAvatarImg(file)
        if (error)
          console.error(error)
      }
    })
  }
}
