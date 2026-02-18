import { ChangeDetectionStrategy, Component, inject, signal } from '@angular/core'
import { MatButtonModule } from '@angular/material/button'
import { MatDialogActions, MatDialogContent, MatDialogRef, MatDialogTitle } from '@angular/material/dialog'
import { FormsModule } from '@angular/forms'
import { MatIconModule } from '@angular/material/icon'

@Component({
  selector: 'app-upload-img',
  imports: [MatDialogTitle, MatDialogContent, MatDialogActions, MatButtonModule, FormsModule, MatIconModule],
  templateUrl: './upload-img.html',
  styleUrl: './upload-img.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UploadImg {
  acceptedMimeType = ['image/jpeg', 'image/png']
  imgFile: File | undefined
  imgPreview = signal<string | undefined>(undefined)
  errorMsg = signal<string | undefined>(undefined)
  uploadMode = signal<'file' | 'url'>('file')
  urlInput = signal<string>('')
  readonly _dialogRef = inject(MatDialogRef<UploadImg>)

  setMode(mode: 'file' | 'url') {
    this.uploadMode.set(mode)
    this.imgFile = undefined
    this.imgPreview.set(undefined)
    this.errorMsg.set(undefined)
    this.urlInput.set('')
  }

  onUrlChange(url: string) {
    this.urlInput.set(url)
    if (url) {
      this.imgPreview.set(url)
    } else {
      this.imgPreview.set(undefined)
    }
  }

  onSubmit() {
    if (this.uploadMode() === 'file') {
      this._dialogRef.close(this.imgFile)
    } else {
      this._dialogRef.close(this.urlInput())
    }
  }
  async onImgPicked(event: Event) {
    this.imgFile = undefined
    this.imgPreview.set(undefined)
    this.errorMsg.set(undefined)

    const input = event.target as HTMLInputElement
    if (input.files && input.files.length > 0) {
      const file = input.files[0]
      console.log('File picked:', file.name, file.type, file.size)

      const fileSizeMB = file.size / (1024 * 1024)
      if (fileSizeMB > 20) {
        this.errorMsg.set("Image file is too large (max 20MB)")
        return
      }

      const isImage = file.type.startsWith('image/')
      const isAccepted = ['image/jpeg', 'image/png'].includes(file.type) ||
        file.name.toLowerCase().endsWith('.jpg') ||
        file.name.toLowerCase().endsWith('.jpeg') ||
        file.name.toLowerCase().endsWith('.png')

      if (isImage && isAccepted) {
        this.imgFile = file
        const reader = new FileReader()
        reader.onerror = () => {
          this.imgFile = undefined
          this.errorMsg.set("System failed to read the image")
        }
        reader.onload = () => {
          this.imgPreview.set(reader.result as string)
        }
        reader.readAsDataURL(file)
      } else {
        this.errorMsg.set("Image file must be .jpg or .png")
      }
    }
  }
}
