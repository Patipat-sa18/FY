import { ApplicationRef, ComponentRef, createComponent, EnvironmentInjector, inject, Injectable } from '@angular/core'
import { Spinner } from '../_components/spinner/spinner'

@Injectable({
  providedIn: 'root',
})
export class LoadingService {
  loadingRequestCount = 0
  private _componentRef: ComponentRef<Spinner> | null = null
  private _appRef = inject(ApplicationRef)
  private _injector = inject(EnvironmentInjector)

  loading() {
    this.loadingRequestCount++
    if (this.loadingRequestCount !== 1) return
    if (!this._componentRef) {
      this._componentRef = createComponent(Spinner, {
        environmentInjector: this._injector
      })
    }
    document.body.appendChild(this._componentRef.location.nativeElement)
    this._appRef.attachView(this._componentRef.hostView)
    this._componentRef.instance.show()
  }

  idle() {
    this.loadingRequestCount--
    if (this.loadingRequestCount <= 0) {
      this.loadingRequestCount = 0
      if (!this._componentRef) return
      this._componentRef.instance.hide()
      this._appRef.detachView(this._componentRef.hostView)
      this._componentRef.location.nativeElement.remove()
      this._componentRef.destroy()
      this._componentRef = null
    }
  }
}
