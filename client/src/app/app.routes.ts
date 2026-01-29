import { Routes } from '@angular/router'
import { Home } from './home/home'
import { Login } from './login/login'
import { Profile } from './profile/profile'
import { ServerError } from './server-error/server-error'
import { NotFound } from './not-found/not-found'
import { authGuard } from './_guard/auth-guard'
import { Missions } from './missions/missions'
import { MissionManager } from './missions/mission-manager/mission-manager'

export const routes: Routes = [
    { path: '', component: Home },
    { path: 'login', component: Login },
    { path: 'profile', component: Profile, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    { path: 'missions', component: Missions, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    {
        path: 'chief',
        component: MissionManager,
        runGuardsAndResolvers: 'always',
        canActivate: [authGuard]
    },

    { path: 'server-error', component: ServerError },
    { path: '**', component: NotFound },
]
