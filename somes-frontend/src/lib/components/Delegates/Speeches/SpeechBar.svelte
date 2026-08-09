<script lang="ts">
	import { errorToNull, vote_result_by_id } from '$lib/api/api';
	import { type VoteResult } from '$lib/types';
	import { aiViewEnabledStore } from '$lib/stores/stores';
	import clockIcon from '$lib/assets/misc_icons/clock-two.svg?raw';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';
	import { Opinion, type DbSpeechRelations, type FullSpeech } from '$lib/speechTypes';
	import type { Keypoint } from '$lib/ai_summary_types';
	import { slide } from 'svelte/transition';

	interface Props {
		speech: FullSpeech;
	}

	let { speech }: Props = $props();

	interface RelatedVoteResult {
		relation: DbSpeechRelations;
		voteResult: VoteResult | null;
	}

	let open = $state(false);
	let analysisOpen = $state(false);
	let relationsLoaded = $state(false);
	let relatedVoteResults: RelatedVoteResult[] = $state([]);
	let loadingVoteResults = $state(false);

	$effect(() => {
		speech;
		open = false;
		analysisOpen = false;
		relationsLoaded = false;
		relatedVoteResults = [];
	});

	function loadRelatedVoteResults() {
		relatedVoteResults = [];
		loadingVoteResults = speech.relations.length > 0;
		speech.relations.forEach(async (relation) => {
			const voteResult = errorToNull(await vote_result_by_id(relation.legis_init_id.toString()));
			relatedVoteResults = [...relatedVoteResults, { relation, voteResult }];
			loadingVoteResults = false;
		});
	}

	function stanceColor(stance: Opinion | null): string {
		if (stance === Opinion.Pro) return 'bg-green-600';
		if (stance === Opinion.Contra) return 'bg-red-500';
		return 'bg-gray-400';
	}

	let opinion = $derived(
		speech.speech.infavor != null
			? speech.speech.infavor
				? 'Pro'
				: 'Contra'
			: speech.speech.opinion
	);
	let barColor = $derived(
		stanceColor(
			speech.speech.infavor === true
				? Opinion.Pro
				: speech.speech.infavor === false
					? Opinion.Contra
					: null
		)
	);

	let speechDuration = $derived.by(() => {
		if (speech.speech.duration_in_seconds === null || speech.speech.duration_in_seconds === 0)
			return null;
		const mins = Math.floor(speech.speech.duration_in_seconds / 60);
		return { mins, seconds: speech.speech.duration_in_seconds - mins * 60 };
	});

	let aiSummary = $derived(aiViewEnabledStore.value ? speech.ai_summary : null);
	let keyPoints = $derived(
		(aiSummary?.full_speech_summary.key_points ?? []).map(
			(keyPoint): Keypoint => ({
				point: keyPoint.summarized_point,
				paragraph_references: []
			})
		)
	);
	let criticalAnalysis = $derived(aiSummary?.full_speech_summary.critical_analysis ?? null);
	let glossary = $derived(aiSummary?.full_speech_summary.glossary ?? null);

	let expandable = $derived(aiSummary != null || speech.relations.length > 0);

	function toggleOpen() {
		if (!expandable) return;
		open = !open;
		if (open && !relationsLoaded) {
			relationsLoaded = true;
			loadRelatedVoteResults();
		}
	}
</script>

<div class="mt-5">
	<!-- svelte-ignore a11y_no_noninteractive_tabindex (tabindex is only set when role="button" is too) -->
	<div
		class="entry flex items-stretch overflow-hidden bg-primary-100 text-black dark:bg-primary-300"
		role={expandable ? 'button' : undefined}
		tabindex={expandable ? 0 : undefined}
		onclick={toggleOpen}
		onkeypress={(e) => (e.key === 'Enter' || e.key === ' ') && toggleOpen()}
	>
		<div class="w-1.5 shrink-0 {barColor}"></div>
		<div class="flex min-w-0 flex-1 items-center justify-between gap-3 p-3 lg:px-5 lg:py-4">
			<div class="flex min-w-0 flex-1 flex-col">
				{#if aiSummary}
					<span
						class="line-clamp-2 text-sm leading-snug font-semibold lg:text-lg"
						style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
					>
						{aiSummary.short_title}
					</span>
					<span class="mt-0.5 line-clamp-3 text-[10px] text-gray-800 lg:line-clamp-none lg:text-sm">
						{aiSummary.short_summary}
					</span>
				{:else}
					<span class="text-sm font-semibold lg:text-lg">{opinion}</span>
					{#if speech.speech.about}
						<span class="mt-0.5 line-clamp-2 text-[10px] text-gray-800 lg:text-sm">
							{speech.speech.about}
						</span>
					{/if}
				{/if}
			</div>
			<div class="flex shrink-0 items-center gap-3 text-gray-700">
				{#if speechDuration}
					<span class="hidden items-center gap-1 text-xs whitespace-nowrap lg:flex">
						<span
							class="h-3.5 w-3.5 shrink-0 [&_path]:stroke-current [&>svg]:h-full [&>svg]:w-full"
						>
							{@html clockIcon}
						</span>
						{speechDuration.mins}:{speechDuration.seconds.toString().padStart(2, '0')} min
					</span>
				{/if}
				{#each speech.speech.document_urls ?? [] as url}
					<a
						href={url}
						target="_blank"
						aria-label="Dokument"
						title="Redeprotokoll öffnen"
						class="transition-transform hover:scale-110"
						onclick={(e) => e.stopPropagation()}
					>
						<svg
							class="h-4 w-4 lg:h-5 lg:w-5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							><path
								d="M2 4h7a3 3 0 0 1 3 3v13a2 2 0 0 0-2-2H2zM22 4h-7a3 3 0 0 0-3 3v13a2 2 0 0 1 2-2h8z"
							/></svg
						>
					</a>
				{/each}
			</div>
		</div>
	</div>

	{#if open}
		<div transition:slide={{ duration: 240 }}>
			<div class="mt-3 flex flex-col gap-3 rounded-xl bg-primary-200 p-2 dark:bg-primary-400">
				{#if aiSummary}
					<div class="rounded-xl bg-primary-300 px-5 pt-3 pb-3 dark:bg-primary-500">
						<div class="flex items-start justify-between gap-2">
							<h1 class="text-lg font-semibold md:text-xl">Zusammenfassung</h1>
							<AiSummaryHintPopup
								{aiSummary}
								aiGenText="Titel, Zusammenfassungen, Schwerpunkte und kritische Analyse wurden mittels KI aus der Rede zusammengefasst."
							/>
						</div>
						<p class="mt-1 text-base text-gray-800 dark:text-gray-200">
							{#if glossary}
								<GlossaryText text={aiSummary.full_speech_summary.summary} {glossary} />
							{:else}
								{aiSummary.full_speech_summary.summary}
							{/if}
						</p>
					</div>

					<Emphasis emphasis={keyPoints} {glossary} />

					<!-- {#if criticalAnalysis}
						<div class="rounded-xl bg-primary-300 px-5 pt-3 pb-3 dark:bg-primary-500">
							<button
								class="flex w-full flex-wrap items-center justify-between gap-2"
								onclick={() => (analysisOpen = !analysisOpen)}
							>
								<h1 class="text-lg font-semibold md:text-xl">Kritische Analyse</h1>
								<span class="text-md font-semibold"
									>{analysisOpen ? 'Weniger' : 'Mehr'} anzeigen</span
								>
							</button>
							{#if analysisOpen}
								<div transition:slide={{ duration: 240 }} class="mt-2 flex flex-col gap-3">
									<div>
										<h2 class="font-semibold text-green-700 dark:text-green-400">Dafür spricht</h2>
										<ul class="list px-3 pt-1">
											{#each criticalAnalysis.arguments_for as argument}
												<li class="mb-2 items-baseline">
													<span class="badge shrink-0 bg-success-600"></span>
													<span class="text-base text-gray-800 dark:text-gray-200">{argument}</span>
												</li>
											{/each}
										</ul>
									</div>
									<div>
										<h2 class="font-semibold text-red-600 dark:text-red-400">Dagegen spricht</h2>
										<ul class="list px-3 pt-1">
											{#each criticalAnalysis.arguments_against as argument}
												<li class="mb-2 items-baseline">
													<span class="badge shrink-0 bg-red-600"></span>
													<span class="text-base text-gray-800 dark:text-gray-200">{argument}</span>
												</li>
											{/each}
										</ul>
									</div>
								</div>
							{/if}
						</div>
					{/if} -->
				{/if}

				{#if loadingVoteResults}
					<ExpandablePlaceholder />
				{:else if relatedVoteResults.length > 0}
					<div class="px-1">
						<h2 class="text-lg font-semibold">Bezieht sich auf</h2>
						{#each relatedVoteResults as { relation, voteResult } (relation.id)}
							{#if voteResult}
								<div
									class="mt-2 flex items-stretch gap-2"
									title="Haltung der Rede zu dieser Abstimmung: {relation.full_speech_relations
										.stance_to_proposal ?? 'unbekannt'}"
								>
									<div
										class="w-1.5 shrink-0 rounded-full {stanceColor(
											relation.full_speech_relations.stance_to_proposal
										)}"
									></div>
									<VoteResultExpandableBar {voteResult} class="min-w-0 flex-1" />
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	}
</style>
