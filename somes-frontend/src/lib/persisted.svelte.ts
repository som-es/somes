import { browser } from '$app/environment';

export function persisted<T>(key: string, defaultValue: T) {
	let internalState = $state<T>(defaultValue);

	if (browser) {
		const saved = localStorage.getItem(key);
		if (saved !== null) {
			try {
				internalState = JSON.parse(saved);
			} catch (e) {
				console.error(`Error parsing localStorage key "${key}":`, e);
			}
		}
	}

	return {
		get value() {
			return internalState;
		},
		set value(newValue: T) {
			internalState = newValue;
			if (browser) {
				localStorage.setItem(key, JSON.stringify(newValue));
			}
		}
	};
}