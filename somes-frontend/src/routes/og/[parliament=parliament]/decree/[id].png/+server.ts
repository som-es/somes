import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { delegate_by_id, isHasError } from '$lib/api/api';
import { is_not_found } from '$lib/api/load_error';
import type { Parliament } from '$lib/api/parliament';
import { cachedPartyColors } from '$lib/caching/party_color';
import { decree_by_ris_id } from '$lib/components/Delegates/Decrees/api';
import { ministerialInfoBadges } from '$lib/components/MinisterialView/infoBadges';
import { viennaDate } from '$lib/date';
import { de } from '$lib/i18n/messages/de';
import { en } from '$lib/i18n/messages/en';
import { render_share_image, SHARE_ICONS } from '$lib/server/share_image';
import { share_person } from '$lib/server/share_person';
import { png_response } from '$lib/server/share_response';
import MinisterialShareCard from '$lib/components/ShareCards/MinisterialShareCard.svelte';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const parliament = params.parliament as Parliament;
	const decree = await decree_by_ris_id(params.id, fetch, parliament);
	if (isHasError(decree)) error(is_not_found(decree) ? 404 : 503, 'Not available');

	const messages = parliament === 'eu' ? en : de;
	const [delegate, partyColors] = await Promise.all([
		delegate_by_id(decree.gov_official_id, fetch, parliament),
		cachedPartyColors(true, parliament, fetch)
	]);
	const date = decree.publication_date;

	const png = await render_share_image(MinisterialShareCard, {
		icons: SHARE_ICONS,
		heading: messages['history.decrees.title'],
		title: decree.ai_summary?.short_title || decree.short_title,
		subtitle: `Verordnung vom ${viennaDate(date)}`,
		summaryHeading: messages['ministerialView.summary'],
		summary: decree.ai_summary?.short_summary ?? '',
		badges: ministerialInfoBadges(decree.ministrial_issuer, date, decree.gp, 'de-AT'),
		person: isHasError(delegate)
			? null
			: await share_person(delegate, { parliament, date, partyColors, messages, fetcher: fetch })
	});
	return png_response(png);
};
