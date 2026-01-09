import type { DbAiSummary } from "$lib/ai_summary_types";
import type { Delegate, Document, Topic } from "$lib/types";

export interface MinisterialViewData {
	delegate: Delegate;
    topics: Topic[];
    type: "decree" | "gov_proposal";
	eurovocTopics: Topic[];
	otherKeywordTopics: Topic[];
	ministerialIssuers: number[];
	documents: Document[];
	aiSummary: DbAiSummary | null;
    alternativeTitle: string;
    date: string;
	ressort: string | null;
	ressortShortform: string | null;
	originalDocumentUrl: string | null;
    infoBadges: string[];
}
