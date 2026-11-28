import {
	delegate_by_id,
	errorToNull,
	general_delegate_info,
	general_gov_official_info,
	interjections_made_by_delegate_per_page,
	interjections_received_by_delegate_per_page,
	parties_per_gp,
	speeches_by_delegate_per_page,
	toActualDateString
} from '$lib/api/api';
import { defaultGpByParliament, type Parliament } from '$lib/api/parliament';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
import { cachedAllSeats } from '$lib/caching/seats';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { or_error } from '$lib/api/load_error';
import { cachedPartyColors } from '$lib/caching/party_color';
import { cachedPlenarySessions } from '$lib/caching/plenarySessions';
import { getMandateLatestPeriod } from './searchDelegates';

export const load: PageServerLoad = async ({ fetch, url, setHeaders, params, locals }) => {
	const parliament = params.parliament as Parliament;
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=120'
		});
	}
	const delegateId = url.searchParams.get('delegate');
	let gp = url.searchParams.get('gp');
	let date = url.searchParams.get('date');

	const cachedPeriods = (await cachedAllLegisPeriods(true, fetch, parliament))?.reverse();

	let delegate = null;

	if (delegateId) {
		if (!/^\d+$/.test(delegateId)) error(404, 'Not found');
		delegate = or_error(await delegate_by_id(+delegateId, fetch, parliament));
	}

	if (delegate && !gp && !date && cachedPeriods) {
		const latest = getMandateLatestPeriod(delegate, cachedPeriods);
		gp = latest.gp;
		date = toActualDateString(latest.date);
	}

	const detailsLocale = locals.locale ?? (parliament === 'eu' ? 'en' : 'de');
	const details = delegate
		? Promise.all([
				general_delegate_info(delegate.id, detailsLocale, fetch, parliament).then(errorToNull),
				general_gov_official_info(delegate.id, fetch, parliament).then(errorToNull),
				speeches_by_delegate_per_page(delegate.id, 0, fetch, parliament).then(errorToNull),
				interjections_made_by_delegate_per_page(delegate.id, 0, fetch, parliament).then(
					errorToNull
				),
				interjections_received_by_delegate_per_page(delegate.id, 0, fetch, parliament).then(
					errorToNull
				)
			])
		: Promise.resolve([null, null, null, null, null] as const);

	const delegates = await fetchDelegates(
		date ?? new Date().toISOString().split('T')[0],
		gp ?? defaultGpByParliament(parliament),
		fetch,
		parliament
	);
	const [
		generalDelegateInfo,
		generalGovOfficialInfo,
		speechesPage0,
		interjectionsMadePage0,
		interjectionsReceivedPage0
	] = await details;
	generalDelegateInfo?.interests.sort((a, b) => b.self_share - a.self_share);
	generalDelegateInfo?.detailed_interests.sort((a, b) => b.self_share - a.self_share);

	const cachedSeats = await cachedAllSeats(true, fetch, parliament);
	const partiesPerGp = errorToNull(await parties_per_gp(fetch, parliament));
	const partyColors = await cachedPartyColors(true, parliament, fetch);

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
		partyColors,
		detailsLocale,
		generalDelegateInfo,
		generalGovOfficialInfo,
		speechesPage0,
		interjectionsMadePage0,
		interjectionsReceivedPage0
	};
};
