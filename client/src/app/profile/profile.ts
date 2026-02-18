import { Component, computed, inject, Signal, signal } from '@angular/core'
import { getAvatarUrl } from '../_helpers/util'
import { PassportService } from '../_services/passport-service'
import { MatDialog } from '@angular/material/dialog'
import { UploadImg } from '../_dialogs/upload-img/upload-img'
import { UserService } from '../_services/user-service'
import { MatIconModule } from '@angular/material/icon'
import { ProfileStats } from '../_models/profile-stats'
import { FormsModule } from '@angular/forms'

@Component({
  selector: 'app-profile',
  imports: [MatIconModule, FormsModule],
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
    ref.afterClosed().subscribe(async result => {
      console.log('UploadImg dialog result:', result)
      if (result) {
        let error: string | null = null
        // Robust check for File or Blob-like object
        const isFile = (typeof result === 'object' && 'name' in result && 'size' in result);

        if (isFile) {
          console.log('Detected result as File object, uploading...', (result as any).name)
          error = await this._user.uploadAvatarImg(result as File)
        } else if (typeof result === 'string') {
          console.log('Detected result as URL string, updating...', result)
          error = await this._user.updateAvatarUrl(result)
        } else {
          console.warn('Unknown result type from dialog:', result)
        }

        if (error) {
          console.error('Profile update failed:', error)
          alert('Failed to update profile picture: ' + error)
        } else {
          console.log('Profile picture updated successfully')
        }
      }
    })
  }
}
