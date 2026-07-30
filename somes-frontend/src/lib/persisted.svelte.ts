import { browser } from '$app/environment';
import { getParliament, type Parliament } from '$lib/api/parliament';

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

	function slot(scope: Parliament = getParliament()) {
		const key = `${scope}:${baseKey}`;
		if (!slots[key]) {
			slots[key] = persisted<T>(key, defaultValue);
		}
		return slots[key];
	}

	return {
		get value(): T {
            return this.valueScoped()
		},
		set value(newValue: T) {
			if (!browser) {
				return;
			}
			slot().value = newValue;
		},
		valueScoped(parliament: Parliament = getParliament()): T {
    		if (!browser) {
    			return defaultValue;
    		}
    		return slot(parliament).value;
	    }
	};
}
