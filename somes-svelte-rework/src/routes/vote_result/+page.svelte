<script lang="ts">
	import { vote_result_by_id } from '$lib/api';
	import { currentDelegateStore, currentVoteResultStore, hasGoBackStore } from '$lib/stores/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { Delegate, VoteResult } from '$lib/types';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import VoteParliament from '$lib/components/Parliaments/VoteParliament.svelte';
	import InfoTiles from '$lib/components/VoteResults/InfoTiles/InfoTiles.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import VoteDelegateCard from '$lib/components/Delegates/VoteDelegateCard.svelte';
	import type { Bubble } from '$lib/parliament';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { replaceState } from '$app/navigation';
	import { convertDelegatesToAutocompleteOptions, delegateFilterOptions } from '$lib/components/Autocompletion/filtering';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	let dels: Delegate[] | null = null;

	let voteResult = get(currentVoteResultStore);
	let voteResultId: string | null = null;
	let oldVoteResultId: string | null = voteResultId;

	let delegate: Delegate | null = null;
	let bubble: Bubble;

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let inputValue = '';

	onMount(async () => {
		dels = await filteredDelegates();
		if (dels !== null) {
			delegate = dels[Math.floor(Math.random() * dels.length)];
			autocompleteOptions = convertDelegatesToAutocompleteOptions(dels);
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

	// const whichGridContainer =
	// 	emphasis == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
</script>

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
				<div class="title-item rounded-xl bg-primary-300 px-3 py-3">
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
					<div class="text-lg named-vote-info-item rounded-xl bg-primary-300 px-3 py-3">
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

				<div class="z-50 search-item text-token  space-y-5">
					<input
						class="input h-10 px-2"
						type="search"
						name="ac-demo"
						bind:value={inputValue}
						placeholder="Suchen..."
						use:popup={popupSettings}
					/>

					{#if autocompleteOptions}
						<div
							class="card  max-h-64 p-4 overflow-y-auto"
							data-popup="popupAutocomplete"
						>
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

				{#if voteResult.legislative_initiative.gp == 'XXVII'}
					<div class="rounded-xl parliament-item bg-primary-300">
						<VoteParliament {dels} {voteResult} bind:delegate bind:selected={bubble} />
					</div>
					{#if bubble}
						<div class="delegate-item">
							<VoteDelegateCard {bubble} />
						</div>
					{/if}
				{/if}
				<div class="info-item">
					<InfoTiles {voteResult} {dels} />
				</div>

				<div
					class="topics-item flex rounded-xl justify-center items-center bg-primary-300 pt-3 pb-3 px-3"
				>
					<Topics
						topics={voteResult.topics.sort((a, b) => {
							return a.topic.length - b.topic.length;
						})}
					/>
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

	.parliament-item {
		grid-area: p;
		flex-basis: 66.6%;
	}
	.delegate-item {
		grid-area: d;
		flex-basis: 31%;
	}
	.topics-item {
		grid-area: t;
		flex-basis: 38%;
	}

	.emphasis-item {
		grid-area: e;
		flex-basis: 100%;
	}

	.info-item {
		grid-area: i;
		flex-basis: 60%;
	}
	.search-item {
		grid-area: search;
		flex-basis: 100%;
	}


	.named-vote-info-item {
		grid-area: nvi;
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
