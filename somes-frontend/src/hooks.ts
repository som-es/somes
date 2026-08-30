import type { Reroute } from '@sveltejs/kit';

const DATA_ROUTES = new Set([
	'history',
	'delegates',
	'decree',
	'gov_proposal',
	'vote_result',
	'statistics',
	'home'
]);

export const reroute: Reroute = ({ url }) => {
	const segments = url.pathname.split('/').filter(Boolean);
	if (segments.length > 0 && DATA_ROUTES.has(segments[0])) {
		return `/at${url.pathname}`;
	}
};
