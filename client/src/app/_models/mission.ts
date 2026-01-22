export interface Mission {
    id: number,
    name: string,
    description?: string,
    status: string,
    chief_id: number,
    crew_count: number,
    created_at: Date,
    updated_at: Date,
}