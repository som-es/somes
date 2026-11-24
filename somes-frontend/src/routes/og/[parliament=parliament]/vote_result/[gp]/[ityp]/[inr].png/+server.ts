import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { isHasError, vote_result_by_path } from '$lib/api/api';
import { is_not_found } from '$lib/api/load_error';
import type { Parliament } from '$lib/api/parliament';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllSeats } from '$lib/caching/seats';
import { cachedPartyColors } from '$lib/caching/party_color';
import { de } from '$lib/i18n/messages/de';
import { en } from '$lib/i18n/messages/en';
import { isVoteInFavor, votesByPartySize } from '$lib/partyInfavor';
import { shorten } from '$lib/seo/text';
import type { Vote, VoteResult } from '$lib/types';
import { voteBadgeTexts } from '$lib/voteBadges';
import { render_share_image, SHARE_ICONS, svgDataUrl } from '$lib/server/share_image';
import { vote_parliament_svg } from '$lib/server/parliament_svg';
import { donut_svg } from '$lib/server/vote_svg';
import { png_response } from '$lib/server/share_response';
import type { ShareCounts, ShareImage, ShareParty } from '$lib/components/ShareCards/types';
import VoteResultShareCard from '$lib/components/ShareCards/VoteResultShareCard.svelte';

function voteCounts(voteResult: VoteResult): ShareCounts {
	const named = voteResult.named_votes;
	if (voteResult.legislative_initiative.voted_by_name && named) {
		const present = named.named_votes.filter((vote) => !vote.was_absent);
		const abstained = (vote: (typeof present)[number]) =>
			(vote as { was_abstention?: boolean }).was_abstention === true;
		return {
			inFavor: named.named_vote_info.pro_count,
			against: named.named_vote_info.contra_count,
			abstention: present.filter(abstained).length
		};
	}
	const sum = (key: 'infavor_count' | 'against_count' | 'abstention_count') =>
		voteResult.votes.reduce((total, vote) => total + vote[key], 0);
	return {
		inFavor: sum('infavor_count'),
		against: sum('against_count'),
		abstention: sum('abstention_count')
	};
}

function partyRow(voteResult: VoteResult): ShareParty[] {
	if (voteResult.legislative_initiative.accepted === null) return [];
	const rollCall = voteResult.legislative_initiative.voted_by_name;
	return votesByPartySize(voteResult).map((vote: Vote) => ({
		name: vote.party,
		donut: rollCall ? svgDataUrl(donut_svg(vote)) : null,
		inFavor: isVoteInFavor(vote)
	}));
}

export const GET: RequestHandler = async ({ params, fetch }) => {
	const parliament = params.parliament as Parliament;
	const voteResult = await vote_result_by_path(
		params.gp,
		params.ityp,
		params.inr,
		fetch,
		parliament
	);
	if (isHasError(voteResult)) error(is_not_found(voteResult) ? 404 : 503, 'Not available');

	const messages = parliament === 'eu' ? en : de;
	const initiative = voteResult.legislative_initiative;
	const accepted = initiative.accepted;
	const result: 'accepted' | 'rejected' | null =
		accepted === null || accepted === 'u' ? null : accepted === 'a' ? 'accepted' : 'rejected';

	let seats: ShareImage | null = null;
	if (accepted !== null) {
		const date = initiative.vote_date ?? initiative.nr_plenary_activity_date;
		const [{ hasSeatInfo, delegates }, allSeats, partyColors] = await Promise.all([
			fetchDelegates(date, params.gp, fetch, parliament),
			cachedAllSeats(false, fetch, parliament),
			cachedPartyColors(true, parliament, fetch)
		]);
		const graphic =
			delegates && allSeats
				? vote_parliament_svg({
						voteResult,
						delegates,
						allSeats,
						hasSeatInfo,
						parliament,
						partyColors
					})
				: null;
		if (graphic) {
			const scale = Math.min(320 / graphic.width, 200 / graphic.height);
			seats = {
				src: svgDataUrl(graphic.svg),
				width: Math.round(graphic.width * scale),
				height: Math.round(graphic.height * scale)
			};
		}
	}

	const png = await render_share_image(VoteResultShareCard, {
		icons: SHARE_ICONS,
		heading: messages['share.voteIn'].replace(
			'{parliament}',
			messages[parliament === 'eu' ? 'seo.parliament.eu' : 'seo.parliament.at']
		),
		title: shorten(voteResult.ai_summary?.short_title || initiative.description, 200),
		summary: voteResult.ai_summary?.short_summary
			? shorten(voteResult.ai_summary.short_summary, 300)
			: '',
		result,
		pills: voteBadgeTexts(voteResult, parliament, (key) => messages[key]),
		parties: partyRow(voteResult),
		counts: voteCounts(voteResult),
		seats
	});

	return png_response(png);
};
