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

@Component({
  selector: 'app-mission-manager',
  imports: [MatIconModule, DatePipe, AsyncPipe],
  templateUrl: './mission-manager.html',
  styleUrl: './mission-manager.scss',
})
export class MissionManager {
  private _mission = inject(MissionService)
  private _dialog = inject(MatDialog)
  private _passport = inject(PassportService)
  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly myMissions$ = this._missionsSubject.asObservable()

  constructor() {
    this.loadMyMission()
  }

  private async loadMyMission() {
    const missions = await this._mission.getMyMissions()
    this._missionsSubject.next(missions)
  }

  openDialog() {
    let chief_display_name = this._passport.data()?.display_name || "unnamed"
    const ref = this._dialog.open(NewMission)
    ref.afterClosed().subscribe(async (addMission: AddMission) => {
      if (addMission) {
        const id = await this._mission.add(addMission)
        const now = new Date()
        const newMission: Mission = {
          id,
          name: addMission.name,
          description: addMission.description,
          status: 'Open',
          chief_id: 0,
          chief_display_name,
          crew_count: 0,
          created_at: now,
          updated_at: now
        }
        // เพิ่มข้อมูลใหม่เข้าไปใน BehaviorSubject
        const currentMissions = this._missionsSubject.value
        this._missionsSubject.next([...currentMissions, newMission])
      }
    })
  }

}
