import { HttpInterceptorFn } from '@angular/common/http'
import { inject } from '@angular/core'
import { LoadingService } from '../_services/loading-service'
import { delay, finalize } from 'rxjs'

export const loadingInterceptor: HttpInterceptorFn = (req, next) => {
  const spinner = inject(LoadingService)
  spinner.loading()
  return next(req).pipe(
    // delay(2000),
    finalize(() => spinner.idle())
  )
}
