import type { Delegate } from "$lib/types";

export interface DecreeDelegate {
    delegate: Delegate | null;
    decree: Decree | null;
}

export interface Decree {
	gov_official_id: number;
	ris_id: string;
	ministrial_issuer: string;
	title: string;
	short_title: string;
	publication_date: string;
	part: string;
	gp: string | null;
	documents: Document[];
    eli: string | null;
    emphasis: string | null;
    document_url: string | null;
}

export interface DecreesWithMaxPage {
	decrees: Decree[];
	entry_count: number;
	max_page: number;
}

export interface DecreeFilter {
	legis_period: string | null;
	gov_officials: number[] | null;
}