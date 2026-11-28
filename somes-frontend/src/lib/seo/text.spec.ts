import { describe, expect, it } from 'vitest';
import { meta_description, shorten } from './text';

describe('shorten', () => {
	it('cuts long text at a word boundary', () => {
		expect(shorten('kurz')).toBe('kurz');
		expect(shorten('wort '.repeat(50))).toMatch(/^(wort ){30,}.*wort…$/);
		expect(shorten('wort '.repeat(50)).length).toBeLessThanOrEqual(160);
	});
});

describe('meta_description', () => {
	it('keeps whole sentences when one fits', () => {
		const text = `Erster Satz über das Gesetz. ${'Zweiter Satz mit sehr vielen Wörtern '.repeat(6)}.`;
		expect(meta_description(text)).toBe('Erster Satz über das Gesetz.');
	});

	it('falls back to a word boundary when no sentence fits', () => {
		const result = meta_description('wort '.repeat(50));
		expect(result).toMatch(/wort…$/);
		expect(result.length).toBeLessThanOrEqual(155);
	});

	it('does not split at ordinal numbers', () => {
		const text = `Das Inkrafttreten ist für den 1. Juli 2025 vorgesehen und ${'betrifft viele Bereiche '.repeat(8)}`;
		expect(meta_description(text)).not.toMatch(/1\.$/);
		expect(meta_description(text).length).toBeLessThanOrEqual(155);
	});
});
