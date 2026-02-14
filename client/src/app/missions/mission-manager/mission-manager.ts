import { Component, inject } from '@angular/core'
import { MissionService } from '../../_services/mission-service'
import { MatDialog } from '@angular/material/dialog'
import { Mission } from '../../_models/mission'
import { NewMission } from '../../_dialogs/new-mission/new-mission'
import { AddMission } from '../../_models/add-mission'
import { MatIconModule } from '@angular/material/icon'
import { AsyncPipe, DatePipe } from '@angular/common'
import { BehaviorSubject } from 'rxjs'
import { PassportService } from '../../_services/passport-service'
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar'
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner'

import { FormsModule } from '@angular/forms'

@Component({
  selector: 'app-mission-manager',
  imports: [MatIconModule, DatePipe, AsyncPipe, FormsModule, MatSnackBarModule, MatProgressSpinnerModule],
  templateUrl: './mission-manager.html',
  styleUrl: './mission-manager.scss',
})
export class MissionManager {
  private _mission = inject(MissionService)
  private _dialog = inject(MatDialog)
  private _passport = inject(PassportService)
  private _snackBar = inject(MatSnackBar)
  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly myMissions$ = this._missionsSubject.asObservable()

  isLoading = false

  constructor() {
    this.loadMyMission()
  }

  private async loadMyMission() {
    try {
      this.isLoading = true
      const missions = await this._mission.getMyMissions()
      this._missionsSubject.next(missions)
    } finally {
      this.isLoading = false
    }
  }

  notificationMessage: string | null = null

  showNotification(message: string) {
    this.notificationMessage = message
    setTimeout(() => {
      this.notificationMessage = null
    }, 2000)
  }

  openDialog() {
    let chief_display_name = this._passport.data()?.display_name || "unnamed"
    const ref = this._dialog.open(NewMission)
    ref.afterClosed().subscribe(async (addMission: AddMission) => {
      if (addMission) {
        try {
          this.isLoading = true
          await this._mission.add(addMission)
          await new Promise(resolve => setTimeout(resolve, 300));
          await this.loadMyMission()
          this.showNotification('Completed')
        } catch (e: any) {
          const errorMessage = e.error?.error || e.error?.message || e.error || e.message || 'Error creating mission';
          this.showNotification(errorMessage)
        } finally {
          this.isLoading = false
        }
      }
    })
  }

  // Edit/Delete Logic
  editingId?: number
  editForm: Partial<Mission> = {}

  startEdit(mission: Mission) {
    this.editingId = mission.id
    this.editForm = {
      name: mission.name,
      description: mission.description,
      max_crew: mission.max_crew,
      difficulty: mission.difficulty
    }
  }

  cancelEdit() {
    this.editingId = undefined
    this.editForm = {}
  }

  async saveEdit() {
    if (this.editingId && this.editForm) {
      try {
        this.isLoading = true
        // Ensure max_crew is a number if present
        if (this.editForm.max_crew) {
          this.editForm.max_crew = Number(this.editForm.max_crew);
        }

        const message = await this._mission.edit(this.editingId, this.editForm)
        this.editingId = undefined
        // Reload data to reflect changes
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

  async start(missionId: number) {
    if (confirm('Are you sure you want to start this mission?')) {
      try {
        this.isLoading = true
        const message = await this._mission.start(missionId)
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

  async deleteMission(missionId: number) {
    if (confirm('Are you sure you want to delete this mission?')) {
      try {
        this.isLoading = true
        const message = await this._mission.delete(missionId)
        await new Promise(resolve => setTimeout(resolve, 300));
        await this.loadMyMission()
        this.showNotification('Completed')
      } catch (e: any) {
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
