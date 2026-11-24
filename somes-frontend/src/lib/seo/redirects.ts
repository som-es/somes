import { SITE_URL } from './sitemap';

export const LEGACY_AT_ROUTES = [
	'history',
	'delegates',
	'decree',
	'gov_proposal',
	'vote_result',
	'statistics',
	'home'
];

export function with_parliament_prefix(pathname: string): string {
	const first = pathname.split('/').filter(Boolean)[0];
	return first && LEGACY_AT_ROUTES.includes(first) ? `/at${pathname}` : pathname;
}

export function redirect_location(url: URL, host: string | null): string | null {
	const path = with_parliament_prefix(url.pathname);
	const isWww = host?.toLowerCase().startsWith('www.') ?? false;

	if (isWww) return `${SITE_URL}${path}${url.search}`;
	if (path !== url.pathname) return `${path}${url.search}`;
	return null;
}
