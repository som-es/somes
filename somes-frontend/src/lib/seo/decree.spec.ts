import { describe, expect, it } from 'vitest';
import type { Decree } from '$lib/components/Delegates/Decrees/types';
import { bgbl_citation, decree_seo } from './decree';

const decree = {
	ris_id: 'BGBLA_2024_II_307',
	title: 'Verordnung über die Festlegung der Vignettenpreise (Vignettenpreisverordnung 2024)',
	short_title: 'Vignettenpreisverordnung 2024',
	ai_summary: null
} as unknown as Decree;

describe('decree_seo', () => {
	it('formats the BGBl citation from the RIS id', () => {
		expect(bgbl_citation('BGBLA_2024_II_307')).toBe('BGBl. II Nr. 307/2024');
		expect(bgbl_citation('BGBLA_2023_I_5')).toBe('BGBl. I Nr. 5/2023');
		expect(bgbl_citation('something-else')).toBeNull();
	});

	it('prefers the AI short title and summary, falls back to the RIS titles', () => {
		expect(decree_seo(decree)).toEqual({
			title: 'Vignettenpreisverordnung 2024 – BGBl. II Nr. 307/2024 | Somes',
			description: decree.title
		});
		const withAi = {
			...decree,
			ai_summary: { short_title: 'Neue Vignettenpreise', short_summary: 'Kurz.' }
		} as unknown as Decree;
		expect(decree_seo(withAi)).toEqual({
			title: 'Neue Vignettenpreise – BGBl. II Nr. 307/2024 | Somes',
			description: 'Kurz.'
		});
	});
});
