import { Component, computed, inject, Signal } from '@angular/core'
import { MissionService } from '../_services/mission-service'
import { MissionFilter } from '../_models/mission-filter'
import { Mission } from '../_models/mission'
import { FormsModule } from '@angular/forms'
import { BehaviorSubject } from 'rxjs'
import { AsyncPipe, DatePipe } from '@angular/common'
import { PassportService } from '../_services/passport-service'

@Component({
  selector: 'app-missions',
  imports: [FormsModule, AsyncPipe, DatePipe],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions {
  private _mission = inject(MissionService)
  private _passport = inject(PassportService)
  filter: MissionFilter = {}
  // missions: Mission[] = []

  private _missionsSubject = new BehaviorSubject<Mission[]>([])
  readonly missions$ = this._missionsSubject.asObservable()
  isSignin: Signal<boolean>

  constructor() {
    this.isSignin = computed(() => this._passport.data() !== undefined)
    this.filter = this._mission.filter
    this.loadMyMission()
  }

  private async loadMyMission() {
    const missions = await this._mission.getByFilter(this.filter)
    this._missionsSubject.next(missions)
  }
  async onSubmit() {
    this.loadMyMission()
  }
}
