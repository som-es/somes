import { getParliament } from '$lib/api/parliament';
import { t } from '$lib/i18n/i18n.svelte';

export function shorten(text: string, max = 160): string {
	const clean = text.replace(/\s+/g, ' ').trim();
	if (clean.length <= max) return clean;
	return `${clean.slice(0, clean.lastIndexOf(' ', max - 1))}…`;
}

export function parliament_name(): string {
	return t(getParliament() === 'eu' ? 'seo.parliament.eu' : 'seo.parliament.at');
}
