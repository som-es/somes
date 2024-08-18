<script lang="ts">
	import { delegates_at, vote_result_by_id } from '$lib/api';
	import { currentDelegateStore, currentVoteResultStore, hasGoBackStore } from '$lib/stores/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { Delegate } from '$lib/types';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import VoteParliament from '$lib/components/Parliaments/VoteParliament.svelte';
	import InfoTiles from '$lib/components/VoteResults/InfoTiles/InfoTiles.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import VoteDelegateCard from '$lib/components/Delegates/VoteDelegateCard.svelte';
	import {
		enrichCirclesWithNamedVoteInfo,
		enrichCirclesWithSpeechInfo,
		type Bubble
	} from '$lib/parliament';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { replaceState } from '$app/navigation';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import SimpleYesNo from '$lib/components/VoteResults/SimpleYesNo/SimpleYesNo.svelte';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';

	let dels: Delegate[] | null = null;

	let voteResult = get(currentVoteResultStore);
	let voteResultId: string | null = null;
	let oldVoteResultId: string | null = voteResultId;

	let delegate: Delegate | null = null;
	let selectedBubble: Bubble;
	let circles2d: Bubble[][];

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let inputValue = '';

	let delegatesAtDate: Delegate[] = [];
	let generalSpeechDelegates: Bubble[] | null = null;
	let generalNamedVoteDelegates: Bubble[] | null = null;

	async function fetchDelegatesAtAndEnrich() {
		if (!voteResult) {
			return;
		}
		delegatesAtDate = (await delegates_at(voteResult.legislative_initiative.created_at)) ?? [];

		if (delegatesAtDate) {
			generalSpeechDelegates = enrichCirclesWithSpeechInfo(voteResult.speeches, delegatesAtDate);
			if (voteResult.named_votes) {
				generalNamedVoteDelegates = enrichCirclesWithNamedVoteInfo(
					voteResult.named_votes.named_votes,
					delegatesAtDate
				);
			} else {
				generalNamedVoteDelegates = [];
			}
		}
	}

	onMount(async () => {
		dels = await filteredDelegates();
		if (dels !== null) {
			delegate = dels[Math.floor(Math.random() * dels.length)];
			autocompleteOptions = convertDelegatesToAutocompleteOptions(dels);
		}
		await fetchDelegatesAtAndEnrich();
		if (delegatesAtDate !== null && voteResult?.legislative_initiative.gp != 'XXVII') {
			delegate = delegatesAtDate[Math.floor(Math.random() * delegatesAtDate.length)];
			autocompleteOptions = convertDelegatesToAutocompleteOptions(delegatesAtDate);
		}

		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			delegate = maybeStoredDelegate;
		}
		const url = new URL(window.location.href);

		voteResultId = url.searchParams.get('id');
		if (voteResultId == null && voteResult !== null) {
			voteResultId = voteResult.legislative_initiative.id;
			oldVoteResultId = voteResultId;
		}

		// if (voteResultId !== null && voteResult?.legislative_initiative.id != voteResultId) {
		//     voteResult = await vote_result_by_id(voteResultId);
		//     if (voteResult !== null) voteResultId = voteResult?.legislative_initiative.id;
		//     currentVoteResultStore.set(voteResult);
		// }
	});

	let currentlyUpdating = false;

	const loadVoteResult = async (voteResultId: string) => {
		if (voteResultId == voteResult?.legislative_initiative.id) {
			return;
		}
		currentlyUpdating = true;
		voteResult = await vote_result_by_id(voteResultId);
		await fetchDelegatesAtAndEnrich();
		currentVoteResultStore.set(voteResult);
		currentlyUpdating = false;
	};

	let updatedQueryParam = false;

	const update = () => {
		if (voteResultId == null) {
			return;
		}

		loadVoteResult(voteResultId);

		// update query params
		const url = new URL(window.location.href);
		url.searchParams.set('id', voteResultId.toString());
		try {
			updatedQueryParam = true;
			replaceState(url, history.state);
			// pushState(url.toString(), { replaceState: true });
		} catch (e) {
			voteResultId = oldVoteResultId;
		}

		oldVoteResultId = voteResultId;
	};

	const goBack = () => {
		history.back();
	};

	$: if (voteResultId) {
		update();
	}

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

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start'
	};

	const emphasis = voteResult?.legislative_initiative.emphasis
		?.split('\n\t')
		.filter((x) => x.length > 0);

	function findBubbleById(id: number): Bubble | undefined {
		return circles2d.flat().find((del) => del.del?.id === id);
	}

	function findDelegateById(dels: Delegate[], id: number): Delegate | undefined {
		return dels.find((del) => del.id === id);
	}

	let iterBubble: Bubble | undefined;
	// console.log(voteResult?.speeches);

	// const whichGridContainer =
	// 	emphasis == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
</script>
<title>
	Abstimmungsergebnis
</title>
<Container>
	{#if voteResult && dels && delegate}
		{#if currentlyUpdating}
			<!-- <CenterPrograssRadial /> -->
		{:else}
			{#if get(hasGoBackStore)}
				<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
			{/if}
			<br />
			<div class=" entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container-with-emphasis">
				<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
					<h1 class="font-bold text-3xl">
						{voteResult.legislative_initiative.voted_by_name ? 'namentliche ' : ''}Abstimmung über
					</h1>
					<span class="text-xl">{voteResult.legislative_initiative.description}</span>
				</div>
				{#if emphasis}
					<div class="emphasis-item">
						<Emphasis {emphasis}></Emphasis>
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

				<div
					class="simple-yes-no-item bg-primary-300 p-3 dark:bg-primary-500 rounded-xl flex flex-wrap justify-between"
				>
					<SimpleYesNo votes={voteResult.votes.slice()} />
				</div>

				<!-- {#if voteResult.legislative_initiative.gp == 'XXVII'} -->
				<div class="z-50 search-item text-token space-y-5">
					<input
						class="!rounded-xl w-full h-12 px-2 input"
						type="search"
						name="ac-demo"
						bind:value={inputValue}
						placeholder="Suchen..."
						use:popup={popupSettings}
					/>

					{#if autocompleteOptions}
						<div class="card max-h-64 p-4 overflow-y-auto" data-popup="popupAutocomplete">
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
						<VoteParliament
							{dels}
							{voteResult}
							delsAtDate={delegatesAtDate}
							bind:delegate
							bind:selected={selectedBubble}
							bind:circles2d
						/>
					</div>
					{#if selectedBubble}
						<div class="max-md:hidden delegate-item rounded-xl bg-primary-300 dark:bg-primary-500">
							<VoteDelegateCard bubble={selectedBubble} />
						</div>
					{/if}
				</div>

				<!-- {/if} -->
				<div class="flex flex-wrap justify-between min-w-full gap-3">
					<div class="info-item">
						<InfoTiles {voteResult} {dels} />
					</div>

					<div
						class="topics-item flex rounded-xl justify-center items-center bg-primary-300 dark:bg-primary-500 p-3 max-h-[169px]"
					>
						<Topics
							topics={voteResult.topics.sort((a, b) => {
								return a.topic.length - b.topic.length;
							})}
						/>
					</div>
				</div>
				{#if generalSpeechDelegates !== null}
					{#if generalSpeechDelegates.length > 0}
						<div class="speeches-item bg-primary-300 dark:bg-primary-500 rounded-xl p-4 gap-3">
							<span class="font-bold text-3xl">Reden</span>
							<div class="flex flex-row flex-wrap mt-3 gap-3">
								{#each generalSpeechDelegates as speechDelegate}
									<div class="w-full max-w-80">
										<VoteDelegateCard bubble={speechDelegate} />
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
										<VoteDelegateCard class="w-80" bubble={namedVoteDelegate} />
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
		padding: 20px;
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
			flex-basis: 66.0%;
		}
	}

	@media (min-width: 768px) {
		.delegate-item {
			grid-area: d;
			flex-basis: 33.0%;
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
