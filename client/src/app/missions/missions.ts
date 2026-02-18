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

  filter: MissionFilter = {}

  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly missions$ = this._missionsSubject.asObservable()
  isSignin: Signal<boolean>
  userId: Signal<number | undefined>

  isLoading = false
  editingId: number | undefined
  editForm: Partial<Mission> = {}
  myMemberships: number[] = []

  constructor() {
    this.isSignin = computed(() => this._passport.data() !== undefined)
    this.userId = computed(() => this._passport.data()?.id)
    this.filter = this._mission.filter
    this.loadMyMission()
    this.loadMyMemberships()
  }

  private async loadMyMemberships() {
    if (this.isSignin()) {
      try {
        this.myMemberships = await this._mission.getMyMemberships()
      } catch (e) {
        console.error('Failed to load memberships', e)
      }
    }
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
      // Optimistic update
      if (!this.myMemberships.includes(missionId)) {
        this.myMemberships.push(missionId);
      }

      await this._mission.join(missionId)
      await this.loadMyMission()
      await this.loadMyMemberships()
      this.showNotification('Joined Mission')
    } catch (e: any) {
      // Rollback on error
      this.myMemberships = this.myMemberships.filter(id => id !== missionId);
      const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'An error occurred while joining the mission';
      this.showNotification(errorMessage)
    } finally {
      this.isLoading = false
    }
  }

  async start(missionId: number) {
    if (confirm('Are you sure you want to start this mission?')) {
      try {
        this.isLoading = true
        await this._mission.start(missionId)
        await new Promise(resolve => setTimeout(resolve, 300));
        await this.loadMyMission()
        this.showNotification('Mission Started')
      } catch (e: any) {
        const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'Error starting mission';
        this.showNotification(errorMessage)
      } finally {
        this.isLoading = false
      }
    }
  }

  async complete(missionId: number) {
    if (confirm('Are you sure you want to complete this mission?')) {
      try {
        this.isLoading = true
        await this._mission.complete(missionId)
        await new Promise(resolve => setTimeout(resolve, 300));
        await this.loadMyMission()
        this.showNotification('Mission Completed')
      } catch (e: any) {
        const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'Error completing mission';
        this.showNotification(errorMessage)
      } finally {
        this.isLoading = false
      }
    }
  }

  async fail(missionId: number) {
    if (confirm('Are you sure you want to fail this mission?')) {
      try {
        this.isLoading = true
        await this._mission.fail(missionId)
        await new Promise(resolve => setTimeout(resolve, 300));
        await this.loadMyMission()
        this.showNotification('Mission Failed')
      } catch (e: any) {
        const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'Error failing mission';
        this.showNotification(errorMessage)
      } finally {
        this.isLoading = false
      }
    }
  }

  async leave(missionId: number) {
    try {
      this.isLoading = true
      // Optimistic update
      this.myMemberships = this.myMemberships.filter(id => id !== missionId);

      await this._mission.leave(missionId)
      await this.loadMyMission()
      await this.loadMyMemberships()
      this.showNotification('Left Mission')
    } catch (e: any) {
      // Rollback
      await this.loadMyMemberships();
      const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'An error occurred while leaving the mission';
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

  closeNotification() {
    this.notificationMessage = null
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

  isMember(missionId: number): boolean {
    return this.myMemberships.includes(missionId)
  }
}
