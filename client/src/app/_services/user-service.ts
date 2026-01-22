import { inject, Injectable } from '@angular/core'
import { environment } from '../../environments/environment'
import { HttpClient } from '@angular/common/http'
import { PassportService } from './passport-service'
import { fileToBase64 } from '../_helpers/file'
import { firstValueFrom } from 'rxjs'
import { CloudinaryImage } from '../_models/cludinary-image'

@Injectable({
  providedIn: 'root',
})
export class UserService {
  private _base_url = environment.baseUrl + '/api/brawler'
  private _http = inject(HttpClient)
  private _passport = inject(PassportService)

  async uploadAvatarImg(file: File): Promise<string | null> {
    const url = this._base_url + '/avatar'
    const base64string = await fileToBase64(file)
    const uploadImg = {
      'base64_string': base64string.split(',')[1]
    }
    try {
      // console.log(uploadImg.base64_string)
      const cloudinaryImg = await firstValueFrom(this._http.post<CloudinaryImage>(url, uploadImg))
      this._passport.saveAvatarImgUrl(cloudinaryImg.url)
    } catch (error: any) {
      return error.error as string
    }
    return null
  }
}
