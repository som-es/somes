import type { Delegate, NamedVote, Speech, VoteResult } from '$lib/types';

export interface Bubble {
	r: number;
	x: number;
	y: number;
	del: Delegate | null;
	namedVote: NamedVote | null;
	color: string | null;
	opacity: number;
	title: string | null;
}

export function generateHalfCircle(n: number, r: number, w: number, h: number) {
	let smaller_n = n - 2;

	let scaled_max_angle = 18000.0;
	let modulo = scaled_max_angle / smaller_n;
	let count_to = scaled_max_angle + modulo + 1;

	let normalize = (100 * count_to) / scaled_max_angle;
	let circles: { x: number; y: number }[] = [];

	for (let angle_deg = count_to; angle_deg > 0; angle_deg -= modulo) {
		let angle_rad = (-(angle_deg / normalize) * Math.PI) / 180;

		const x = 2.0 * r * Math.cos(angle_rad) + w / 2;
		const y = 2.0 * r * Math.sin(angle_rad) + h / 2;

		const circle = {
			x,
			y
		};
		circles = circles.concat(circle);
		// circles.push(circle);
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
	circles2d[del.seat_row - 1][del.seat_col - 1].del = del;
	circles2d[del.seat_row - 1][del.seat_col - 1].title = '';
	// circles2d[del.seat_row-1][del.seat_col-1].color = partyToColor(del.party);
	circles2d[del.seat_row - 1][del.seat_col - 1].color = fn(del.party);
}

function findDelegateById(dels: Delegate[], id: number): Delegate | undefined {
	return dels.find((del) => del.id === id);
}

export function enrichCirclesWithSpeechInfo(speeches: Speech[], dels: Delegate[]): Bubble[] {
	const speechDelegates: Bubble[] = [];
	const delegatesAt: Delegate[] = dels;
	speeches.forEach((speech) => {
		const delegate = findDelegateById(delegatesAt, speech.delegate_id);

		if (delegate) {
			speechDelegates.push({
				r: 0,
				x: 0,
				y: 0,
				del: delegate,
				namedVote: null,
				color: null,
				opacity: 0,
				title: speech.opinion
			});
		}
	});
	return speechDelegates;
}

export function enrichCirclesWithNamedVoteInfo(namedVotes: NamedVote[], dels: Delegate[]): Bubble[] {
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
				namedVote: null,
				color: null,
				opacity: 0,
				title,
			});
		}
	});
	return namedVoteDelegates;
}

export function enrichCirclesWithSpeechInfoOnSeat(
	speeches: Speech[],
	circles2d: Bubble[][],
	dels: Delegate[]
) {
	speeches.forEach((speech) => {
		let del = findDelegateById(dels, speech.delegate_id);
		if (del == null || del.seat_col == null || del.seat_row == null) return;

		circles2d[del.seat_row - 1][del.seat_col - 1].title = speech.infavor
			? `Dafür gesprochen`
			: `Dagegen gesprochen`;
		circles2d[del.seat_row - 1][del.seat_col - 1].opacity = speech.infavor ? 1.0 : 0.2;
		circles2d[del.seat_row - 1][del.seat_col - 1].r = +10.9;
	});
}

export function enrichCirclesWithNamedVoteInfoOnSeat(
	namedVotes: NamedVote[],
	circles2d: Bubble[][],
	dels: Delegate[]
) {
	namedVotes.forEach((namedVote) => {
		let del = findDelegateById(dels, namedVote.delegate_id);
		if (del == null || del.seat_col == null || del.seat_row == null) return;
		if (namedVote.was_absent) {
			circles2d[del.seat_row - 1][del.seat_col - 1].r = +5.9;
			circles2d[del.seat_row - 1][del.seat_col - 1].title = `abwesend/keine Stimme abgegeben`;
			return;
		}

		circles2d[del.seat_row - 1][del.seat_col - 1].namedVote = namedVote;
		circles2d[del.seat_row - 1][del.seat_col - 1].title = namedVote.infavor ? `Ja` : `Nein`;
		circles2d[del.seat_row - 1][del.seat_col - 1].opacity = namedVote.infavor ? 1.0 : 0.2;
		circles2d[del.seat_row - 1][del.seat_col - 1].r = +9.9;
	});
}

export function setupParliament(
	seats: number[],
	width: number,
	height: number,
	r: number
): Bubble[][] {
	let circles2d: Bubble[][] = [];
	seats.forEach((seat, idx) => {
		circles2d.push(
			generateHalfCircle(
				seat,
				70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30 : 0),
				width,
				height
			).map((circle) => {
				return {
					r,
					x: circle.x,
					y: circle.y,
					del: null,
					color: 'rgb(196, 180, 189)',
					opacity: 0.0,
					title: null,
					namedVote: null
				};
			})
		);
	});

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
