import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { isHasError } from '$lib/api/api';
import { is_not_found } from '$lib/api/load_error';
import type { Parliament } from '$lib/api/parliament';
import { cachedPartyColors } from '$lib/caching/party_color';
import { gov_proposal_by_path } from '$lib/components/Proposals/api';
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
	const result = await gov_proposal_by_path(params.gp, params.inr, fetch, parliament);
	if (isHasError(result)) error(is_not_found(result) ? 404 : 503, 'Not available');

	const messages = parliament === 'eu' ? en : de;
	const proposal = result.gov_proposal;
	const meta = proposal.ministrial_proposal;
	const date = meta.raw_data_created_at;
	const delegate = result.delegates?.at(0);
	const partyColors = await cachedPartyColors(true, parliament, fetch);

	const png = await render_share_image(MinisterialShareCard, {
		icons: SHARE_ICONS,
		heading: messages['history.proposals.title'],
		title: proposal.ai_summary?.short_title || meta.description,
		subtitle: `Ministerialentwurf vom ${viennaDate(date)}`,
		summaryHeading: messages['ministerialView.summary'],
		summary: proposal.ai_summary?.short_summary ?? '',
		badges: ministerialInfoBadges(meta.ressort, date, meta.gp, 'de-AT'),
		person: delegate
			? await share_person(delegate, { parliament, date, partyColors, messages, fetcher: fetch })
			: null
	});
	return png_response(png);
};
