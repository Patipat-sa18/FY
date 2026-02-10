import { inject, Injectable } from '@angular/core'
import { environment } from '../../environments/environment'
import { HttpClient } from '@angular/common/http'
import { MissionFilter } from '../_models/mission-filter'
import { firstValueFrom } from 'rxjs'
import { Mission } from '../_models/mission'
import { AddMission } from '../_models/add-mission'

@Injectable({
  providedIn: 'root',
})
export class MissionService {
  ///view
  private _base_url = environment.baseUrl + '/api'
  private _http = inject(HttpClient)

  filter: MissionFilter = {}

  async getByFilter(filter: MissionFilter): Promise<Mission[]> {
    const queryString = this.createQueryString(filter)
    const url = this._base_url + '/view/filter?' + queryString
    const missions = await firstValueFrom(this._http.get<Mission[]>(url))
    return missions
  }

  private createQueryString(filter: MissionFilter): string {
    this.filter = filter
    const params: string[] = []

    if (filter.name && filter.name.trim()) {
      params.push(`name=${encodeURIComponent(filter.name.trim())}`)
    }
    if (filter.status) {
      params.push(`status=${encodeURIComponent(filter.status)}`)
    }

    return params.join("&")
  }

  async add(mission: AddMission): Promise<number> {
    const url = this._base_url + '/mission-management'
    const observable = this._http.post<{ mission_id: number }>(url, mission)
    const resp = await firstValueFrom(observable)
    return resp.mission_id
  }

  async getMyMissions(): Promise<Mission[]> {
    const url = this._base_url + '/brawler/my-missions'
    console.log('get ' + url)
    const observable = this._http.get<Mission[]>(url)
    const missions = await firstValueFrom(observable)
    return missions
  }

  async join(missionId: number): Promise<string> {
    const url = this._base_url + `/crew/join/${missionId}`
    return await firstValueFrom(this._http.post(url, {}, { responseType: 'text' }))
  }

  async edit(missionId: number, data: Partial<Mission>): Promise<string> {
    const url = this._base_url + `/mission-management/${missionId}`
    return await firstValueFrom(this._http.patch(url, data, { responseType: 'text' }))
  }

  async delete(missionId: number): Promise<string> {
    const url = this._base_url + `/mission-management/${missionId}`
    return await firstValueFrom(this._http.delete(url, { responseType: 'text' }))
  }


}
