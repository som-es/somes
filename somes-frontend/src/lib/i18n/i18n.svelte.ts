import { persisted } from '$lib/persisted.svelte';
import { de } from './messages/de';
import { en } from './messages/en';
import type { Messages } from './messages/types';

export type Locale = 'de' | 'en';

/**
 * Reactive locale store, persisted to localStorage (same pattern as
 * `lightModeStore`). German is the default because it was the original
 * language of the application.
 */
export const localeStore = persisted<Locale>('locale', 'de');

const catalogs: Record<Locale, Messages> = { de, en };

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
