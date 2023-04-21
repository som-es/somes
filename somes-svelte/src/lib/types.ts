export interface Delegate {
    id: number,
    name: string,
    party: string,
    constituency: string,
    council: string,
    seat_row: number | null,
    seat_col: number | null,
}