import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { delegate_by_id, isHasError, toActualDateString } from '$lib/api/api';
import { is_not_found } from '$lib/api/load_error';
import type { Parliament } from '$lib/api/parliament';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
import { cachedPartyColors } from '$lib/caching/party_color';
import { de } from '$lib/i18n/messages/de';
import { en } from '$lib/i18n/messages/en';
import { render_share_image, SHARE_ICONS } from '$lib/server/share_image';
import { share_person } from '$lib/server/share_person';
import { png_response } from '$lib/server/share_response';
import { getMandateLatestPeriod } from '../../../../[parliament=parliament]/delegates/searchDelegates';
import DelegateShareCard from '$lib/components/ShareCards/DelegateShareCard.svelte';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const parliament = params.parliament as Parliament;
	if (!/^\d+$/.test(params.id)) error(404, 'Not found');
	const byId = await delegate_by_id(+params.id, fetch, parliament);
	if (isHasError(byId)) error(is_not_found(byId) ? 404 : 503, 'Not available');

	const messages = parliament === 'eu' ? en : de;
	const periods = ((await cachedAllLegisPeriods(true, fetch, parliament)) ?? []).reverse();
	const latest = getMandateLatestPeriod(byId, periods);
	const [{ delegates }, partyColors] = await Promise.all([
		fetchDelegates(toActualDateString(latest.date), latest.gp, fetch, parliament),
		cachedPartyColors(true, parliament, fetch)
	]);
	const delegate = delegates?.find((d) => d.id === byId.id) ?? byId;

	const png = await render_share_image(DelegateShareCard, {
		icons: SHARE_ICONS,
		heading: messages[parliament === 'eu' ? 'delegates.title.eu' : 'delegates.title.at'],
		person: await share_person(delegate, {
			parliament,
			partyColors,
			messages,
			fetcher: fetch,
			extraLines: [
				...(delegate.constituency != null ? [delegate.constituency] : []),
				...(delegate.divisions?.length ? [delegate.divisions.join(', ')] : [])
			]
		})
	});
	return png_response(png);
};
