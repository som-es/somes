export type FilterOption = string | boolean | number;

export type FilterInfoStatistics<T=FilterOption|FilterOption[]> = {
	id: string;
	name: string;
	isShown: boolean;
	infoText: string | null;
	multiple: boolean
	options: FilterOption[]; 
	default?: T; 
	label?: (value: T) => string; 
};

export type Config = {
	filters: FilterInfoStatistics[];
};