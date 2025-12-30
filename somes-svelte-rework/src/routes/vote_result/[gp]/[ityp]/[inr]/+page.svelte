<script lang="ts">
	import { errorToNull, get_eurovoc_topics, vote_result_by_path } from '$lib/api/api';
	import {
		currentDelegateStore,
		currentVoteResultStore,
		hasGoBackStore,
		useCurrentDelegate
	} from '$lib/stores/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { Delegate, VoteResult } from '$lib/types';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import InfoTiles from '$lib/components/VoteResults/InfoTiles/InfoTiles.svelte';
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
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import SimpleYesNo from '$lib/components/VoteResults/SimpleYesNo/SimpleYesNo.svelte';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { cachedLegisInitFavos } from '$lib/caching/favos';
	import { addLegisInitFavo, removeLegisInitFavo } from '$lib/api/authed';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import FetchDelegateCard from '$lib/components/Delegates/FetchDelegateCard.svelte';
	import VoteResultIdBar from '$lib/components/Bars/VoteResultIdBar.svelte';
	import { page } from '$app/stores';
	import VoteTypeBadge from '$lib/components/VoteResults/VoteTypeBadge.svelte';
	import Documents from '$lib/components/Documents/Documents.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import InfoBadges from '$lib/components/VoteResults/InfoTiles/InfoBadges.svelte';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import linkIcon from '$lib/assets/misc_icons/external-link.svg?raw';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';
	import { partyColors } from '$lib/partyColor';

	$: gp = $page.params.gp;
	$: ityp = $page.params.ityp;
	$: inr = $page.params.inr;

	let delegates: Delegate[] = [];

	let voteResult: VoteResult | null = null;

	let delegate: Delegate | null = null;
	let selectedBubble: Bubble;
	let circles2d: Bubble[][] = [];

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let inputValue = '';

	let delegatesAtDate: Delegate[] = [];
	let generalSpeechDelegates: Bubble[] | null = null;
	let generalNamedVoteDelegates: Bubble[] | null = null;
	let generalAbsencesDelegates: Bubble[] | null = null;

	function enrichDelegates(delegates: Delegate[]) {
		if (!voteResult) {
			return;
		}

		delegatesAtDate = delegates;
		// delegatesAtDate = (errorToNull(await delegates_at(voteResult.legislative_initiative.created_at))) ?? [];
		if (delegatesAtDate) {
			generalSpeechDelegates = genCirclesWithSpeechInfo(voteResult.speeches, delegatesAtDate);
			if (voteResult.named_votes) {
				generalNamedVoteDelegates = genCirclesWithNamedVoteInfo(
					voteResult.named_votes.named_votes,
					delegatesAtDate
				);
			} else {
				generalNamedVoteDelegates = [];
			}
			// TODO set general absences delegates -> mind to update absence delegates
		}
	}

	$: issuedByDels = new Map<string, number[]>();
	$: description = voteResult?.legislative_initiative?.description;

	$: if (voteResult?.issued_by_dels) {
		issuedByDels = new Map<string, number[]>();
		voteResult.issued_by_dels.forEach((del) => {
			const text = del.text ? del.text : 'Abgeordnete';
			if (issuedByDels.has(text)) {
				issuedByDels.get(text)?.push(del.delegate_id);
			} else {
				issuedByDels.set(text, [del.delegate_id]);
			}
		});
	}

	const update = () => {
		loadVoteResult();
	};

	const updateAutocompletion = () => {
		autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates, [], voteResult);
	};

	const selectRandomlyFromDels = () => {
		delegate = delegates[Math.floor(Math.random() * delegates.length)];

		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			const foundDel = delegates.find((del) => del.id == maybeStoredDelegate.id);
			if (foundDel) {
				delegate = foundDel;
			}
		}
	};

	let legisInitFavos: Set<number> | null = null;

	const runVoteResultUpdate = async () => {
		legisInitFavos = await cachedLegisInitFavos();

		voteResult = get(currentVoteResultStore);
		const stored_gp = voteResult?.legislative_initiative?.gp;
		const stored_ityp = voteResult?.legislative_initiative?.ityp;
		const stored_inr = voteResult?.legislative_initiative?.inr ?? 0;
		if (gp != stored_gp || ityp != stored_ityp || inr != stored_inr?.toString()) {
			update();
		}

		if (!delegates) {
			return;
		}

		enrichDelegates(delegates);

		selectRandomlyFromDels();
		updateAutocompletion();
	};

	onMount(runVoteResultUpdate);

	// let tempInr = "0";

	// setInterval(async () => {
	// 	tempInr = ((+tempInr) + 1).toString();
	// 	console.log(tempInr);
	// 	voteResult = errorToNull(await vote_result_by_path(gp, ityp, tempInr));
	// }, 5000)

	let currentlyUpdating = false;

	const loadVoteResult = async () => {
		currentlyUpdating = true;
		voteResult = errorToNull(await vote_result_by_path(gp, ityp, inr));
		// if (delegates)
		// await fetchDelegatesAtAndEnrich();
		currentVoteResultStore.set(voteResult);
		currentlyUpdating = false;
	};

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

	// let emphasis: string[] | undefined = undefined;
	// $: if (voteResult || voteResultId) emphasis = voteResult?.legislative_initiative.emphasis
	// 	?.split('\n\t')
	// 	.filter((x) => x.length > 0);

	$: if (delegates || voteResult) {
		updateAutocompletion();
		selectRandomlyFromDels();
		enrichDelegates(delegates);
	}

	let iterBubble: Bubble | undefined;
	// console.log(voteResult?.speeches);

	// const whichGridContainer =
	// 	emphasis == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
	$: speeches = circles2d.flat(1).filter((circle) => circle.speech !== null);
	$: parliamentUrl = `https://parlament.gv.at/gegenstand/${gp}/${ityp}/${inr}?utm_source=somes.at`;
	$: documents = voteResult?.documents ?? [];
	$: votedByName = voteResult?.legislative_initiative.voted_by_name ?? false;
	$: couldExtractNamedVotes =
		(voteResult?.named_votes?.named_votes?.length ?? 0) > 0 && votedByName;
</script>

<title>
	{#if voteResult}
		{#if voteResult.ai_summary}
			{voteResult.ai_summary.short_title}
		{:else}
			{description}
		{/if}
	{/if}
</title>
<Container>
	{#if voteResult}
		{#if currentlyUpdating}
			<!-- <CenterPrograssRadial /> -->
		{:else}
			<br />
			<div class="entry bg-primary-200 dark:bg-primary-400 md:mt-3 grid-container-with-emphasis">
				<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-6 py-5">
					<!-- Title, Date and Result Icon -->
					<div class="flex justify-between items-start">
						<div class="flex items-center gap-4">
							<!-- Title & Date Stack -->
							<div class="flex flex-col">
								<div class="flex items-start gap-2">
									<span
										class="text-xl lg:text-3xl font-bold leading-tight"
										style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
									>
										{#if voteResult.ai_summary}
											<AiSummaryHintPopup aiSummary={voteResult.ai_summary} />
											{voteResult.ai_summary.short_title}
										{:else}
											{description}
										{/if}
									</span>
								</div>

								{#if voteResult.legislative_initiative.accepted}
									<span class="text-base text-gray-800">
										{voteResult.legislative_initiative.voted_by_name ? 'namentlich ' : ''}
										abgestimmt am {dashDateToDotDate(
											voteResult.legislative_initiative.created_at.toString()
										)}
									</span>
								{/if}
							</div>
						</div>

						<!-- Right Actions, Result Icon and  Star -->
						<div class="flex flex-wrap items-center gap-2 flex-shrink-0">
							<a href={parliamentUrl} target="_blank" class="w-5 text-gray-500">
								{@html linkIcon}
							</a>
							<!-- Result Icon -->
							{#if voteResult.legislative_initiative.accepted}
								<div class="flex-shrink-0">
									{#if voteResult.legislative_initiative.accepted == 'a'}
										<span
											class="stroke-green-600 dark:stroke-green-500 block"
											style="width:40px; height:40px;"
										>
											{@html checkmarkIcon}
										</span>
									{:else}
										<span class="block" style="width:45px; height:45px;">
											{@html crossmarkIcon}
										</span>
									{/if}
								</div>
							{/if}
							{#if legisInitFavos}
								<button
									on:click={async () => {
										/* toggle logic */
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
					{#if voteResult.ai_summary}
						<div class="mt-5 pb-3">
							<h1 class="font-semibold text-lg md:text-xl">Zusammenfassung</h1>
							<span class="text-base lg:text-base text-gray-800">
								<GlossaryText
									text={voteResult.ai_summary.short_summary}
									glossary={voteResult.ai_summary.full_summary.glossary}
								/>
							</span>
						</div>
					{/if}

					<div class="flex flex-wrap justify-between items-center gap-3 w-full pt-1">
						<div>
							<InfoBadges {voteResult} />
						</div>

						<div class="flex-1 flex justify-end">
							{#if voteResult.ai_summary && voteResult.eurovoc_topics.length == 0}
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
				{#if voteResult.ai_summary}
					<div class="emphasis-item">
						<Emphasis
							emphasis={voteResult.ai_summary.full_summary.key_points}
							glossary={voteResult.ai_summary.full_summary.glossary}
						></Emphasis>
					</div>
				{/if}

				<!-- Mini Parlament and Vote Results-->
				{#if voteResult && voteResult.votes}
					<div
						class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-5 pt-3 pb-3 flex"
					>
						<!-- Abstimmung, Fractions, Result - Mobile -->
						<div class="w-full max-lg:block hidden">
							<h3 class="font-semibold text-lg md:text-xl">Abstimmung</h3>
							<div class="ml-1 flex justify-between w-full">
								<div>
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										{#if vote.infavor}
											<div class="flex items-center justify-between">
												<div class="flex items-center gap-2">
													<div
														class="w-3 h-3 rounded-full"
														style="background-color: {partyColors.get(vote.party) ?? '#ccc'};"
													></div>
													<span class="text-base text-gray-800 font-medium"
														>{vote.party} ({vote.fraction})</span
													>
												</div>
												<div class="w-5 h-5">
													<span class="stroke-green-600 dark:stroke-green-500"
														>{@html checkmarkIcon}</span
													>
												</div>
											</div>
										{/if}
									{/each}
								</div>
								<div>
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										{#if !vote.infavor}
											<div class="flex items-center justify-between">
												<div class="flex items-center gap-2 mr-1">
													<div
														class="w-3 h-3 rounded-full"
														style="background-color: {partyColors.get(vote.party) ?? '#ccc'};"
													></div>
													<span class="text-base text-gray-800 font-medium"
														>{vote.party} ({vote.fraction})</span
													>
												</div>
												<div class="w-5 h-5">
													<span class="stroke-red-600 dark:stroke-red-500"
														>{@html crossmarkIcon}</span
													>
												</div>
											</div>
										{/if}
									{/each}
								</div>
							</div>
						</div>

						<!-- Abstimmung, Fractions, Result and Mini Parlament - Desktop-->
						<div class="max-lg:hidden">
							<h3 class="font-semibold text-lg md:text-xl">Abstimmung</h3>
							<div class="ml-1">
								{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
									<div class="flex items-center justify-between">
										<div class="flex items-center gap-2">
											<div
												class="w-3 h-3 rounded-full"
												style="background-color: {partyColors.get(vote.party) ?? '#ccc'};"
											></div>
											<span class="text-sm lg:text-base">{vote.party} ({vote.fraction})</span>
										</div>
										<div class="w-5 h-5">
											{#if vote.infavor}
												<span class="stroke-green-600 dark:stroke-green-500"
													>{@html checkmarkIcon}</span
												>
											{:else}
												<span class="stroke-red-600 dark:stroke-red-500">{@html crossmarkIcon}</span
												>
											{/if}
										</div>
									</div>
								{/each}
							</div>
						</div>

						<div class="max-lg:hidden max-w-md parliament-item m-auto">
							<VoteParliament2 {voteResult} bind:delegates={delegatesAtDate} preview={true} />
						</div>

						<div class="max-lg:hidden w-40"></div>
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

				

					<div class="!z-20 search-item text-token space-y-5">
						<input
							class="!rounded-xl w-full h-12 px-2 input"
							type="search"
							name="ac-demo"
							bind:value={inputValue}
							placeholder="Suchen..."
							use:popup={popupSettings}
						/>

						{#if autocompleteOptions}
							<div class="!z-10 card max-h-64 p-4 overflow-y-auto" data-popup="popupAutocomplete">
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
									date={voteResult.legislative_initiative.created_at}
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

				<div class="flex max-lg:flex-wrap gap-2 w-full">
					<div
						class="flex {voteResult.issued_by_dels.length > 0 ? 'flex-col' : 'flex-row'} gap-2"
						style="flex-basis: {voteResult.issued_by_dels.length > 0 ? '30%' : '100%;'}"
					>
						<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
							<Documents {documents} />
						</div>
						{#if voteResult.referenced_by_others_ids.length > 0}
							<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3 h-full">
								<span class="font-bold text-lg md:text-3xl">Referenziert in</span>
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
							<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3 h-full">
								<span class="font-bold text-lg md:text-3xl">
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
					{#if issuedByDels.size > 0}
						<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
							{#each issuedByDels as [text, delegate_ids]}
								<span class="font-bold text-xl md:text-3xl">{text}</span>
								<!-- <span class="font-bold text-xl"></span> -->
								<div class="flex flex-row flex-wrap mt-3 gap-3">
									{#each delegate_ids as delegate_id}
										<FetchDelegateCard
											delegateId={delegate_id}
											showAI={false}
											showQA={false}
											onlyTop={true}
											showMoreDetailsBtn={true}
											showImg={true}
											showAge={false}
										/>
									{/each}
								</div>
							{/each}
						</div>
					{/if}
				</div>
				{#if circles2d}
					{#if circles2d.length > 0 && speeches.length > 0}
						<div class="speeches-item bg-primary-300 dark:bg-primary-500 rounded-xl p-4 gap-3">
							<span class="font-bold text-xl md:text-3xl">Reden</span>
							<div class="flex flex-row flex-wrap mt-3 gap-3">
								{#each speeches as speechDelegate}
									<div class="w-full max-w-80">
										<VoteDelegateCard
											bubble={speechDelegate}
											gp={voteResult.legislative_initiative.gp}
											date={voteResult.legislative_initiative.created_at}
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
						<div class="speeches-item bg-primary-300 dark:bg-primary-500 rounded-xl p-4 gap-3">
							<span class="font-bold text-3xl">namentliche Abstimmungsergebnisse</span>
							<div class="flex flex-row flex-wrap mt-3 gap-3">
								{#each generalNamedVoteDelegates as namedVoteDelegate}
									<div>
										<VoteDelegateCard
											class="w-80"
											bubble={namedVoteDelegate}
											gp={voteResult.legislative_initiative.gp}
											date={voteResult.legislative_initiative.created_at}
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
