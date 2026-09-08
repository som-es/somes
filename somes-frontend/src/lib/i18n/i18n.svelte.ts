import { browser } from '$app/environment';
import { getParliament } from '$lib/api/parliament';
import { de } from './messages/de';
import { en } from './messages/en';
import type { Messages } from './messages/types';

export type Locale = 'de' | 'en';

const catalogs: Record<Locale, Messages> = { de, en };

const STORAGE_KEY = 'locale';

/**
 * The default locale depends on the parliament being rendered:
 * EU sites default to English, AT sites to German.
 *
 * It is computed inside the store getter (not at module scope) so that during
 * SSR it reads the *current request's* parliament via `getParliament()`
 * (which reads `page.params.parliament`, set by SvelteKit per request). This
 * means the EU site is server-rendered in English by default, which prevents
 * a German→English flash for the common case (no stored preference).
 */
function defaultLocale(): Locale {
	return getParliament() === 'eu' ? 'en' : 'de';
}

/**
 * Reactive locale store.
 *
 * Precedence: explicit user choice (stored in localStorage / set via
 * `setLocale`) > parliament-based default.
 *
 * The stored preference is read once at module scope on the browser (same
 * pattern as the previous `persisted` helper). On the server there is no
 * localStorage, so the getter falls back to the parliament-based default.
 */
let internalState = $state<Locale | null>(null);

if (browser) {
	const saved = localStorage.getItem(STORAGE_KEY);
	if (saved !== null) {
		try {
			const parsed = JSON.parse(saved);
			if (parsed === 'de' || parsed === 'en') internalState = parsed;
		} catch (e) {
			console.error(`Error parsing localStorage key "${STORAGE_KEY}":`, e);
		}
	}
}

export const localeStore = {
	get value(): Locale {
		return internalState ?? defaultLocale();
	},
	set value(newLocale: Locale) {
		internalState = newLocale;
		if (browser) {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(newLocale));
		}
	}
};

/**
 * Translate a message key for the current locale.
 *
 * Reading `localeStore.value` inside the function makes every call site in a
 * component reactive: when the locale changes, the rendered text updates.
 *
 * Optional `params` are interpolated into `{placeholders}` in the message.
 */
export function t<K extends keyof Messages>(
	key: K,
	params?: Record<string, string | number>
): string {
	const message = catalogs[localeStore.value][key] ?? '';
	if (!params) return message;
	return message.replace(/\{(\w+)\}/g, (_match, name: string) => {
		const value = params[name];
		return value === undefined ? '' : String(value);
	});
}

/** Return the current locale (reactive). */
export function getLocale(): Locale {
	return localeStore.value;
}

/** Set the current locale and persist it. */
export function setLocale(locale: Locale) {
	localeStore.value = locale;
}
