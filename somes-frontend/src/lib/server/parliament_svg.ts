import { render } from 'svelte/server';
import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
import type { Parliament } from '$lib/api/parliament';
import type { Delegate, VoteResult } from '$lib/types';
import { VOTE_COLORS } from '$lib/voteColors';
import { VOTE_HEX, type VoteSvg } from './vote_svg';

const CSS_COLORS: Record<string, string> = {
	[VOTE_COLORS.infavor]: VOTE_HEX.infavor,
	[VOTE_COLORS.against]: VOTE_HEX.against,
	[VOTE_COLORS.abstention]: VOTE_HEX.abstention,
	[VOTE_COLORS.absent]: VOTE_HEX.absent
};

export function vote_parliament_svg(props: {
	voteResult: VoteResult;
	delegates: Delegate[];
	allSeats: Map<string, number[]>;
	hasSeatInfo: boolean;
	parliament: Parliament;
	partyColors: Map<string, string>;
}): VoteSvg | null {
	const { body } = render(VoteParliament2, {
		props: {
			voteResult: props.voteResult,
			delegates: props.delegates,
			allSeats: props.allSeats,
			noSeats: !props.hasSeatInfo,
			useOffset: props.hasSeatInfo,
			parliament: props.parliament,
			showGovs: true,
			overrideDelegates: true,
			partyColoring: props.partyColors,
			colorMode: 'vote'
		}
	});

	const svg = /<svg[^>]*parliament-svg[\s\S]*?<\/svg>/.exec(body)?.[0];
	if (!svg) return null;

	const circles = [...svg.matchAll(/<circle\b[^>]*>/g)].map(([tag]) => {
		const attr = (name: string) => new RegExp(`\\s${name}="([^"]*)"`).exec(tag)?.[1] ?? '';
		const fill = attr('fill');
		return {
			cx: +attr('cx'),
			cy: +attr('cy'),
			r: +attr('r'),
			fill: CSS_COLORS[fill] ?? fill,
			opacity: attr('fill-opacity') === '' ? 1 : +attr('fill-opacity')
		};
	});
	const visible = circles.filter((c) => c.opacity > 0 && c.r > 0);
	if (visible.length === 0) return null;

	const minX = Math.min(...visible.map((c) => c.cx - c.r));
	const minY = Math.min(...visible.map((c) => c.cy - c.r));
	const width = Math.max(...visible.map((c) => c.cx + c.r)) - minX;
	const height = Math.max(...visible.map((c) => c.cy + c.r)) - minY;
	const content = visible
		.map(
			(c) =>
				`<circle cx="${(c.cx - minX).toFixed(1)}" cy="${(c.cy - minY).toFixed(1)}" r="${c.r}" fill="${c.fill}" fill-opacity="${c.opacity}"/>`
		)
		.join('');

	return {
		svg: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width.toFixed(1)} ${height.toFixed(1)}">${content}</svg>`,
		width,
		height
	};
}
