import { HttpInterceptorFn } from '@angular/common/http'
import { inject } from '@angular/core'
import { ErrorService } from '../_services/error-service'
import { catchError } from 'rxjs'

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
  const error = inject(ErrorService)
  return next(req).pipe(catchError(e => error.handleError(e)))
}
