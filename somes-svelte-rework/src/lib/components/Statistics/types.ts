export type FilterOption = string | boolean | number | null | undefined;

export type FilterInfoStatistics = {
	id: string;
	name: string;
	isShown: boolean;
	infoText: string | null;
	multiple: boolean
	options?: FilterOption[]; 
	default?: FilterOption; 
	label?: (value: FilterOption) => string; 
};

export type Config = {
	filters: FilterInfoStatistics[];
};