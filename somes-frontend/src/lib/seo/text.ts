import { getParliament } from '$lib/api/parliament';
import { t } from '$lib/i18n/i18n.svelte';

export function shorten(text: string, max = 160): string {
	const clean = text.replace(/\s+/g, ' ').trim();
	if (clean.length <= max) return clean;
	return `${clean.slice(0, clean.lastIndexOf(' ', max - 1))}…`;
}

export function meta_description(text: string, max = 155): string {
	const clean = text.replace(/\s+/g, ' ').trim();
	if (clean.length <= max) return clean;
	const head = clean.slice(0, max + 1);
	const sentenceEnds = [...head.matchAll(/[a-zäöüß)“"]{2}[.!?](?=\s)/g)];
	const sentenceEnd = sentenceEnds.at(-1);
	if (sentenceEnd?.index !== undefined) return clean.slice(0, sentenceEnd.index + 3);
	return shorten(clean, max);
}

export function parliament_name(): string {
	return t(getParliament() === 'eu' ? 'seo.parliament.eu' : 'seo.parliament.at');
}
