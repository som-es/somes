export const VOTE_COLORS = {
	infavor: 'var(--color-green-600)',
	against: 'var(--color-red-500)',
	abstention: 'var(--color-blue-500)',
	absent: 'var(--color-gray-400)'
} as const;

export type SeatColorMode = 'party' | 'vote';
