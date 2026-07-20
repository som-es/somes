import { browser } from '$app/environment';
import { getParliament } from '$lib/api/parliament';

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

export function persistedScoped<T>(baseKey: string, defaultValue: T) {
	const slots: Record<string, ReturnType<typeof persisted<T>>> = {};

	function slot() {
		const key = `${getParliament()}:${baseKey}`;
		if (!slots[key]) {
			slots[key] = persisted<T>(key, defaultValue);
		}
		return slots[key];
	}

	return {
		get value(): T {
			if (!browser) {
				return defaultValue;
			}
			return slot().value;
		},
		set value(newValue: T) {
			if (!browser) {
				return;
			}
			slot().value = newValue;
		}
	};
}