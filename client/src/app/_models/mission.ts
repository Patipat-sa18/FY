export interface Mission {
    id: number
    name: string
    description?: string
    difficulty: string
    status: string
    chief_id: number
    chief_display_name: string
    crew_count: number
    max_crew: number
    created_at: Date
    updated_at: Date
}