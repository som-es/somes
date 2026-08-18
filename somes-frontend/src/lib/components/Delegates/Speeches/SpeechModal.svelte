<script lang="ts">
	import { Dialog, Select } from 'bits-ui';
	import type { Snippet } from 'svelte';
	import { slide } from 'svelte/transition';
	import { errorToNull, vote_result_by_id } from '$lib/api/api';
	import clockIcon from '$lib/assets/misc_icons/clock-two.svg?raw';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import checkmarkSmall from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import linkIcon from '$lib/assets/misc_icons/external-link.svg?raw';
	import type { VoteResult } from '$lib/types';
	import { Opinion, type DbSpeechRelations, type FullSpeech } from '$lib/speechTypes';
	import { aiViewEnabledStore, speechDetailLevelStore } from '$lib/stores/stores';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';

	interface Props {
		speech: FullSpeech;
		open: boolean;
		header?: Snippet;
	}

	let { speech, open = $bindable(false), header }: Props = $props();

	interface RelatedVoteResult {
		relation: DbSpeechRelations;
		voteResult: VoteResult | null;
	}

	let relatedVoteResults: RelatedVoteResult[] = $state([]);
	let loadingVoteResults = $state(false);
	let loadedSpeechId: number | null = null;

	let expandedKeyPoints: Set<number> = $state(new Set());

	function toggleKeyPoint(index: number) {
		const next = new Set(expandedKeyPoints);
		if (!next.delete(index)) next.add(index);
		expandedKeyPoints = next;
	}

	$effect(() => {
		if (!open || loadedSpeechId === speech.id) return;
		loadedSpeechId = speech.id;
		relatedVoteResults = [];
		expandedKeyPoints = new Set();
		loadingVoteResults = speech.relations.length > 0;
		speech.relations.forEach(async (relation) => {
			const voteResult = errorToNull(await vote_result_by_id(relation.legis_init_id.toString()));
			relatedVoteResults = [...relatedVoteResults, { relation, voteResult }];
			loadingVoteResults = false;
		});
	});

	function encodeQuotePart(text: string): string {
		return encodeURIComponent(text).replace(/-/g, '%2D');
	}

	// for long text, only search via 3 start and end words 
	function protocolLink(quote: string): string | null {
		const documentUrl = speech.speech.document_urls?.[0];
		if (!documentUrl) return null;

		const words = quote.trim().split(/\s+/);
		let searchText = encodeQuotePart(quote);
		if (words.length > 12) {
			const start = encodeQuotePart(words.slice(0, 3).join(' '));
			const end = encodeQuotePart(words.slice(-3).join(' '));
			searchText = `${start},${end}`;
		}

		const separator = documentUrl.includes('#') ? '' : '#';
		return documentUrl + separator + ':~:text=' + searchText;
	}

	let aiSummary = $derived(aiViewEnabledStore.value ? speech.ai_summary : null);
	let glossary = $derived(aiSummary?.full_speech_summary.glossary ?? null);
	let keyPoints = $derived(aiSummary?.full_speech_summary.key_points ?? []);

	/** Ein Detailgrad: Beschriftung für den Slider + der zugehörige Text. */
	interface DetailLevel {
		label: string;
		text: string;
	}

	let detailLevels: DetailLevel[] = $derived(
		[
			{ label: 'Ein Satz', text: aiSummary?.one_sentence_short_summary },
			{ label: 'Sehr kurz', text: aiSummary?.very_short_summary },
			{ label: 'Kurz', text: aiSummary?.short_summary },
			{ label: 'Normal', text: aiSummary?.full_speech_summary.summary },
			{ label: 'Lang', text: aiSummary?.detailed_summary },
			{ label: 'Sehr lang', text: aiSummary?.very_detailed_summary }
		].filter((level): level is DetailLevel => !!level.text?.trim())
	);

	let detailIndex = $state(speechDetailLevelStore.value);

	let safeDetailIndex = $derived(Math.min(detailIndex, Math.max(detailLevels.length - 1, 0)));
	$effect(() => {
		speechDetailLevelStore.value = detailIndex;
	});

	function stanceColor(stance: Opinion | null): string {
		if (stance === Opinion.Pro) return 'bg-green-600';
		if (stance === Opinion.Contra) return 'bg-red-500';
		return 'bg-gray-400';
	}

	function relationLabel(relation: DbSpeechRelations) {
		const { speech_related_to_proposal_summary, speech_related_to_detailed_proposal_summary } =
			relation.full_speech_relations;
		if (speech_related_to_proposal_summary)
			return { icon: '✓', text: 'Behandelt diesen Antrag', class: 'bg-green-600 text-white' };
		if (speech_related_to_detailed_proposal_summary)
			return {
				icon: '~',
				text: 'Behandelt nur Details des Antrags',
				class: 'bg-yellow-400 text-black'
			};
		return {
			icon: '!',
			text: 'Bezieht sich kaum auf diesen Antrag',
			class: 'bg-orange-600 text-white'
		};
	}

	let speechDuration = $derived.by(() => {
		const seconds = speech.speech.duration_in_seconds;
		if (!seconds) return null;
		const mins = Math.floor(seconds / 60);
		return `${mins}:${(seconds - mins * 60).toString().padStart(2, '0')} min`;
	});
</script>

{#snippet pointText(text: string)}
	<span class="text-base text-gray-800 dark:text-gray-200">
		{#if glossary}
			<GlossaryText {text} {glossary} />
		{:else}
			{text}
		{/if}
	</span>
{/snippet}

<Dialog.Root bind:open>
	<Dialog.Portal>
		<Dialog.Overlay
			class="fixed inset-0 z-70 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
		/>
		<Dialog.Content
			class="fixed top-[50%] left-[50%] z-70 h-[90vh] w-4xl max-w-[90%] translate-x-[-50%] translate-y-[-50%] overflow-y-auto rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-primary-300"
		>
			<div class="flex flex-col gap-4 p-5 text-black lg:p-8">
				<div class="flex items-start justify-between gap-3">
					<div class="min-w-0">
						{#if header}
							{@render header()}
						{/if}
						<h1 class="text-xl font-bold lg:text-2xl">
							{#if aiSummary}
								<AiSummaryHintPopup
									{aiSummary}
									align="start"
									aiGenText="Titel, Zusammenfassungen, Schwerpunkte und Bezüge zu Abstimmungen wurden mittels KI aus der Rede erstellt."
								/>
								{aiSummary.short_title}
							{:else}
								{speech.speech.about ?? 'Rede'}
							{/if}
						</h1>
						<div class="mt-1 flex flex-wrap items-center gap-3 text-sm text-gray-700">
							{#if speechDuration}
								<span class="flex items-center gap-1">
									<span
										class="h-4 w-4 shrink-0 [&_path]:stroke-current [&>svg]:h-full [&>svg]:w-full"
									>
										{@html clockIcon}
									</span>
									{speechDuration}
								</span>
							{/if}
							{#each speech.speech.document_urls ?? [] as url (url)}
								<a href={url} target="_blank" class="underline">Redeprotokoll öffnen</a>
							{/each}
						</div>
					</div>
					<Dialog.Close>
						<ModalCloseButton />
					</Dialog.Close>
				</div>

				{#if aiSummary}
					<div class="rounded-xl bg-primary-300 px-5 py-3 dark:bg-primary-500">
						<div class="flex flex-wrap items-center gap-x-3 gap-y-1">
							<h2 class="text-lg font-semibold md:text-xl">Zusammenfassung</h2>
							{#if detailLevels.length > 1}
								<Select.Root
									type="single"
									value={safeDetailIndex.toString()}
									onValueChange={(value) => (detailIndex = Number(value))}
									items={detailLevels.map((level, i) => ({
										value: i.toString(),
										label: level.label
									}))}
								>
									<Select.Trigger
										class="flex touch-manipulation items-center rounded-lg bg-primary-600 px-2 py-0.5 text-xs text-white transition-colors focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none [&>svg]:ml-1 [&>svg]:size-3"
									>
										<span class="truncate">{detailLevels[safeDetailIndex].label}</span>
										{@html upDownArrowIcon}
									</Select.Trigger>
									<Select.Portal>
										<Select.Content
											class="z-500 max-h-60 w-[150px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-lg border border-gray-200 bg-surface-100 shadow-lg dark:bg-surface-500"
											sideOffset={6}
										>
											<Select.Viewport class="p-1">
												{#each detailLevels as level, i (i)}
													<Select.Item
														class="flex h-8 w-full cursor-pointer items-center rounded-md py-2 pr-1.5 pl-2.5 text-xs transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
														value={i.toString()}
														label={level.label}
													>
														{#snippet children({ selected })}
															<div class="flex items-center gap-2">
																{level.label}
															</div>
															{#if selected}
																<div class="ml-auto h-4 stroke-black dark:stroke-white">
																	{@html checkmarkSmall}
																</div>
															{/if}
														{/snippet}
													</Select.Item>
												{/each}
											</Select.Viewport>
										</Select.Content>
									</Select.Portal>
								</Select.Root>
							{/if}
						</div>

						<p class="text-base text-gray-800 dark:text-gray-200">
							{#if detailLevels.length === 0}
								Keine Zusammenfassung vorhanden.
							{:else if glossary}
								<GlossaryText text={detailLevels[safeDetailIndex].text} {glossary} />
							{:else}
								{detailLevels[safeDetailIndex].text}
							{/if}
						</p>
					</div>

					{#if keyPoints.length > 0}
						<div class="rounded-xl bg-primary-300 px-5 py-3 dark:bg-primary-500">
							<h2 class="text-lg font-semibold md:text-xl">Schwerpunkte der Rede</h2>
							<ul class="mt-2 flex flex-col gap-4">
								{#each keyPoints as keyPoint, i (i)}
									<!-- Ohne Zitat gibt es nichts aufzuklappen. -->
									{@const quote = keyPoint.unmodified_reference_point?.trim()}
									{@const isOpen = expandedKeyPoints.has(i)}
									<li>
										<div class="flex gap-2">
											<span class="mt-2 h-2 w-2 shrink-0 rounded-full bg-primary-500"></span>
											<div class="min-w-0 flex-1">
												{#if quote}
													<button
														type="button"
														class="flex w-full cursor-pointer items-start gap-2 text-left"
														aria-expanded={isOpen}
														onclick={() => toggleKeyPoint(i)}
													>
														<span class="min-w-0 flex-1">
															{@render pointText(keyPoint.summarized_point)}
														</span>
														<span
															aria-hidden="true"
															class="mt-1.5 h-3 w-3 shrink-0 text-gray-700 transition-transform duration-300 dark:text-gray-300 [&>svg]:h-full [&>svg]:w-full {isOpen
																? 'rotate-180'
																: ''}"
														>
															{@html downArrowIcon}
														</span>
													</button>

													{#if isOpen}
														{@const link = protocolLink(quote)}
														<blockquote
															transition:slide={{ duration: 240 }}
															class="mt-2 flex items-center gap-2 rounded-lg border-l-4 border-secondary-500 bg-primary-200 py-3 pr-4 pl-3 dark:bg-primary-400"
														>
															<!-- Rein dekorativ. Das Zeichen sitzt oben in seiner Zeilenbox,
															     das mt-2 schiebt es optisch auf die Mitte des Zitats. -->
															<span
																aria-hidden="true"
																class="mt-2 shrink-0 self-center font-serif text-4xl leading-none text-secondary-500 select-none"
															>
																“
															</span>
															{#if link}
																<a
																	href={link}
																	target="_blank"
																	rel="noopener"
																	title="Stelle im Protokoll öffnen"
																	class="group flex min-w-0 flex-1 items-center gap-2"
																>
																	<p
																		class="text-sm text-gray-800 italic group-hover:underline dark:text-gray-200"
																	>
																		{quote}
																	</p>
																	<span
																		aria-hidden="true"
																		class="h-4 w-4 shrink-0 self-start text-gray-700 dark:text-gray-300 [&>svg]:h-full [&>svg]:w-full"
																	>
																		{@html linkIcon}
																	</span>
																</a>
															{:else}
																<p class="text-sm text-gray-800 italic dark:text-gray-200">
																	{quote}
																</p>
															{/if}
														</blockquote>
													{/if}
												{:else}
													{@render pointText(keyPoint.summarized_point)}
												{/if}
											</div>
										</div>
									</li>
								{/each}
							</ul>
						</div>
					{/if}
				{/if}

				{#if loadingVoteResults}
					<ExpandablePlaceholder />
				{:else if relatedVoteResults.length > 0}
					<div>
						<h2 class="text-lg font-semibold md:text-xl">Bezieht sich auf</h2>
						{#each relatedVoteResults as { relation, voteResult } (relation.id)}
							{#if voteResult}
								{@const label = relationLabel(relation)}
								<div class="mt-3">
									<div class="mb-1 flex flex-wrap items-center gap-2">
										<span
											class="flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-semibold {label.class}"
										>
											<span aria-hidden="true" class="font-bold">{label.icon}</span>
											{label.text}
										</span>
										{#if relation.full_speech_relations.stance_to_proposal}
											<span
												class="rounded-full px-2 py-0.5 text-xs font-semibold text-white {stanceColor(
													relation.full_speech_relations.stance_to_proposal
												)}"
											>
												{relation.full_speech_relations.stance_to_proposal}
											</span>
										{/if}
									</div>
									<div class="flex items-stretch gap-2">
										<div
											class="w-1.5 shrink-0 rounded-full {stanceColor(
												relation.full_speech_relations.stance_to_proposal
											)}"
										></div>
										<VoteResultExpandableBar {voteResult} class="min-w-0 flex-1" />
									</div>
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
