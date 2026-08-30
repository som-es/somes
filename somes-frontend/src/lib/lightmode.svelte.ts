import { persisted } from './persisted.svelte';

export type Theme = 'light' | 'dark';

export const lightModeStore = persisted<Theme>('theme', 'light');

export function toggleTheme() {
	lightModeStore.value = lightModeStore.value === 'dark' ? 'light' : 'dark';
}

/**
 * rsolves the initial theme (preference else OS)
 */
export function syncTheme() {
	if (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches) {
		lightModeStore.value = 'dark';
	}
	document.documentElement.classList.toggle('dark', lightModeStore.value === 'dark');
	document.cookie = `theme=${lightModeStore.value}; path=/; expires=${new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toUTCString()}`;
}
