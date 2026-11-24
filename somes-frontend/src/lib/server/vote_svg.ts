import type { Vote } from '$lib/types';
import {
	donutSegments,
	DONUT_CENTER,
	DONUT_RADIUS,
	DONUT_STROKE_WIDTH,
	DONUT_VIEWBOX
} from '$lib/voteDonut';

export const VOTE_HEX = {
	infavor: '#00a63e',
	against: '#fb2c36',
	abstention: '#2b7fff',
	absent: '#99a1af'
} as const;

export interface VoteSvg {
	svg: string;
	width: number;
	height: number;
}

export function donut_svg(vote: Vote): string {
	const segments = donutSegments(
		[
			{ value: vote.infavor_count, color: VOTE_HEX.infavor },
			{ value: vote.against_count, color: VOTE_HEX.against },
			{ value: vote.abstention_count, color: VOTE_HEX.abstention },
			{ value: vote.absence_count, color: VOTE_HEX.absent }
		].filter((d) => d.value > 0)
	);
	const circles = segments
		.map(
			(s) =>
				`<circle cx="${DONUT_CENTER}" cy="${DONUT_CENTER}" r="${DONUT_RADIUS}" fill="none" stroke="${s.color}" stroke-width="${DONUT_STROKE_WIDTH}" stroke-dasharray="${s.dasharray}" stroke-dashoffset="${s.dashoffset}"/>`
		)
		.join('');
	return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${DONUT_VIEWBOX} ${DONUT_VIEWBOX}"><g transform="rotate(-90 ${DONUT_CENTER} ${DONUT_CENTER})">${circles}</g></svg>`;
}
