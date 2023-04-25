export interface Delegate {
    id: number,
    name: string,
    party: string,
    image_url: string,
    constituency: string,
    council: string,
    seat_row: number | null,
    seat_col: number | null,
}

export interface LegislativeInitiative {
    id: string,
    ityp: string,
    gp: string,
    title: string,
    description: string,
    accepted: boolean,
    created_at: Date
}

export interface Vote {
    party: string,
    fraction: number,
    infavor: boolean,
    legislative_initiatives_id: string,
}

export interface VoteResult {
    legislative_initiative: LegislativeInitiative,
    votes: Vote[],
}
