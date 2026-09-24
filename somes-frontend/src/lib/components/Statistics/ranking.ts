/** Competition ranks for values already sorted in display order: 1, 1, 3. */
export function competitionRanks(values: readonly number[]): number[] {
	let rank = 0;
	let groupValue = 0;
	return values.map((value, index) => {
		// Ignore floating-point noise, not differences hidden by display rounding.
		const tolerance = Number.EPSILON * 16 * Math.max(1, Math.abs(value), Math.abs(groupValue));
		if (index === 0 || Math.abs(value - groupValue) > tolerance) {
			rank = index + 1;
			groupValue = value;
		}
		return rank;
	});
}
