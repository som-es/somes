import type { Decree } from '$lib/components/Delegates/Decrees/types';
import { meta_description } from './text';

export function bgbl_citation(risId: string): string | null {
	const match = /^BGBL[A-Z]?_(\d{4})_([IVX]+)_(\d+)$/.exec(risId);
	return match ? `BGBl. ${match[2]} Nr. ${match[3]}/${match[1]}` : null;
}

export function decree_seo(decree: Decree): { title: string; description: string } {
	const subject = decree.ai_summary?.short_title || decree.short_title || decree.title;
	const citation = bgbl_citation(decree.ris_id);
	return {
		title: `${subject}${citation ? ` – ${citation}` : ''} | Somes`,
		description: meta_description(decree.ai_summary?.short_summary || decree.title)
	};
}
