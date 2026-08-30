import type { GenericFilterGroup } from './types';

export function createFilterGroup<T extends string | boolean>(config: {
	title: () => string;
	hidden: () => boolean;
	options: () => { title: string; value: T | undefined }[];
	initialValue?: T;
	disabled?: () => boolean | undefined;
	disabledText?: () => string | undefined;
	advanced?: boolean;
	id?: string;
	data?: Record<string, string>;
}): GenericFilterGroup<T> {
	let activeValue = $state<T | undefined>(config.initialValue);

	return {
		get activeValue() {
			return activeValue;
		},
		set activeValue(v) {
			activeValue = v;
		},
		get title() {
			return config.title();
		},
		get hidden() {
			return config.hidden();
		},
		get options() {
			return config.options();
		},
		get disabled() {
			return config.disabled?.();
		},
		get disabledText() {
			return config.disabledText?.();
		},
		advanced: config.advanced,
		id: config.id,
		data: config.data
	};
}
