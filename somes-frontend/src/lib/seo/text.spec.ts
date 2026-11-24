import { describe, expect, it } from 'vitest';
import { shorten } from './text';

describe('shorten', () => {
	it('cuts long text at a word boundary', () => {
		expect(shorten('kurz')).toBe('kurz');
		expect(shorten('wort '.repeat(50))).toMatch(/^(wort ){30,}.*wort…$/);
		expect(shorten('wort '.repeat(50)).length).toBeLessThanOrEqual(160);
	});
});
