import { describe, expect, it, vi } from 'vitest';

vi.mock('./parliament', () => ({ getParliament: () => 'at' }));

import { fetchSavely } from './api';

describe('fetchSavely', () => {
	it('handles a plain-text missing route without trying to parse JSON', async () => {
		const response = new Response('route not found', {
			status: 404,
			statusText: 'Not Found',
			headers: { 'Content-Type': 'text/plain' }
		});
		const parse = vi.spyOn(response, 'json');
		expect(await fetchSavely(() => Promise.resolve(response))).toMatchObject({
			error: 'HTTP 404: Not Found',
			error_type: 'FetchError'
		});
		expect(parse).not.toHaveBeenCalled();
	});

	it('preserves structured API errors', async () => {
		const error = {
			error: 'Internal server error',
			error_type: 'StatisticsResponse',
			field: 'DbSelectFailure',
			meta: null
		};
		expect(await fetchSavely(() => Promise.resolve(Response.json(error, { status: 500 })))).toEqual(
			error
		);
	});

	it('returns successful statistics data', async () => {
		const data = [{ delegate_name: 'Example', accuracy_score: 0.75 }];
		expect(await fetchSavely(() => Promise.resolve(Response.json(data)))).toEqual(data);
	});
});
