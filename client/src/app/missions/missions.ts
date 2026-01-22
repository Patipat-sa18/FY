import { Component, inject } from '@angular/core'
import { MissionService } from '../_services/mission-service'
import { MissionFilter } from '../_models/mission-filter'
import { Mission } from '../_models/mission'
import { FormsModule } from '@angular/forms'

@Component({
  selector: 'app-missions',
  imports: [FormsModule],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions {
  private _mission = inject(MissionService)
  filter: MissionFilter = {}
  missions: Mission[] = []

  constructor() {
    this.filter = this._mission.filter
  }

  async onSubmit() {
    this.missions = await this._mission.getByFilter(this.filter)
  }
}
