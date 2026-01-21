<script lang="ts">
	import { errorToNull, get_eurovoc_topics, vote_result_by_path } from '$lib/api/api';
	import {
		currentDelegateStore,
		currentVoteResultStore,
		hasGoBackStore,
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
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import AiSummaryHintPopup from '$lib/components/AiHint/AiSummaryHintPopup.svelte';
	import { page } from '$app/state';

	let gp = $derived(page.params.gp);
	let ityp = $derived(page.params.ityp);
	let inr = $derived(page.params.inr);

	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let voteResult: VoteResult | null = $derived(errorToNull(data.voteResult));
	let delegates: Delegate[] = $derived(data.delegates ?? []);

	let delegate: Delegate | null = $state(null);
	let selectedBubble: Bubble | undefined = $state();

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let inputValue = '';

	let generalSpeechDelegates = $derived.by(() => {
		const res = voteResult ? genCirclesWithSpeechInfo(voteResult.speeches, delegates) : []
		return res
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

	let legisInitFavos: Set<number> | null = $state(null);

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

	let parliamentUrl = $derived(`https://parlament.gv.at/gegenstand/${gp}/${ityp}/${inr}?utm_source=somes.at`);
	let documents = $derived(voteResult?.documents ?? []);
	let votedByName = $derived(voteResult?.legislative_initiative?.voted_by_name ?? false);
	let couldExtractNamedVotes =
		$derived((voteResult?.named_votes?.named_votes?.length ?? 0) > 0 && votedByName);
</script>

<svelte:head>
    <title>Abstimmungsergebnis</title>
    <meta name="description" content="Spezifisches Abstimmungsergebnis" />
</svelte:head>

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
			{#if hasGoBackStore.value}
				<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
			{/if}
			<br />
			<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container-with-emphasis">
				<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
					<div class="flex justify-between items-start">
						<div class="flex items-center gap-4">
							<!-- Title & Date Stack -->
							<div class="flex flex-col">
								<span class="text-3xl font-bold leading-tight">
									{#if voteResult.ai_summary}
										<AiSummaryHintPopup
											aiSummary={voteResult.ai_summary}
										/>
										{voteResult.ai_summary.short_title}
									{:else}
										{description}
									{/if}	
								</span>

								{#if voteResult.legislative_initiative.accepted && voteResult.legislative_initiative.vote_date}
									<span class="text-sm opacity-90 -mt-1">
										{voteResult.legislative_initiative.voted_by_name ? 'namentlich ' : ''}
										abgestimmt am {dashDateToDotDate(voteResult.legislative_initiative.vote_date.toString())}
									</span>
								{/if}
							</div>

							<!-- Result Icon -->
							{#if voteResult.legislative_initiative.accepted}
								<div class="shrink-0">
									{#if voteResult.legislative_initiative.accepted == 'a'}
										<span class="stroke-green-600 dark:stroke-green-500 block" style="width:60px; height:60px;">
											{@html checkmarkIcon}
										</span>
									{:else}
										<span class="block" style="width:60px; height:60px;">
											{@html crossmarkIcon}
										</span>
									{/if}
								</div>
							{/if}
						</div>

						<!-- Right Actions (Fixed Width) -->
						<div class="flex flex-wrap items-center ml-2 gap-2 shrink-0">
							<a href={parliamentUrl} target="_blank">
								<img class="w-12" alt="favicon" src="https://www.parlament.gv.at/static/img/favicon/favicon.svg" />
							</a>
							{#if legisInitFavos}
								<button onclick={async () => {/* toggle logic */}} class="w-14 p-2">
									{@html legisInitFavos.has(+voteResult.legislative_initiative.id) ? starFilled : star}
								</button>
							{/if}
						</div>
					</div>

					<div class="flex flex-wrap justify-between items-center gap-3 w-full border-t border-black/5 dark:border-white/5 pt-1 ">
						<div class="shrink-0">
							<InfoBadges {voteResult} />
						</div>
						
						<div class="flex-1 flex justify-end">
							{#if voteResult.ai_summary && voteResult.eurovoc_topics.length == 0}
								<Topics topics={voteResult.ai_summary.full_summary.topics.sort((a, b) => {
										return a.length - b.length;
									}).map(topic => {return {topic}})} />
							{:else}
								<Topics topics={voteResult.eurovoc_topics.sort((a, b) => {
									return a.topic.length - b.topic.length;
								})} />
							{/if}
						</div>
					</div>
				</div>
				{#if voteResult.ai_summary}
					<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 pt-3 pb-3">
						
						<h1 class="font-bold text-lg md:text-xl">Zusammenfassung</h1>
						<span class="text-sm lg:text-base">
							<GlossaryText text={voteResult.ai_summary.short_summary} glossary={voteResult.ai_summary.full_summary.glossary} />
						</span>
					</div>
					<div class="emphasis-item">
						<Emphasis
							emphasis={voteResult.ai_summary.full_summary.key_points} glossary={voteResult.ai_summary.full_summary.glossary}
						></Emphasis>
					</div>
				{/if}
				{#if voteResult.named_votes}
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
						
					</div>

					<div class="flex flex-wrap min-w-full justify-between">
						<div class="rounded-xl w-full parliament-item flex- bg-primary-200 dark:bg-primary-200">
							<VoteParliament2
								{voteResult}
								bind:delegate
								{delegates}
								allSeats={data.cachedSeats}
								bind:selected={selectedBubble}
								noSeats={!data.hasSeatInfo}
								useOffset={data.hasSeatInfo}
								showGovs
								show3D
								overrideDelegates
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
				{/if}

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
				{#if generalSpeechDelegates != null}
					{#if generalSpeechDelegates.length > 0}
						<div class="speeches-item bg-primary-300 dark:bg-primary-500 rounded-xl p-4 gap-3">
							<span class="font-bold text-xl md:text-3xl">Reden</span>
							<div class="flex flex-row flex-wrap mt-3 gap-3">
								{#each generalSpeechDelegates as speechDelegate}
									<div class="w-full max-w-80">
										<VoteDelegateCard
											bubble={speechDelegate}
											gp={voteResult.legislative_initiative.gp}
											date={voteResult.legislative_initiative.vote_date ?? voteResult.legislative_initiative.nr_plenary_activity_date}
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
											date={voteResult.legislative_initiative.vote_date ?? voteResult.legislative_initiative.nr_plenary_activity_date}
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

	.emphasis-item {
		grid-area: e;
		flex-basis: 100%;
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

</style>
