import { SITE_URL } from './sitemap';

const CONTENT_PARAMS: Record<string, string[]> = {
	'/[parliament=parliament]/delegates': ['delegate'],
	'/[parliament=parliament]/questions': ['page'],
	'/[parliament=parliament]/history/votes': ['page'],
	'/[parliament=parliament]/history/unfinished_votes': ['page'],
	'/[parliament=parliament]/history/proposals': ['page'],
	'/[parliament=parliament]/history/decrees': ['page']
};

const NOT_INDEXABLE = new Set([
	'/user',
	'/admin/delegate-questions',
	'/resolve_token',
	'/[parliament=parliament]/questions/ask/[delegateId]',
	'/__eu_strasbourg_preview',
	'/__speechbar_preview'
]);

export function canonical_url(url: URL, routeId: string | null, status = 200): string | null {
	if (!routeId || status >= 400 || NOT_INDEXABLE.has(routeId)) return null;

	const query = new URLSearchParams();
	for (const key of CONTENT_PARAMS[routeId] ?? []) {
		const value = url.searchParams.get(key);
		if (value && !(key === 'page' && value === '1')) query.set(key, value);
	}

	const search = query.toString();
	return `${SITE_URL}${url.pathname}${search ? `?${search}` : ''}`;
}

export function delegate_canonical_url(parliament: string, delegateId: number | string): string {
	return `${SITE_URL}/${parliament}/delegates?delegate=${delegateId}`;
}
