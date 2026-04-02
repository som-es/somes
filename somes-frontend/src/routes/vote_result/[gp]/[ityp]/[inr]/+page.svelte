<script lang="ts">
	import { errorToNull, get_eurovoc_topics, vote_result_by_path, url } from '$lib/api/api';
	import {
		currentDelegateStore,
		currentVoteResultStore,
		hasGoBackStore,
		aiViewEnabledStore
	} from '$lib/stores/stores';
	import { onMount } from 'svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { Delegate, VoteResult } from '$lib/types';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import VoteDelegateCard from '$lib/components/Delegates/VoteDelegateCard.svelte';
	import {
		genCirclesWithNamedVoteInfo,
		genCirclesWithSpeechInfo,
		type Bubble
	} from '$lib/parliament';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import SimpleYesNo from '$lib/components/VoteResults/SimpleYesNo/SimpleYesNo.svelte';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { cachedLegisInitFavos } from '$lib/caching/favos';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import FetchDelegateCard from '$lib/components/Delegates/FetchDelegateCard.svelte';
	import VoteResultIdBar from '$lib/components/Bars/VoteResultIdBar.svelte';
	import Documents from '$lib/components/Documents/Documents.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import InfoBadges from '$lib/components/VoteResults/InfoTiles/InfoBadges.svelte';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';
	import { page } from '$app/state';
	import linkIcon from '$lib/assets/misc_icons/external-link.svg?raw';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import DelegateListItem from '$lib/components/Delegates/DelegateListItem.svelte';
	import { Select } from 'bits-ui';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';

	let gp = $derived(page.params.gp);
	let ityp = $derived(page.params.ityp);
	let inr = $derived(page.params.inr);

	import type { PageProps } from './$types';
	import { browser } from '$app/environment';
	import { partyColors } from '$lib/partyColor';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import type { SvelteSet } from 'svelte/reactivity';
	import { addLegisInitFavo, removeLegisInitFavo } from '$lib/api/authed';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	let { data }: PageProps = $props();

	let voteResult: VoteResult | null = $derived(errorToNull(data.voteResult));
	let delegates: Delegate[] = $derived(data.delegates ?? []);

	let delegate: Delegate | null = $state(null);
	let selectedBubble: Bubble | undefined = $state();
	let searchValue: string = $state('');

	// Search PopUp Logic
	let showMobileSearch: boolean = $state(false);

	let isSearchPopupOpen = $state(false);
	let searchWrapper: HTMLDivElement | undefined = $state();
	function handleFocusOut(e: FocusEvent) {
		const relatedTarget = e.relatedTarget as Node | null;
		if (relatedTarget) {
			if (searchWrapper?.contains(relatedTarget)) return;
			if ((relatedTarget as Element).closest('.search-filter-portal')) return;
		}
		isSearchPopupOpen = false;
	}

	let selectedPartiesNames = $state<string[]>([]);
	let uniqueParties = $derived.by(() => {
		const parties = new Set<string>();
		delegates.forEach((d) => parties.add(d.party?.trim() ? d.party : 'Ohne Klub'));
		return Array.from(parties).map((party) => ({
			name: party,
			color: partyColors.get(party) ?? '#ccc'
		}));
	});

	let filteredDelegates = $derived.by(() => {
		let res = delegates;
		if (searchValue) {
			res = res.filter((d) => d.name.toLowerCase().includes(searchValue.toLowerCase()));
		}
		if (selectedPartiesNames.length > 0) {
			res = res.filter((d) => {
				const p = d.party?.trim() ? d.party : 'Ohne Klub';
				return selectedPartiesNames.includes(p);
			});
		}
		return res;
	});

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let inputValue = '';

	let generalSpeechDelegates = $derived.by(() => {
		const res = voteResult ? genCirclesWithSpeechInfo(voteResult.speeches, delegates) : [];
		return res;
	});

	let generalNamedVoteDelegates: Bubble[] | null = $derived(
		voteResult && voteResult.named_votes
			? genCirclesWithNamedVoteInfo(voteResult.named_votes.named_votes, delegates)
			: []
	);

	let generalAbsencesDelegates: Bubble[] | null = $state(null);

	let description = $derived(voteResult?.legislative_initiative?.description);

	let issuedByDels = $derived.by(() => {
		if (voteResult?.issued_by_dels) {
			const issuedByDels = new Map<string, number[]>();
			voteResult.issued_by_dels.forEach((del) => {
				const text = del.text ? del.text : 'Abgeordnete';
				if (issuedByDels.has(text)) {
					issuedByDels.get(text)?.push(del.delegate_id);
				} else {
					issuedByDels.set(text, [del.delegate_id]);
				}
			});
			return issuedByDels;
		} else {
			return new Map<string, number[]>();
		}
	});

	const goBack = () => {
		history.back();
	};

	const updateAutocompletion = () => {
		autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates, [], voteResult);
	};

	const selectRandomlyFromDels = () => {
		delegate = delegates[Math.floor(Math.random() * delegates.length)];

		const maybeStoredDelegate = currentDelegateStore.value;
		if (maybeStoredDelegate) {
			const foundDel = delegates.find((del) => del.id == maybeStoredDelegate.id);
			if (foundDel) {
				delegate = foundDel;
			}
		}
	};

	let legisInitFavos: SvelteSet<number> | null = $state(null);

	const runVoteResultUpdate = async () => {
		legisInitFavos = await cachedLegisInitFavos();

		if (!delegates) {
			return;
		}

		// enrichDelegates(delegates);

		selectRandomlyFromDels();
		updateAutocompletion();
	};

	onMount(runVoteResultUpdate);

	let currentlyUpdating = $state(false);

	function delegateFilter(): AutocompleteOption<string>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return delegateFilterOptions(_options, _inputValue);
	}

	function onDelegateSelection(event: CustomEvent<AutocompleteOption<string>>): void {
		// @ts-ignore
		delegate = event.detail.meta;
		inputValue = event.detail.label;
	}

	/*run(() => {
		if (delegates || voteResult) {
			updateAutocompletion();
			selectRandomlyFromDels();
			enrichDelegates(delegates);
		}
	});*/

	let parliamentUrl = $derived(
		`https://parlament.gv.at/gegenstand/${gp}/${ityp}/${inr}?utm_source=somes.at`
	);
	let documents = $derived(voteResult?.documents ?? []);
	let votedByName = $derived(voteResult?.legislative_initiative?.voted_by_name ?? false);
	let couldExtractNamedVotes = $derived(
		(voteResult?.named_votes?.named_votes?.length ?? 0) > 0 && votedByName
	);
</script>

<svelte:head>
	<title>Abstimmungsergebnis</title>
	<meta name="description" content="Spezifisches Abstimmungsergebnis" />
</svelte:head>

{#if browser}
	<title>
		{#if voteResult}
			{#if aiViewEnabledStore.value && voteResult.ai_summary}
				{voteResult.ai_summary.short_title}
			{:else}
				{description}
			{/if}
		{/if}
	</title>
{/if}

<Container>
	{#if voteResult}
		{#if currentlyUpdating}
			<!-- <CenterPrograssRadial /> -->
		{:else}
			<br />
			<div class="grid-container-with-emphasis flex gap-3">
				<div class="title-item rounded-xl bg-primary-300 px-6 py-5 dark:bg-primary-500">
					<!-- Title, Date and Result Icon -->
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-4">
							<!-- Title & Date Stack -->
							<div class="flex flex-col">
								<div class="flex items-start gap-2">
									<span
										class="text-xl leading-tight font-bold lg:text-3xl"
										style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
									>
										{#if aiViewEnabledStore.value && voteResult.ai_summary}
											<AiSummaryHintPopup aiSummary={voteResult.ai_summary} />
											{voteResult.ai_summary.short_title}
										{:else}
											{description}
										{/if}
									</span>
								</div>

								{#if voteResult.legislative_initiative.accepted && voteResult.legislative_initiative.vote_date}
									<span class="text-sm opacity-90">
										{voteResult.legislative_initiative.voted_by_name ? 'namentlich ' : ''}
										abgestimmt am {dashDateToDotDate(
											voteResult.legislative_initiative.vote_date.toString()
										)}
									</span>
								{/if}
							</div>
						</div>

						<!-- Right Actions, Result Icon and  Star -->
						<div class="flex flex-shrink-0 flex-wrap items-center gap-2">
							<a href={parliamentUrl} target="_blank" class="w-5 text-gray-500 dark:text-gray-300">
								{@html linkIcon}
							</a>
							<!-- Result Icon -->
							{#if voteResult.legislative_initiative.accepted}
								<div class="shrink-0">
									{#if voteResult.legislative_initiative.accepted == 'a'}
										<span
											class="block stroke-green-600 dark:stroke-green-500"
											style="width:34px; height:34px;"
										>
											{@html checkmarkIcon}
										</span>
									{:else}
										<span class="block" style="width:34px; height:34px;">
											{@html crossmarkIcon}
										</span>
									{/if}
								</div>
							{/if}
							{#if legisInitFavos}
								<button
									onclick={async () => {
										if (!voteResult || !legisInitFavos) return;

										if (legisInitFavos.has(+voteResult.legislative_initiative.id)) {
											const res = await removeLegisInitFavo({
												vote_result_id: +voteResult.legislative_initiative.id
											});
											if (res === null) {
												legisInitFavos.delete(+voteResult.legislative_initiative.id);
											}
										} else {
											const res = await addLegisInitFavo({
												vote_result_id: +voteResult.legislative_initiative.id
											});
											if (res === null) {
												legisInitFavos.add(+voteResult.legislative_initiative.id);
											}
										}
									}}
									class="w-14 p-2"
								>
									{@html legisInitFavos.has(+voteResult.legislative_initiative.id)
										? starFilled
										: star}
								</button>
							{/if}
						</div>
					</div>

					<!-- Zusammenfassung -->
					{#if aiViewEnabledStore.value && voteResult.ai_summary}
						<div class="mt-5 pb-3">
							<h1 class="text-lg font-semibold md:text-xl">Zusammenfassung</h1>
							<span class="text-base text-gray-800 lg:text-base dark:text-gray-200">
								<GlossaryText
									text={voteResult.ai_summary.short_summary}
									glossary={voteResult.ai_summary.full_summary.glossary}
								/>
							</span>
						</div>
					{/if}

					<div class="flex w-full flex-wrap items-center justify-between gap-3 pt-1">
						<div>
							<InfoBadges {voteResult} />
						</div>

						<div class="flex flex-1 justify-end">
							{#if aiViewEnabledStore.value && voteResult.ai_summary && voteResult.eurovoc_topics.length == 0}
								<Topics
									topics={voteResult.ai_summary.full_summary.topics
										.sort((a, b) => {
											return a.length - b.length;
										})
										.map((topic) => {
											return { topic };
										})}
								/>
							{:else}
								<Topics
									topics={voteResult.eurovoc_topics.sort((a, b) => {
										return a.topic.length - b.topic.length;
									})}
								/>
							{/if}
						</div>
					</div>
				</div>

				<!-- CARD main topics  -->
				{#if aiViewEnabledStore.value && voteResult.ai_summary}
					<div class="emphasis-item">
						<Emphasis
							emphasis={voteResult.ai_summary.full_summary.key_points}
							glossary={voteResult.ai_summary.full_summary.glossary}
						></Emphasis>
					</div>
				{/if}

				{#snippet searchContent(onClose: () => void)}
					<div class="mt-4 lg:mt-0">
						<span class="text-sm font-semibold text-gray-800 lg:text-base dark:text-gray-200"
							>Filter</span
						>
						<div class="mt-2 flex h-10 w-full gap-2 md:mt-1 md:w-auto">
							<!-- Parteien Filter -->
							<div
								class="flex h-full grow touch-manipulation items-center justify-center gap-1 lg:grow-0"
							>
								<Select.Root
									type="multiple"
									bind:value={selectedPartiesNames}
									items={uniqueParties.map((p) => ({ value: p.name, label: p.name }))}
								>
									<Select.Trigger
										class="flex h-full w-full touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 text-white transition-colors placeholder:text-gray-600 focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none lg:w-auto lg:px-3"
									>
										<div class="flex items-center gap-2">
											{#each selectedPartiesNames.slice(0, 1) as partyName}
												{@const party = uniqueParties.find((p) => p.name === partyName)}
												{#if party}
													<div
														class="h-3 w-3 rounded-full"
														style="background-color: {party.color};"
													></div>
													<span class="truncate">{party.name}</span>
												{/if}
											{/each}
											{#if selectedPartiesNames.length > 1}
												<span class="truncate">+{selectedPartiesNames.length - 1}</span>
											{/if}
											{#if selectedPartiesNames.length === 0}
												<span class="truncate">Alle Parteien</span>
											{/if}
										</div>
										{@html upDownArrowIcon}
									</Select.Trigger>
									<Select.Portal>
										<Select.Content
											class="z-500 max-h-60 w-[calc(100vw-2rem)] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg md:w-[200px] dark:bg-surface-500"
											sideOffset={8}
										>
											<Select.Viewport class="p-1">
												{#each uniqueParties as party}
													<Select.Item
														class="flex h-10 w-full cursor-pointer justify-between rounded-lg py-3 pr-1.5 pl-3 text-sm capitalize transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
														value={party.name}
														label={party.name}
													>
														{#snippet children({ selected })}
															<div class="flex items-center gap-2">
																<div
																	class="h-3 w-3 rounded-full"
																	style="background-color: {party.color};"
																></div>
																{party.name}
															</div>
															{#if selected}
																<div class="ml-auto h-4 stroke-black dark:stroke-white">
																	{@html checkmarkIcon}
																</div>
															{/if}
														{/snippet}
													</Select.Item>
												{/each}
											</Select.Viewport>
										</Select.Content>
									</Select.Portal>
								</Select.Root>
							</div>
						</div>
					</div>

					<!-- Search Results -->
					<div class="mt-3 max-h-[50vh] overflow-y-auto lg:mt-4 lg:max-h-70">
						{#if isSearchPopupOpen}
							<div class="mb-1">
								<span class="text-sm font-semibold text-gray-800 lg:text-base dark:text-gray-200"
									>Suchergebnisse</span
								>
							</div>
							<div class="space-y-2">
								{#each filteredDelegates as del (del.id)}
									{@const namedVote = generalNamedVoteDelegates?.find((b) => b.del?.id === del.id)}
									{@const partyVoteInfo = voteResult?.votes.find((v) => v.party === del.party)}
									<DelegateListItem
										delegate={del}
										class="w-full bg-primary-200 lg:bg-primary-300 dark:bg-surface-600 dark:lg:bg-primary-500"
										onclick={() => {
											delegate = del;
											selectedBubble = undefined;
											onClose();
										}}
									>
										{#if namedVote && namedVote.namedVote}
											{#if namedVote.namedVote.infavor}
												<span
													class="inline-block stroke-green-600 dark:stroke-green-500"
													style="width:24px; height:24px;">{@html checkmarkIcon}</span
												>
											{:else if namedVote.namedVote.was_absent}
												<span class="text-xs font-medium text-gray-500 dark:text-gray-200"
													>Nicht abgestimmt</span
												>
											{:else}
												<span class="inline-block" style="width:24px; height:24px;"
													>{@html crossmarkIcon}</span
												>
											{/if}
										{:else if partyVoteInfo}
											{#if partyVoteInfo.infavor}
												<span
													class="inline-block stroke-green-600 opacity-60 dark:stroke-green-500"
													style="width:24px; height:24px;">{@html checkmarkIcon}</span
												>
											{:else}
												<span class="inline-block opacity-60" style="width:24px; height:24px;"
													>{@html crossmarkIcon}</span
												>
											{/if}
										{/if}
									</DelegateListItem>
								{/each}
								{#if filteredDelegates.length === 0}
									<div class="p-4 text-center text-gray-500">Keine Ergebnisse gefunden</div>
								{/if}
							</div>
						{/if}
					</div>
				{/snippet}

				<!-- Mini Parlament and Vote Results-->
				{#if voteResult && voteResult.votes.length > 0}
					<div class="emphasis-item rounded-xl bg-primary-300 px-5 pt-5 pb-3 dark:bg-primary-500">
						<!-- Desktop Search PopUp -->
						<div
							class="relative mb-3 hidden lg:block"
							bind:this={searchWrapper}
							onfocusout={handleFocusOut}
						>
							<SearchBar
								onfocus={() => (isSearchPopupOpen = true)}
								onclick={() => (isSearchPopupOpen = true)}
								oninput={() => (isSearchPopupOpen = true)}
								bind:searchValue
								placeholder="Suche nach Abgeordneten..."
							/>

							{#if isSearchPopupOpen}
								<div
									class="absolute top-full right-0 left-0 z-100 mt-2 w-[98%] rounded-xl border border-gray-300 bg-surface-50 px-5 pt-4 pb-5 shadow-lg md:w-140 dark:bg-surface-600"
									data-popup="popupSearch"
									role="button"
									tabindex="0"
									onmousedown={(e) => e.preventDefault()}
								>
									{@render searchContent(() => {
										isSearchPopupOpen = false;
									})}
								</div>
							{/if}
						</div>

						<!-- Mobile Search Overlay -->
						{#if showMobileSearch}
							<div
								class="fixed inset-0 z-50 flex items-start justify-center bg-black/50 p-4 backdrop-blur-sm lg:hidden"
							>
								<div
									class="w-full max-w-md rounded-2xl bg-primary-100 p-4 shadow-xl dark:bg-primary-600"
								>
									<div class="mb-4 flex items-center justify-between">
										<h3 class="text-lg font-semibold">Suche</h3>
										<ModalCloseButton class="p-1" onclick={() => (showMobileSearch = false)} />
									</div>
									<SearchBar
										bind:searchValue
										placeholder="Suche nach Abgeordneten..."
										onfocus={() => (isSearchPopupOpen = true)}
										onclick={() => (isSearchPopupOpen = true)}
										oninput={() => (isSearchPopupOpen = true)}
										autofocus={true}
									/>

									{@render searchContent(() => {
										showMobileSearch = false;
									})}
								</div>
							</div>
						{/if}

						<div class="flex">
							<!-- Abstimmung, Fractions, Result - Mobile -->
							<div class="hidden w-full max-lg:block">
								<div class="flex w-full items-center justify-between">
									<h3 class="text-lg leading-none font-semibold md:text-xl">Abstimmung</h3>
									<button
										class="flex items-center justify-center"
										onclick={() => {
											showMobileSearch = true;
											isSearchPopupOpen = true;
										}}
									>
										<div class="flex h-6 w-6 items-center text-gray-800 dark:text-gray-200">
											{@html searchIcon}
										</div>
									</button>
								</div>

								<div class="mt-2 flex flex-col gap-4">
									<!-- In Favor -->
									<div class="rounded-xl bg-primary-200/50 p-3 dark:bg-primary-600/50">
										<div class="mb-2 flex items-center gap-2">
											<span class="inline-block stroke-green-600" style="width:20px; height:20px;"
												>{@html checkmarkIcon}</span
											>
											<span class="font-semibold">Dafür</span>
										</div>
										<div class="flex flex-col gap-2 pl-2">
											{#each voteResult.votes
												.slice()
												.sort((a, b) => b.fraction - a.fraction) as vote}
												{#if vote.infavor}
													<div class="flex items-center justify-between">
														<div class="flex items-center gap-2">
															<div
																class="h-2.5 w-2.5 rounded-full"
																style="background-color: {partyColors.get(vote.party) ?? '#ccc'};"
															></div>
															<span class="text-base font-medium">{vote.party}</span>
														</div>
														<span class="text-base font-medium">({vote.fraction})</span>
													</div>
												{/if}
											{/each}
										</div>
									</div>

									<!-- Against -->
									<div class="rounded-xl bg-primary-200/50 p-3 dark:bg-primary-600/50">
										<div class="mb-2 flex items-center gap-2">
											<span class="inline-block stroke-red-600" style="width:20px; height:20px;"
												>{@html crossmarkIcon}</span
											>
											<span class="font-semibold">Dagegen</span>
										</div>
										<div class="flex flex-col gap-2 pl-2">
											{#each voteResult.votes
												.slice()
												.sort((a, b) => b.fraction - a.fraction) as vote}
												{#if !vote.infavor}
													<div class="flex items-center justify-between">
														<div class="flex items-center gap-2">
															<div
																class="h-2.5 w-2.5 rounded-full"
																style="background-color: {partyColors.get(vote.party) ?? '#ccc'};"
															></div>
															<span class="text-base font-medium">{vote.party}</span>
														</div>
														<span class="text-base font-medium">({vote.fraction})</span>
													</div>
												{/if}
											{/each}
										</div>
									</div>
								</div>
							</div>

							<!-- Abstimmung, Fractions, Result and Mini Parlament - Desktop-->
							<div class="absolute ml-1 max-lg:hidden">
								<h3 class="mb-1 text-lg font-semibold md:text-xl">Abstimmung</h3>
								<div class="ml-1">
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										<div class="flex items-center justify-between gap-4">
											<div class="flex items-center gap-2">
												<div
													class="h-2.5 w-2.5 rounded-full"
													style="background-color: {partyColors.get(vote.party) ?? '#ccc'};"
												></div>
												<span class="text-sm lg:text-base">{vote.party}</span>
											</div>
											<div class="flex items-center gap-1">
												<span class="text-sm lg:text-base">({vote.fraction})</span>
												{#if vote.infavor}
													<span
														class="inline-block stroke-green-600 align-middle dark:stroke-green-500"
														style="width:18px; height:18px;">{@html checkmarkIcon}</span
													>
												{:else}
													<span class="inline-block align-middle" style="width:18px; height:18px;"
														>{@html crossmarkIcon}</span
													>
												{/if}
											</div>
										</div>
									{/each}
								</div>
							</div>

							<div class="flex w-full items-center justify-center max-lg:hidden">
								<div class="w-2/3">
									<VoteParliament2
										{voteResult}
										bind:delegate
										{delegates}
										allSeats={data.cachedSeats}
										bind:selected={selectedBubble}
										noSeats={!data.hasSeatInfo}
										useOffset={data.hasSeatInfo}
										showGovs
										overrideDelegates
										{searchValue}
									/>
								</div>
							</div>

							<div class="mt-1 w-100 max-lg:hidden">
								{#if selectedBubble}
									<VoteDelegateCard
										bubble={selectedBubble}
										gp={voteResult.legislative_initiative.gp}
										date={voteResult.legislative_initiative.vote_date ??
											voteResult.legislative_initiative.nr_plenary_activity_date}
									/>
								{/if}
							</div>
						</div>

						<!-- Eingebracht von -->
						{#if issuedByDels.size > 0}
							<!-- Divider -->
							<hr class="my-4 border-t border-gray-400 dark:border-gray-600" />
							<div class="mb-1">
								<h3 class="text-md font-semibold md:text-lg">Eingebracht von</h3>
								<div class="mt-1 flex flex-col gap-2 md:flex-row md:flex-wrap md:gap-3">
									{#each Array.from(issuedByDels.entries()) as [text, delegate_ids]}
										{#each delegate_ids as delegate_id}
											{@const del = delegates.find((d) => d.id === delegate_id)}
											{#if del}
												<DelegateListItem
													delegate={del}
													class="w-full md:w-auto md:max-w-full"
													onclick={() => {
														delegate = del;
														selectedBubble = undefined;
													}}
												/>
											{/if}
										{/each}
									{/each}
								</div>
							</div>
						{/if}
					</div>
				{:else if issuedByDels.size > 0}
					<div class="emphasis-item rounded-xl bg-primary-300 px-5 pt-3 pb-3 dark:bg-primary-500">
						<h3 class="text-md font-semibold md:text-lg">Eingebracht von</h3>
						<div class="mt-1 flex flex-col gap-2 md:flex-row md:flex-wrap md:gap-3">
							{#each Array.from(issuedByDels.entries()) as [text, delegate_ids]}
								{#each delegate_ids as delegate_id}
									{@const del = delegates.find((d) => d.id === delegate_id)}
									{#if del}
										<DelegateListItem
											delegate={del}
											class="w-full md:w-auto md:max-w-full"
											onclick={() => {
												delegate = del;
												selectedBubble = undefined;
											}}
										/>
									{/if}
								{/each}
							{/each}
						</div>
					</div>
				{/if}

				<!-- {#if voteResult.named_votes}
					<div
						class="text-lg named-vote-info-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3"
					>
						abgegebene Stimmen: <span class="font-bold"
							>{voteResult.named_votes.named_vote_info.given_vote_sum}</span
						>, Ja-Stimmen:
						<span class="font-bold">{voteResult.named_votes.named_vote_info.pro_count}</span>,
						Nein-Stimmen:
						<span class="font-bold">{voteResult.named_votes.named_vote_info.contra_count}</span>
						{#if voteResult.named_votes.named_vote_info.invalid_count > 0}
							Ungültige Stimmen:
							<span class="font-bold">{voteResult.named_votes.named_vote_info.invalid_count}</span>
						{/if}
					</div>
				{/if}

				{#if voteResult.legislative_initiative.accepted}
					<div
						class="simple-yes-no-item bg-primary-300 p-3 dark:bg-primary-500 rounded-xl flex flex-wrap justify-between"
					>
						{#if votedByName && !couldExtractNamedVotes}
							Namentliche Abstimmungsergebnisse konnten nicht extrahiert werden.
						{:else}
							<SimpleYesNo votes={voteResult.votes.slice()} />
						{/if}
					</div>



					<div class="z-20! search-item base-font-color space-y-5">
						<input
							class="rounded-xl! w-full h-12 px-2 input"
							type="search"
							name="ac-demo"
							bind:value={inputValue}
							placeholder="Suchen..."
							use:popup={popupSettings}
						/>

						{#if autocompleteOptions}
							<div class="z-10! card max-h-64 p-4 overflow-y-auto" data-popup="popupAutocomplete">
								<Autocomplete
									bind:input={inputValue}
									options={autocompleteOptions}
									on:selection={onDelegateSelection}
									emptyState={'Keine Person gefunden'}
									filter={delegateFilter}
								/>
							</div>
						{/if}
					</div>

					<div class="flex flex-wrap min-w-full justify-between">
						<div class="rounded-xl w-full parliament-item flex- bg-primary-200 dark:bg-primary-200">
							<VoteParliament2
								{voteResult}
								bind:delegate
								bind:delegates
								bind:selected={selectedBubble}
								bind:circles2d
								showGovs
								show3D
							/>
						</div>
						{#if selectedBubble}
							<div
								class="max-md:hidden delegate-item rounded-xl bg-primary-300 dark:bg-primary-500"
							>
								<VoteDelegateCard
									bubble={selectedBubble}
									gp={voteResult.legislative_initiative.gp}
									date={voteResult.legislative_initiative.vote_date ?? voteResult.legislative_initiative.nr_plenary_activity_date}
								/>
							</div>
						{/if}
					</div>
				{/if} -->

				<!-- {/if} -->
				<!-- <div class="flex flex-wrap justify-between min-w-full gap-3">
					{#if delegates}
						<div class="md:hidden info-item">
							<InfoTiles {voteResult} dels={delegates} isCenter />
						</div>
						<div class="max-md:hidden info-item">
							<InfoTiles {voteResult} dels={delegates} />
						</div>
					{/if}
				</div> -->

				<div class="flex w-full gap-2 max-lg:flex-wrap">
					<div
						class="flex {voteResult.issued_by_dels.length > 0 ? 'flex-col' : 'flex-row'} gap-2"
						style="flex-basis: {voteResult.issued_by_dels.length > 0 ? '30%' : '100%;'}"
					>
						{#if voteResult.referenced_by_others_ids.length > 0}
							<div class="h-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
								<span class="text-lg font-bold md:text-3xl">Referenziert in</span>
								{#each voteResult.referenced_by_others_ids as refered_by}
									<VoteResultIdBar
										requiringVotes
										on:dataUpdated={(event) => {
											voteResult = { ...event.detail };
										}}
										legis_init_id={refered_by}
									/>
								{/each}
							</div>
						{/if}
						{#if voteResult.references && voteResult.references.length > 0}
							<div class="h-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
								<span class="text-lg font-bold md:text-3xl">
									{#if voteResult.legislative_initiative.ityp == 'AA'}
										Hauptgegenstand
									{:else}
										Bezug zu
									{/if}
								</span>
								{#each voteResult.references as refered_by}
									<VoteResultIdBar
										requiringVotes
										on:dataUpdated={(event) => {
											voteResult = { ...event.detail };
										}}
										legis_init_ref={refered_by}
									/>
								{/each}
							</div>
						{/if}
					</div>
				</div>
				{#if generalSpeechDelegates != null}
					{#if generalSpeechDelegates.length > 0}
						<div class="speeches-item gap-3 rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
							<span class="text-xl font-bold md:text-3xl">Reden</span>
							<div class="mt-3 flex flex-row flex-wrap gap-3">
								{#each generalSpeechDelegates as speechDelegate}
									<div class="w-full max-w-80">
										<VoteDelegateCard
											bubble={speechDelegate}
											gp={voteResult.legislative_initiative.gp}
											date={voteResult.legislative_initiative.vote_date ??
												voteResult.legislative_initiative.nr_plenary_activity_date}
										/>
									</div>
								{/each}
							</div>
						</div>
					{/if}
				{:else}
					{#each { length: voteResult.speeches.length * 4 } as _}
						<ExpandablePlaceholder class="" />
					{/each}
				{/if}
				{#if generalNamedVoteDelegates != null}
					{#if generalNamedVoteDelegates.length > 0}
						<div class="speeches-item gap-3 rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
							<span class="text-3xl font-bold">namentliche Abstimmungsergebnisse</span>
							<div class="mt-3 flex flex-row flex-wrap gap-3">
								{#each generalNamedVoteDelegates as namedVoteDelegate}
									<div>
										<VoteDelegateCard
											class="w-80"
											bubble={namedVoteDelegate}
											gp={voteResult.legislative_initiative.gp}
											date={voteResult.legislative_initiative.vote_date ??
												voteResult.legislative_initiative.nr_plenary_activity_date}
										/>
									</div>
								{/each}
							</div>
						</div>
					{/if}
				{:else}
					{#each { length: 5 } as _}
						<ExpandablePlaceholder class="" />
					{/each}
				{/if}
				<div class="w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
					<Documents {documents} />
				</div>
			</div>
		{/if}
	{:else}
		{#each { length: 10 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if}
</Container>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 11px;
		gap: 10px;
	}
	/* .grid-container-with-emphasis {
		box-sizing: border-box;
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 2fr;
		grid-template-rows: auto auto 2fr auto auto;
		grid-template-areas:
            'ti ti'
            'e e'
			'p d'
            'r r'
			'i t';
		padding: 10px;
	} */

	.grid-container-with-emphasis {
		display: flex;
		flex-wrap: wrap;
	}

	.title-item {
		grid-area: ti;
		flex-basis: 100%;
	}

	@media (min-width: 768px) {
		.parliament-item {
			grid-area: p;
			flex-basis: 66%;
		}
	}

	@media (min-width: 768px) {
		.delegate-item {
			grid-area: d;
			flex-basis: 33%;
		}
	}

	.topics-item {
		grid-area: t;
		/* flex-basis: 40%; */
	}

	.emphasis-item {
		grid-area: e;
		flex-basis: 100%;
	}

	.info-item {
		grid-area: i;
		/* flex-basis: 60%; */
	}
	.search-item {
		grid-area: search;
		flex-basis: 100%;
	}

	.simple-yes-no-item {
		grid-area: eyn;
		flex-basis: 100%;
	}

	.named-vote-info-item {
		grid-area: nvi;
		flex-basis: 100%;
	}

	.speeches-item {
		grid-area: speeches;
		flex-basis: 100%;
	}

	.grid-container-without-emphasis {
		/* box-sizing: border-box; */
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 1fr;
		grid-template-rows: auto 2fr auto auto;
		grid-template-areas:
			'ti ti'
			'p d'
			'r r'
			'i t';
		padding: 10px;
	}
</style>
