import { Brawler } from "../_models/brawler"
import { Passport } from "../_models/passport"

const _default_avatar = '/assets/default.avatar.jpg'

export function getAvatarUrl(passport: Passport | undefined): string {
    if (passport && passport.avatar_url) return passport.avatar_url
    return _default_avatar
}