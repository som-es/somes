export type FilterInfoStatistics = {
	id: 'primary' | 'gender' | 'normalized' | 'desc';
	name: string;
	isShown: boolean;
	infoText: string | null;
};

export type Config = {
	filters: FilterInfoStatistics[];
};