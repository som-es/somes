export type PlatformItemType = 'vote' | 'proposal' | 'decree';

export interface PlatformItem {
	id: number;
	path?: string;
	type: PlatformItemType;
	title: string;
	date: string;
	status?: 'accepted' | 'rejected' | 'pending';
}

export interface SomesEvent {
	id: number | null;
	title: string;
	location: string;
	date: string; // ISO format YYYY-MM-DD
	time: string;
	description: string;
	image?: string;
	requiresMembership?: boolean;
	requiresRegistration?: boolean;
}
