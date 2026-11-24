import { describe, expect, it } from 'vitest';
import { donut_svg } from './vote_svg';
import type { Vote } from '$lib/types';

describe('donut_svg', () => {
	it('draws one segment per non-empty category in the site colours', () => {
		const vote = {
			party: 'X',
			code: null,
			infavor_count: 10,
			against_count: 5,
			abstention_count: 0,
			absence_count: 2
		} as Vote;
		const svg = donut_svg(vote);
		expect(svg.match(/<circle /g)).toHaveLength(3);
		expect(svg).toContain('#00a63e');
		expect(svg).not.toContain('#2b7fff');
	});
});
