import { HttpClient } from '@angular/common/http'
import { inject, Injectable, signal } from '@angular/core'
import { environment } from '../../environments/environment' ///
import { LoginModel, Passport, RegisterModel } from '../_models/passport'
import { firstValueFrom } from 'rxjs'
import { H } from '@angular/cdk/keycodes'
import { getAvatarUrl } from '../_helpers/util'

@Injectable({
  providedIn: 'root',
})
export class PassportService {
  private _key = 'passport'
  private _base_url = environment.baseUrl + '/api'
  private _http = inject(HttpClient)

  data = signal<undefined | Passport>(undefined)
  avatar = signal<string>("")

  saveAvatarImgUrl(url: string) {
    let passport = this.data()
    if (passport) {
      this.data.set({ ...passport, avatar_url: url })
      this.avatar.set(url)
      this.savePassportToLocalStorage()
    }
  }

  updateDisplayName(name: string) {
    let passport = this.data()
    if (passport) {
      this.data.set({ ...passport, display_name: name })
      this.savePassportToLocalStorage()
    }
  }

  private loadPassportFormLocalStorage(): string | null {
    const jsonString = localStorage.getItem(this._key)
    if (!jsonString) return 'not found'
    try {
      const passport = JSON.parse(jsonString) as Passport
      this.data.set(passport)
      const avatar = getAvatarUrl(passport)
      this.avatar.set(avatar)
    } catch (error) {
      return `${error}`
    }
    return null
  }

  private savePassportToLocalStorage() {
    const passport = this.data()
    if (!passport) return
    const jsonString = JSON.stringify(passport)
    localStorage.setItem(this._key, jsonString)
  }

  constructor() {
    if (this.loadPassportFormLocalStorage() === 'not found') {
      this.checkSession()
    }
  }

  async checkSession() {
    try {
      const url = this._base_url + '/authentication/me'
      const passport = await firstValueFrom(this._http.get<Passport>(url))
      if (passport) {
        this.data.set(passport)
        const avatar = getAvatarUrl(passport)
        this.avatar.set(avatar)
        this.savePassportToLocalStorage()
      }
    } catch (e) {
      // No active session in cookie
    }
  }

  destroy() {
    this.data.set(undefined)
    this.avatar.set("")
    localStorage.removeItem(this._key)
  }

  async get(login: LoginModel): Promise<null | string> {
    const api_url = this._base_url + '/authentication/login'
    return await this.fetchPassport(api_url, login)
  }

  async register(register: RegisterModel): Promise<null | string> {
    const api_url = this._base_url + '/brawler/register'
    try {
      await firstValueFrom(this._http.post(api_url, register))
      return null
    } catch (error: any) {
      return error.error
    }
  }

  private async fetchPassport(api_url: string, model: LoginModel | RegisterModel): Promise<string | null> {
    try {
      const result = this._http.post<Passport>(api_url, model)
      const passport = await firstValueFrom(result)
      this.data.set(passport)
      this.savePassportToLocalStorage()
      return null
    } catch (error: any) {
      // console.error(error)
      // console.log(error.error)
      return error.error
    }

  }

}
