import { persisted } from './persisted.svelte';

export type Theme = 'light' | 'dark';

export const lightModeStore = persisted<Theme>('theme', 'light');

export function toggleTheme() {
	lightModeStore.value = lightModeStore.value === 'dark' ? 'light' : 'dark';
}

/**
 * Keeps the `<html class="dark">` and the `theme` cookie (used for SSR) in sync with
 * the store. Call from a `$effect` in the root layout so it re-runs on every change.
 *
 * The OS preference (`prefers-color-scheme`) is intentionally not consulted for now.
 */
export function syncTheme() {
	document.documentElement.classList.toggle('dark', lightModeStore.value === 'dark');
	document.cookie = `theme=${lightModeStore.value}; path=/; expires=${new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toUTCString()}`;
}
