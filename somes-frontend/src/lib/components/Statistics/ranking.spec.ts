import { describe, expect, it } from 'vitest';
import { competitionRanks } from './ranking';

describe('competitionRanks', () => {
	it('gives seventeen tied leaders rank one, then continues at eighteen', () => {
		expect(competitionRanks([...Array(17).fill(1.2), 1.183])).toEqual([...Array(17).fill(1), 18]);
	});
	it('also handles ascending values and later ties', () => {
		expect(competitionRanks([1, 1.1, 1.1, 1.2])).toEqual([1, 2, 2, 4]);
	});
	it('ignores floating-point noise without equating scores rounded to the same display value', () => {
		expect(competitionRanks([1.2000000000000002, 1.2, 1.1999])).toEqual([1, 1, 3]);
	});
	it('handles empty data', () => {
		expect(competitionRanks([])).toEqual([]);
	});
});
