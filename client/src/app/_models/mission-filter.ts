export interface MissionFilter {
    name?: string
    status?: MissionStatus
}

export type MissionStatus =
    'Open' |
    'InProgress' |
    'Completed' |
    'Failed'