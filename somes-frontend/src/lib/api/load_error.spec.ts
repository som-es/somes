import { describe, expect, it } from 'vitest';
import { isHttpError } from '@sveltejs/kit';
import { or_error } from './load_error';

const apiError = (field: string, status?: number) => ({
	error: 'x',
	error_type: 'FilterError',
	field,
	meta: null,
	status
});

function statusOf(fn: () => unknown): number | null {
	try {
		fn();
		return null;
	} catch (e) {
		return isHttpError(e) ? e.status : -1;
	}
}

describe('or_error', () => {
	it('passes real data through', () => {
		expect(or_error({ id: 1 })).toEqual({ id: 1 });
	});

	it('turns a missing entry into a 404', () => {
		expect(statusOf(() => or_error(apiError('NotFound', 404)))).toBe(404);
		expect(statusOf(() => or_error(apiError('Custom', 404)))).toBe(404);
		expect(statusOf(() => or_error(apiError('NotFound')))).toBe(404);
	});

	it('turns other failures into a temporary 503', () => {
		expect(statusOf(() => or_error(apiError('SqlFailure', 500)))).toBe(503);
		expect(statusOf(() => or_error(apiError('', undefined)))).toBe(503);
	});
});
