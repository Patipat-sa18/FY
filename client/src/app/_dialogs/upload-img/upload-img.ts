import { ChangeDetectionStrategy, Component, inject, signal } from '@angular/core'
import { MatButtonModule } from '@angular/material/button'
import { MatDialogActions, MatDialogContent, MatDialogRef, MatDialogTitle } from '@angular/material/dialog'
import { fileTypeFromBlob } from 'file-type'

@Component({
  selector: 'app-upload-img',
  imports: [MatDialogTitle, MatDialogContent, MatDialogActions, MatButtonModule],
  templateUrl: './upload-img.html',
  styleUrl: './upload-img.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UploadImg {
  acceptedMimeType = ['image/jpeg', 'image/png']
  imgFile: File | undefined
  imgPreview = signal<string | undefined>(undefined)
  errorMsg = signal<string | undefined>(undefined)
  private readonly _dialogRef = inject(MatDialogRef<UploadImg>)

  onSubmit() {
    this._dialogRef.close(this.imgFile)
  }
  async onImgPicked(event: Event) {
    this.imgFile = undefined
    this.imgPreview.set(undefined)
    this.errorMsg.set(undefined)

    const input = event.target as HTMLInputElement
    if (input.files && input.files.length > 0) {
      this.imgFile = input.files[0]
      const fileType = await fileTypeFromBlob(this.imgFile)
      if (fileType && this.acceptedMimeType.includes(fileType.mime)) {
        const reader = new FileReader()
        reader.onerror = () => {
          this.imgFile = undefined
          this.errorMsg.set("some thing went wrong")
        }
        reader.onload = () => {
          this.imgPreview.set(reader.result as string)
        }
        reader.readAsDataURL(this.imgFile)
      } else {
        this.imgFile = undefined
        this.errorMsg.set("image file must be .jpg or .png")
      }
    }
  }
}
