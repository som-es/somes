import type { Parliament } from '$lib/api/parliament';
import { dashDateToDotDate } from '$lib/date';
import type { Messages } from '$lib/i18n/messages/types';
import type { VoteResult } from '$lib/types';

export type Translate = (key: keyof Messages) => string;

export function majorityLabel(voteResult: VoteResult, translate: Translate): string {
	return voteResult.legislative_initiative.requires_simple_majority
		? translate('filterOption.simpleMajority')
		: translate('filterOption.twoThirdsMajority');
}

export function voteDateLabel(voteResult: VoteResult): string {
	const initiative = voteResult.legislative_initiative;
	return dashDateToDotDate(
		initiative.vote_date !== null
			? initiative.vote_date
			: initiative.nr_plenary_activity_date.toString()
	);
}

export function voteTypeLabel(voteResult: VoteResult, translate: Translate): string {
	const voting = voteResult.legislative_initiative.voting;
	if (!voting) return translate('voteType.other');
	switch (voting) {
		case 'Law':
			return translate('voteType.law');
		case 'Amendment':
			return translate('voteType.amendment');
		case 'Resolution':
			return translate('voteType.resolution');
		case 'Report':
			return translate('voteType.report');
		default:
			return '';
	}
}

export function voteBadgeTexts(
	voteResult: VoteResult,
	parliament: Parliament,
	translate: Translate
): string[] {
	return [
		parliament !== 'eu' ? majorityLabel(voteResult, translate) : '',
		voteResult.legislative_initiative.gp,
		voteDateLabel(voteResult),
		voteTypeLabel(voteResult, translate)
	].filter((text) => text !== '');
}
