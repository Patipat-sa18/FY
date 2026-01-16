import { inject } from '@angular/core'
import { CanActivateFn, Router } from '@angular/router'
import { PassportService } from '../_services/passport-service'

export const authGuard: CanActivateFn = (route, state) => {
  const passport = inject(PassportService)
  const router = inject(Router)
  if (passport.data()?.token)
    return true
  router.navigate(['/not-found'])
  return false
}
