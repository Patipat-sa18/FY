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
      console.log('Uploading avatar...', { fileName: file.name, fileSize: file.size });
      const base64Content = base64string.split(',')[1]
      console.log('Base64 content length:', base64Content?.length);

      if (!base64Content) {
        throw new Error('Failed to extract base64 content from file');
      }

      const cloudinaryImg = await firstValueFrom(this._http.post<CloudinaryImage>(url, { base64_string: base64Content }))
      console.log('Upload successful:', cloudinaryImg.url);
      this._passport.saveAvatarImgUrl(cloudinaryImg.url)
    } catch (error: any) {
      console.error('Upload failed details:', error);
      return error.error?.message || error.error || error.message || 'Upload failed'
    }
    return null
  }

  async updateAvatarUrl(url: string): Promise<string | null> {
    const endpoint = this._base_url + '/avatar-url'
    try {
      const response = await firstValueFrom(this._http.post(endpoint, { url: url }, { responseType: 'text' }))
      console.log('Avatar URL update response:', response)
      this._passport.saveAvatarImgUrl(url)
      return null
    } catch (error: any) {
      console.error('Failed to update avatar URL:', error)
      return error.error || error.message || 'Failed to update avatar URL'
    }
  }

  async getProfileStats(): Promise<import('../_models/profile-stats').ProfileStats | null> {
    const url = this._base_url + '/stats'
    try {
      const stats = await firstValueFrom(this._http.get<import('../_models/profile-stats').ProfileStats>(url))
      return stats
    } catch (error) {
      console.error('Failed to fetch stats:', error)
      return null
    }
  }

  async updateUsername(newUsername: string): Promise<string | null> {
    const url = this._base_url + '/username'
    try {
      const response = await firstValueFrom(this._http.post(url, { new_username: newUsername }, { responseType: 'text' }))
      console.log('Username update response:', response)
      return null
    } catch (error: any) {
      console.error('Failed to update username:', error)
      return error.error || error.message || 'Failed to update username'
    }
  }
}
