export interface ShareParty {
	name: string;
	donut: string | null;
	inFavor: boolean;
}

export interface ShareCounts {
	inFavor: number;
	against: number;
	abstention: number;
}

export interface ShareImage {
	src: string;
	width: number;
	height: number;
}

export interface ShareIcons {
	logo: string;
	check: string;
	cross: string;
	minus: string;
}

export interface SharePersonData {
	name: string;
	age: number | null;
	party: string;
	partyColor: string;
	photo: string | null;
	copyright: string | null;
	lines: string[];
}
