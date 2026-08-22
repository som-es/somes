import type { Delegate, NamedVote, VoteResult } from '$lib/types';
import type { Material, Texture } from 'three';
import { partyToColor } from './partyColor';
import type { FullSpeech } from './speechTypes';

export const AMOUNT_PER_SIDE: number = 15;

export interface Bubble {
	r: number;
	x: number;
	y: number;
	angle_rad: number;
	del: Delegate | null;
	speech: FullSpeech | null;
	namedVote: NamedVote | null;
	color: string | null;
	opacity: number;
	title: string | null;
	texture: any | null;
	material: Material | null;
	id: number;
}

export function generateHalfCircle(
	n: number,
	r: number,
	w: number,
	h: number,
	maxAngle: number = 180
) {
	const smaller_n = n - 1;

	const scaled_max_angle = maxAngle;
	const modulo = scaled_max_angle / smaller_n;
	const count_to = scaled_max_angle + 1;

	const normalize = count_to / scaled_max_angle;
	let circles: { x: number; y: number; angle_rad: number }[] = [];

	const diff = (maxAngle - 180) / 2;

	for (let angle_deg = count_to - diff; angle_deg > -diff; angle_deg -= modulo) {
		const angle_rad = (-(angle_deg / normalize) * Math.PI) / 180;

		const x = 2.0 * r * Math.cos(angle_rad) + w / 2;
		const y = 2.0 * r * Math.sin(angle_rad) + h / 2;

		const circle = {
			x,
			y,
			angle_rad
		};
		circles = circles.concat(circle);
		// circles.push(circle);
	}
	return circles;
}

export function generateGovCircles(r: number, w: number, h: number) {
	const centerX = w * 0.5;
	const centerY = h * 0.5 + r * 0.5;

	const spacingFactor = 22;

	let circles: { x: number; y: number; angle_rad: number }[] = [];

	for (let i = -AMOUNT_PER_SIDE; i <= AMOUNT_PER_SIDE; i++) {
		if (i >= -1 && i <= 1) {
			continue;
		}
		const x = centerX + spacingFactor * i;

		const circle = {
			x,
			y: centerY,
			angle_rad: 0
		};
		circles = circles.concat(circle);
	}
	return circles;
}

export function setDelOnBubble(
	del: Delegate,
	circles2d: Bubble[][],
	fn: (party: string) => string
) {
	if (del.seat_row == null || del.seat_col == null) {
		return;
	}

	if (!circles2d[del.seat_row - 1] || !circles2d[del.seat_row - 1][del.seat_col - 1]) {
		return;
	}
	// console.log(circles2d[del.seat_row - 1][del.seat_col - 1]);
	circles2d[del.seat_row - 1][del.seat_col - 1].del = del;
	circles2d[del.seat_row - 1][del.seat_col - 1].title = '';
	// circles2d[del.seat_row-1][del.seat_col-1].color = partyToColor(del.party);
	circles2d[del.seat_row - 1][del.seat_col - 1].color = fn(del.party);
}

function findDelegateById(dels: Delegate[], id: number): Delegate | undefined {
	return dels.find((del) => del.id === id);
}

export function enrichParliamentBubbles(
	bubbles: Bubble[][],
	dels: Delegate[],
	voteResult: VoteResult | null,
	setOpacity: (bubble: Bubble) => void,
	colorFn = partyToColor
) {
	if (bubbles.length == 0) {
		return;
	}
	dels.forEach(async (del) => {
		setDelOnBubble(del, bubbles, colorFn);

		if (del.seat_col != null && del.seat_row != null) {
			try {
				setOpacity(bubbles[del.seat_row - 1][del.seat_col - 1]);
			} catch (e) {
				const x = bubbles[del.seat_row - 1];
				console.error(x);
				console.error('could not set opacity for delegate:', del);
			}
		}
	});

	if (voteResult) {
		enrichCirclesWithSpeechInfoOnSeat(
			voteResult.speeches,
			bubbles,
			dels,
			voteResult.legislative_initiative.pre_declined_type?.includes('p'),
			setOpacity
		);
		if (voteResult.named_votes) {
			enrichCirclesWithNamedVoteInfoOnSeat(
				voteResult.named_votes.named_votes,
				bubbles,
				dels,
				setOpacity
			);
		}
		enrichtCirclesWithAbsenceInfoOnSeat(voteResult.absences, bubbles, dels);
	}
}

export function genCirclesWithAbsenceInfo(absences: number[], dels: Delegate[]): Bubble[] {
	const speechDelegates: Bubble[] = [];
	const delegatesAt: Delegate[] = dels;
	absences.forEach((absence) => {
		const delegate = findDelegateById(delegatesAt, absence);

		if (delegate) {
			// TODO adapt
			speechDelegates.push({
				r: 0,
				x: 0,
				y: 0,
				del: delegate,
				speech: null,
				namedVote: null,
				color: null,
				opacity: 0,
				title: 'abwesen',
				texture: null,
				material: null,
				angle_rad: 0,
				id: 0
			});
		}
	});
	return speechDelegates;
}

export function genCirclesWithSpeechInfo(speeches: FullSpeech[], dels: Delegate[]): Bubble[] {
	const speechDelegates: Bubble[] = [];
	const delegatesAt: Delegate[] = dels;

	speeches.forEach((speech) => {
		if (speech.speech == undefined) {
			const delegate = findDelegateById(delegatesAt, speech.speech.delegate_id);

			if (delegate) {
				speechDelegates.push({
					r: 0,
					x: 0,
					y: 0,
					del: delegate,
					speech,
					namedVote: null,
					color: null,
					opacity: 0,
					title: speech.speech.opinion,
					texture: null,
					material: null,
					angle_rad: 0,
					id: 0
				});
			}
		}
	});
	return speechDelegates;
}

export function genCirclesWithNamedVoteInfo(namedVotes: NamedVote[], dels: Delegate[]): Bubble[] {
	const namedVoteDelegates: Bubble[] = [];
	const delegatesAt: Delegate[] = dels;
	namedVotes.forEach((vote) => {
		const delegate = findDelegateById(delegatesAt, vote.delegate_id);
		let title;
		if (vote.was_absent) {
			title = `abwesend/keine Stimme abgegeben`;
		} else {
			title = vote.infavor ? `Ja` : `Nein`;
		}
		if (delegate) {
			namedVoteDelegates.push({
				r: 0,
				x: 0,
				y: 0,
				del: delegate,
				speech: null,
				namedVote: vote,
				color: null,
				opacity: 0,
				title,
				texture: null,
				material: null,
				angle_rad: 0,
				id: 0
			});
		}
	});
	return namedVoteDelegates;
}

export async function enrichCirclesWithSpeechInfoOnSeat(
	speeches: FullSpeech[],
	circles2d: Bubble[][],
	dels: Delegate[],
	reversed = false,
	setOpacity: (bubble: Bubble) => void
) {
	speeches.forEach((speech) => {
		const del = findDelegateById(dels, speech.speech.delegate_id);
		if (del == null || del.seat_col == null || del.seat_row == null) return;

		const infavor = speech.speech.infavor;

		circles2d[del.seat_row - 1][del.seat_col - 1].speech = speech;

		if (infavor == null) {
			circles2d[del.seat_row - 1][del.seat_col - 1].title = speech.speech.opinion;
		} else {
			circles2d[del.seat_row - 1][del.seat_col - 1].title = infavor ? `Pro` : `Contra`;
		}
		setOpacity(circles2d[del.seat_row - 1][del.seat_col - 1]);
		circles2d[del.seat_row - 1][del.seat_col - 1].r = +10.9;
	});
}

export function enrichtCirclesWithAbsenceInfoOnSeat(
	absences: number[],
	circles2d: Bubble[][],
	dels: Delegate[]
) {
	absences.forEach((delegate_id) => {
		const del = findDelegateById(dels, delegate_id);
		if (del == null || del.seat_col == null || del.seat_row == null) return;

		circles2d[del.seat_row - 1][del.seat_col - 1].r = +4.9;
		// circles2d[del.seat_row - 1][del.seat_col - 1].color = "white"
		circles2d[del.seat_row - 1][del.seat_col - 1].title = `abwesend`;
	});
}

export function enrichCirclesWithNamedVoteInfoOnSeat(
	namedVotes: NamedVote[],
	circles2d: Bubble[][],
	dels: Delegate[],
	setOpacity: (bubble: Bubble) => void
) {
	namedVotes.forEach((namedVote) => {
		const del = findDelegateById(dels, namedVote.delegate_id);
		if (del == null || del.seat_col == null || del.seat_row == null) return;

		circles2d[del.seat_row - 1][del.seat_col - 1].namedVote = namedVote;

		if (namedVote.was_absent) {
			circles2d[del.seat_row - 1][del.seat_col - 1].r = +4.9;
			circles2d[del.seat_row - 1][del.seat_col - 1].title = `abwesend/keine Stimme abgegeben`;
			setOpacity(circles2d[del.seat_row - 1][del.seat_col - 1]);
			return;
		}
		if (namedVote.was_abstention) {
			circles2d[del.seat_row - 1][del.seat_col - 1].r = +15;
			circles2d[del.seat_row - 1][del.seat_col - 1].title = `enthalten`;
			setOpacity(circles2d[del.seat_row - 1][del.seat_col - 1]);
			return;
		}

		circles2d[del.seat_row - 1][del.seat_col - 1].title = namedVote.infavor ? `Ja` : `Nein`;
		setOpacity(circles2d[del.seat_row - 1][del.seat_col - 1]);
		circles2d[del.seat_row - 1][del.seat_col - 1].r = +9.9;
	});
}

export function setupParliament(
	seats: number[],
	width: number,
	height: number,
	r: number,
	useOffset = true,
	maxAngle: number = 180,
	yOffset: number = 0
): Bubble[][] {
	let id = 0;
	const circles2d: Bubble[][] = [];
	seats.forEach((seat, idx) => {
		circles2d.push(
			generateHalfCircle(
				seat,
				70 + (useOffset ? idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30 : 0) : idx * 19),
				width,
				height,
				maxAngle
			).map((circle) => {
				id += 1;
				return {
					r,
					x: circle.x,
					y: circle.y + yOffset,
					angle_rad: circle.angle_rad,
					del: null,
					color: 'rgb(196, 180, 189)',
					opacity: 0,
					title: null,
					namedVote: null,
					speech: null,
					texture: null,
					material: null,
					id
				};
			})
		);
	});

	circles2d.push(
		generateGovCircles(70, width, height).map((circle) => {
			id += 1;
			return {
				r,
				x: circle.x,
				y: circle.y + yOffset,
				angle_rad: circle.angle_rad,
				del: null,
				color: 'rgb(196, 180, 189)',
				opacity: 0,
				title: null,
				namedVote: null,
				speech: null,
				texture: null,
				material: null,
				id
			};
		})
	);

	return circles2d;
}

function selectBubble(prev_selected: Bubble | null, selection: Bubble) {
	if (selection.del == null) {
		return;
	}
	if (prev_selected != null) {
		prev_selected.r = 6;
	}
	selection.r = +10.9;
}
