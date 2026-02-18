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
  private _safetyTimeout: any = null

  loading() {
    this.loadingRequestCount++
    console.log(`[LoadingService] Request count: ${this.loadingRequestCount}`)

    if (this.loadingRequestCount !== 1) return

    this.showSpinner()
    this.startSafetyTimeout()
  }

  private showSpinner() {
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
    console.log(`[LoadingService] Request count: ${this.loadingRequestCount}`)

    if (this.loadingRequestCount <= 0) {
      this.forceIdle()
    }
  }

  private forceIdle() {
    console.log('[LoadingService] Forcing idle state and removing spinner')
    this.loadingRequestCount = 0
    this.clearSafetyTimeout()

    if (!this._componentRef) return
    this._componentRef.instance.hide()
    this._appRef.detachView(this._componentRef.hostView)
    this._componentRef.location.nativeElement.remove()
    this._componentRef.destroy()
    this._componentRef = null
  }

  private startSafetyTimeout() {
    this.clearSafetyTimeout()
    this._safetyTimeout = setTimeout(() => {
      if (this.loadingRequestCount > 0) {
        console.warn('[LoadingService] Safety timeout reached. Clearing spinner...')
        this.forceIdle()
      }
    }, 10000) // 10 seconds safety timeout
  }

  private clearSafetyTimeout() {
    if (this._safetyTimeout) {
      clearTimeout(this._safetyTimeout)
      this._safetyTimeout = null
    }
  }
}
