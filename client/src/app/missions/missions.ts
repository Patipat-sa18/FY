import { Component, computed, inject, Signal } from '@angular/core'
import { MissionService } from '../_services/mission-service'
import { MissionFilter } from '../_models/mission-filter'
import { Mission } from '../_models/mission'
import { FormsModule } from '@angular/forms'
import { BehaviorSubject } from 'rxjs'
import { AsyncPipe, DatePipe } from '@angular/common'
import { PassportService } from '../_services/passport-service'
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar'
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner'
import { MatIconModule } from '@angular/material/icon'

@Component({
  selector: 'app-missions',
  imports: [FormsModule, AsyncPipe, DatePipe, MatSnackBarModule, MatProgressSpinnerModule, MatIconModule],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions {
  private _mission = inject(MissionService)
  private _passport = inject(PassportService)
  private _snackBar = inject(MatSnackBar)

  filter: MissionFilter = {}
  
  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly missions$ = this._missionsSubject.asObservable()
  isSignin: Signal<boolean>
  userId: Signal<number | undefined>

  isLoading = false
  editingId: number | undefined
  editForm: Partial<Mission> = {}

  constructor() {
    this.isSignin = computed(() => this._passport.data() !== undefined)
    this.userId = computed(() => this._passport.data()?.id)
    this.filter = this._mission.filter
    this.loadMyMission()
  }

  private async loadMyMission() {
    try {
      this.isLoading = true
      const missions = await this._mission.getByFilter(this.filter)
      this._missionsSubject.next(missions)
    } catch (e) {
      console.error(e)
    } finally {
      this.isLoading = false
    }
  }

  async onSubmit() {
    this.loadMyMission()
  }

  notificationMessage: string | null = null

  async join(missionId: number) {
    try {
      this.isLoading = true
      const message = await this._mission.join(missionId)
      // Wait a moment to ensure DB update propagates if needed, gives a "processing" feel
      await new Promise(resolve => setTimeout(resolve, 300));
      await this.loadMyMission()
      this.showNotification('Completed')
    } catch (e: any) {
      // If responseType is text, success might still throw if status != 200, 
      // but if status is 200 it returns string.
      // If error, e.error might be the text body.
      const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'An error occurred while joining the mission';
      this.showNotification(errorMessage)
    } finally {
      this.isLoading = false
    }
  }

  showNotification(message: string) {
    this.notificationMessage = message
    setTimeout(() => {
      this.notificationMessage = null
    }, 2000)
  }

  startEdit(mission: Mission) {
    this.editingId = mission.id
    this.editForm = { ...mission }
  }

  cancelEdit() {
    this.editingId = undefined
    this.editForm = {}
  }

  async saveEdit() {
    if (this.editingId && this.editForm) {
      try {
        this.isLoading = true
        const message = await this._mission.edit(this.editingId, this.editForm)
        this.editingId = undefined
        // Wait a small delay for DB propagation/UX feel
        await new Promise(resolve => setTimeout(resolve, 300));
        await this.loadMyMission()
        this.showNotification('Completed')
      } catch (e: any) {
        console.error(e)
        const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'Error updating mission';
        this.showNotification(errorMessage)
      } finally {
        this.isLoading = false
      }
    }
  }

  async deleteMission(missionId: number) {
    if (confirm('Are you sure you want to delete this mission?')) {
      try {
        this.isLoading = true
        const message = await this._mission.delete(missionId)
        await new Promise(resolve => setTimeout(resolve, 300));
        await this.loadMyMission()
        this.showNotification('Completed')
      } catch (e: any) {
        console.error(e)
        const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'Error deleting mission';
        this.showNotification(errorMessage)
      } finally {
        this.isLoading = false
      }
    }
  }

  isEditing(missionId: number): boolean {
    return this.editingId === missionId
  }

  closeNotification() {
    this.notificationMessage = null
  }
}
