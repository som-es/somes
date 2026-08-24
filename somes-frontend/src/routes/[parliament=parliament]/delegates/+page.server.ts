import { delegate_by_id, errorToNull, parties_per_gp } from '$lib/api/api';
import { defaultGpByParliament, type Parliament } from '$lib/api/parliament';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
import { cachedAllSeats } from '$lib/caching/seats';
import type { PageServerLoad } from './$types';
import { cachedPartyColors } from '$lib/caching/party_color';

export const load: PageServerLoad = async ({ fetch, url, setHeaders, params }) => {
	const parliament = params.parliament as Parliament;
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=120'
		});
	}
	const delegateId = url.searchParams.get('delegate');
	const gp = url.searchParams.get('gp');
	const date = url.searchParams.get('date');

	const delegates = await fetchDelegates(
		date ?? new Date().toISOString().split('T')[0],
		gp ?? defaultGpByParliament(parliament),
		fetch,
		parliament
	);
	const cachedPeriods = (await cachedAllLegisPeriods(true, fetch, parliament))?.reverse();
	const cachedSeats = await cachedAllSeats(true, fetch, parliament);
	const partiesPerGp = errorToNull(await parties_per_gp(fetch, parliament));
	const partyColors = await cachedPartyColors(true, parliament, fetch);

	let delegate = null;

	if (delegateId) {
		delegate = errorToNull(await delegate_by_id(+delegateId, fetch, parliament));
	}

	return {
		...delegates,
		delegate,
		delegateId,
		cachedPeriods,
		gp,
		cachedSeats,
		date,
		partiesPerGp,
		parliament,
		partyColors
	};
};
