<script lang="ts">
	import SpeechBar from '$lib/components/Delegates/Speeches/SpeechBar.svelte';
	import { Opinion, type FullSpeech } from '$lib/speechTypes';
	import { Tone } from '$lib/ai_summary_types';

	function fakeVoteResult(id: number, title: string) {
		return {
			id,
			legislative_initiative: {
				title,
				description: `${title} — Beschreibung der legislativen Initiative zur Änderung des Bundesgesetzes.`,
				gp: 'XXVIII',
				ityp: 'BI',
				inr: id,
				vote_date: '2026-05-01',
				accepted: id % 2 === 0 ? 'a' : null,
				requires_simple_majority: true,
				nr_plenary_activity_date: '2026-05-01',
				voting: 'Law'
			},
			votes: [
				{ party: 'ÖVP', code: 'V', fraction: 51, infavor: true },
				{ party: 'SPÖ', code: 'S', fraction: 41, infavor: false },
				{ party: 'FPÖ', code: 'F', fraction: 30, infavor: id % 2 === 0 }
			],
			named_votes: null,
			ai_summary: null,
			eurovoc_topics: [],
			speeches: [],
			topics: [],
			other_keyword_topics: [],
			documents: [],
			absences: [],
			issued_by_dels: [],
			referenced_by_others_ids: [],
			references: null
		};
	}

	if (typeof window !== 'undefined') {
		const originalFetch = window.fetch.bind(window);
		window.fetch = async (input, init) => {
			const urlStr = typeof input === 'string' ? input : (input as Request).url;
			if (urlStr.includes('/vote_results/id/')) {
				const id = Number(urlStr.split('/').pop());
				return new Response(JSON.stringify(fakeVoteResult(id, `Testinitiative ${id}`)), {
					status: 200,
					headers: { 'content-type': 'application/json' }
				});
			}
			return originalFetch(input, init);
		};
	}

	const speech: FullSpeech = {
		id: 1,
		speech: {
			delegate_id: 1,
			vote_result_ids: [10, 11],
			infavor: true,
			duration_in_seconds: 185,
			opinion: 'Pro',
			document_urls: ['https://example.com/doc.pdf'],
			about: 'Fallback about text without ai summary',
			start: '10:00:00'
		},
		ai_summary: {
			id: 1,
			speech_id: 1,
			short_title: 'Kurzer KI-Titel der Rede',
			one_sentence_short_summary: 'Ein Satz.',
			very_short_summary: 'Sehr kurz.',
			short_summary: 'Dies ist eine kurze KI-generierte Zusammenfassung der Rede.',
			summary: 'Zusammenfassung.',
			detailed_summary: 'Detaillierte Zusammenfassung.',
			very_detailed_summary: 'Sehr detaillierte Zusammenfassung.',
			full_speech_summary: {
				short_title: 'Kurzer KI-Titel der Rede',
				one_sentence_short_summary: 'Ein Satz.',
				very_short_summary: 'Sehr kurz.',
				short_summary: 'Dies ist eine kurze KI-generierte Zusammenfassung der Rede.',
				summary: 'Zusammenfassung.',
				detailed_summary: 'Detaillierte Zusammenfassung.',
				very_detailed_summary: 'Sehr detaillierte Zusammenfassung.',
				glossary: {
					difficult_terms: [{ term: 'Legislative', simple_definition: 'Gesetzgebende Gewalt' }]
				},
				critical_analysis: {
					arguments_for: ['Stärkt die Rechtssicherheit', 'Schnellere Umsetzung'],
					arguments_against: ['Hohe Kosten', 'Unklare Zuständigkeiten'],
					tone: Tone.Neutral
				},
				key_points: [
					{
						summarized_point: 'Die Legislative sollte gestärkt werden.',
						unmodified_reference_point: 'unmodified'
					},
					{ summarized_point: 'Zweiter wichtiger Punkt der Rede.', unmodified_reference_point: 'u2' }
				]
			},
			model_used: 'gpt-test',
			version: '1.0',
			generated_at: '2026-06-01T12:00:00Z'
		},
		relations: [
			{
				id: 1,
				speech_ai_summary_id: 1,
				legis_init_id: 10,
				full_speech_relations: {
					propsal_keypoint_relations: null,
					speech_related_to_proposal_summary: true,
					speech_related_to_detailed_proposal_summary: true,
					stance_to_proposal: Opinion.Pro
				},
				model_used: 'gpt-test',
				version: '1.0',
				generated_at: '2026-06-01T12:00:00Z'
			},
			{
				id: 2,
				speech_ai_summary_id: 1,
				legis_init_id: 11,
				full_speech_relations: {
					propsal_keypoint_relations: null,
					speech_related_to_proposal_summary: true,
					speech_related_to_detailed_proposal_summary: false,
					stance_to_proposal: Opinion.Contra
				},
				model_used: 'gpt-test',
				version: '1.0',
				generated_at: '2026-06-01T12:00:00Z'
			}
		]
	} as unknown as FullSpeech;

	const speechNoAi: FullSpeech = {
		...speech,
		ai_summary: null,
		relations: []
	};
</script>

<div class="mx-auto max-w-3xl p-6">
	<h2 class="mb-2 text-xl font-bold">With AI summary + relations</h2>
	<SpeechBar {speech} />

	<h2 class="mt-8 mb-2 text-xl font-bold">Without AI summary (fallback)</h2>
	<SpeechBar speech={speechNoAi} />
</div>
