import type { PopupSettings } from '@skeletonlabs/skeleton';

export interface FilterInfo<T> {
	title: string;
	popup: PopupSettings;
	attributeName: string;
	filterObj: T;
	translationFn: (x: FilterInfo<T>) => string | undefined;
	hidden: boolean;
	values: { title: string; value: T }[];
}

export function translationFn<T>(filterInfo: FilterInfo<T>): string | undefined {
	const title = filterInfo.values.find((value) => {
		return value.value == filterInfo.filterObj;
	})?.title;

	if (title !== undefined) {
		if (title.length > 15) {
			return `${title.slice(0, 15)}...`;
		}
		return title;
	} else {
		return undefined;
	}
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function translationFnAny(filter: any) {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	return (filter.translationFn as (f: any) => string)(filter);
}
