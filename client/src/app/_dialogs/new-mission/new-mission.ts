import { Component, inject } from '@angular/core'
import { AddMission } from '../../_models/add-mission'
import { MatDialogActions, MatDialogClose, MatDialogContent, MatDialogRef, MatDialogTitle } from '@angular/material/dialog'
import { MatButtonModule } from '@angular/material/button'
import { MatIconModule } from '@angular/material/icon'
import { FormsModule } from '@angular/forms'

@Component({
  selector: 'app-new-mission',
  imports: [MatDialogTitle, MatDialogContent, MatDialogActions, MatDialogClose, MatButtonModule, MatIconModule, FormsModule],
  templateUrl: './new-mission.html',
  styleUrl: './new-mission.scss',
})
export class NewMission {
  addMission: AddMission = {
    name: '',
    description: ''
  }
  private readonly _dialogRef = inject(MatDialogRef<NewMission>)

  onSubmit() {
    const mission = this.clean(this.addMission)
    this._dialogRef.close(mission)
  }

  private clean(addMission: AddMission): AddMission {
    return {
      name: addMission.name.trim() || 'untitled',
      description: addMission.description?.trim() || undefined
    }
  }
}

