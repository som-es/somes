<script lang="ts">
	import { delegates_at, errorToNull, vote_result_by_id } from '$lib/api/api';
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
	import { filteredDelegates } from '$lib/caching/delegates';
	import VoteDelegateCard from '$lib/components/Delegates/VoteDelegateCard.svelte';
	import {
		genCirclesWithNamedVoteInfo,
		genCirclesWithSpeechInfo,
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
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { cachedLegisInitFavos } from '$lib/caching/favos';
	import { addLegisInitFavo, removeLegiInitFavo } from '$lib/api/authed';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import FetchDelegateCard from '$lib/components/Delegates/FetchDelegateCard.svelte';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';
	import NamedVoteBar from '$lib/components/Delegates/NamedVote/NamedVoteBar.svelte';
	import VoteResultIdBar from '$lib/components/Bars/VoteResultIdBar.svelte';

	let delegates: Delegate[] = [];

	let voteResult: VoteResult | null = null;
	let voteResultId: string | null = null;
	let oldVoteResultId: string | null = voteResultId;

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

	let updatedQueryParam = false;

	$: rawEmphasis = voteResult?.legislative_initiative.emphasis;

	const update = (voteResultId: string | null) => {
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

	const updateAutocompletion = () => {
		autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates, [], voteResult);
	};

	const selectRandomlyFromDels = () => {
		delegate = delegates[Math.floor(Math.random() * delegates.length)];
	};

	let legisInitFavos: Set<number> | null = null;

	const runVoteResultUpdate = async () => {
		legisInitFavos = await cachedLegisInitFavos();

		const url = new URL(window.location.href);

		voteResultId = url.searchParams.get('id');
		if (oldVoteResultId != voteResultId) {
			update(voteResultId);
		}

		if (!voteResultId) {
			voteResult = get(currentVoteResultStore);

			const url = new URL(window.location.href);
			url.searchParams.set('id', voteResult?.legislative_initiative.id.toString() ?? '');
			replaceState(url, history.state);
		}

		if (!delegates) {
			return;
		}

		enrichDelegates(delegates);

		selectRandomlyFromDels();
		updateAutocompletion();

		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			const foundDel = delegates.find((del) => del.id == maybeStoredDelegate.id);
			if (foundDel) {
				delegate = foundDel;
			}
		}

		if (voteResultId == null && voteResult !== null) {
			voteResultId = voteResult.legislative_initiative.id.toString();
			oldVoteResultId = voteResultId;
		}

		// if (voteResultId !== null && voteResult?.legislative_initiative.id != voteResultId) {
		//     voteResult = await vote_result_by_id(voteResultId);
		//     if (voteResult !== null) voteResultId = voteResult?.legislative_initiative.id;
		//     currentVoteResultStore.set(voteResult);
		// }
	};

	onMount(runVoteResultUpdate);

	let currentlyUpdating = false;

	const loadVoteResult = async (voteResultId: string) => {
		if (voteResultId == voteResult?.legislative_initiative.id.toString()) {
			return;
		}
		currentlyUpdating = true;
		voteResult = errorToNull(await vote_result_by_id(voteResultId));
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

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start'
	};

	// let emphasis: string[] | undefined = undefined;
	// $: if (voteResult || voteResultId) emphasis = voteResult?.legislative_initiative.emphasis
	// 	?.split('\n\t')
	// 	.filter((x) => x.length > 0);

	$: if (delegates) {
		updateAutocompletion();
		selectRandomlyFromDels();
		enrichDelegates(delegates);
	}

	let iterBubble: Bubble | undefined;
	// console.log(voteResult?.speeches);

	// const whichGridContainer =
	// 	emphasis == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
	$: speeches = circles2d.flat(1).filter((circle) => circle.speech !== null);
</script>

<title>
	{#if voteResult?.legislative_initiative.accepted}
		Abstimmungsergebnis
	{:else}
		Gegenstandsübersicht
	{/if}
</title>
<Container>
	{#if voteResult}
		{#if currentlyUpdating}
			<!-- <CenterPrograssRadial /> -->
		{:else}
			{#if get(hasGoBackStore)}
				<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
			{/if}
			<br />
			<div class=" entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container-with-emphasis">
				<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
					<div class="flex justify-between items-center">
						<div>
							<h1 class="font-bold text-3xl">
								{#if voteResult?.legislative_initiative.accepted}
									{voteResult.legislative_initiative.voted_by_name ? 'namentliche ' : ''}Abstimmung
									über
								{:else}
									Gegenstand
								{/if}
							</h1>
							<span class="text-xl">{voteResult.legislative_initiative.description}</span>

							{#if voteResult.legislative_initiative.is_law}
								<div class="badge bg-tertiary-400 ml-2 text-black">Gesetz</div>
							{/if}
						</div>
						<div>
							{#if legisInitFavos}
								{#if legisInitFavos.has(+voteResult.legislative_initiative.id)}
									<button
										on:click={async () => {
											if (!voteResult) return;
											if (
												(await removeLegiInitFavo({
													vote_result_id: +voteResult.legislative_initiative.id
												})) == null
											) {
												legisInitFavos?.delete(+voteResult.legislative_initiative.id);
												legisInitFavos = legisInitFavos;
											}
										}}
										class="w-14 p-2"
									>
										{@html starFilled}
									</button>
								{:else}
									<button
										on:click={async () => {
											if (!voteResult) return;
											if (
												(await addLegisInitFavo({
													vote_result_id: +voteResult.legislative_initiative.id
												})) == null
											) {
												legisInitFavos?.add(+voteResult.legislative_initiative.id);
												legisInitFavos = legisInitFavos;
											}
										}}
										class="w-14 p-2"
									>
										{@html star}
									</button>
								{/if}
							{/if}
						</div>
					</div>
				</div>
				{#if rawEmphasis}
					<div class="emphasis-item">
						<Emphasis
							{rawEmphasis}
							isAiGenerated={voteResult.legislative_initiative.is_emphasis_ai_generated ?? false}
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
						<SimpleYesNo votes={voteResult.votes.slice()} />
					</div>

					<!-- {#if voteResult.legislative_initiative.gp == 'XXVII'} -->

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
				{/if}

				<!-- {/if} -->
				<div class="flex flex-wrap justify-between min-w-full gap-3">
					{#if delegates}
						<div class="md:hidden info-item">
							<InfoTiles {voteResult} dels={delegates} isCenter />
						</div>
						<div class="max-md:hidden info-item">
							<InfoTiles {voteResult} dels={delegates} />
						</div>
					{/if}

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

				<div class="flex max-md:flex-wrap gap-2 w-full">
					<div class="flex flex-col gap-2" style="flex-basis: 30%;">
					<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
						<span class="font-bold text-xl">Dokumente (PDFs)</span>
						<div class="gap-3 flex flex-wrap">
							{#each voteResult.documents.sort((a, b) => (b.title ?? "").length - (a.title ?? "").length) as document}
								{#if document.document_type.includes('PDF')}
									<SButton
										class="bg-secondary-500 text-black"
										on:click={() =>
											window.open(`https://www.parlament.gv.at${document.document_url}`, '_blank')}
										>{document.title}</SButton
									>
								{/if}
							{/each}
						</div>
					</div>
						{#if voteResult.referenced_by_others_ids.length > 0}
							<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3 h-full">
								<span class="font-bold text-3xl ">Referenziert in</span>
								{#each voteResult.referenced_by_others_ids as refered_by}
									<VoteResultIdBar  legis_init_id={refered_by} />
								{/each}
							</div>
						{/if}
						{#if voteResult.references.length > 0}
							<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3 h-full">
								<span class="font-bold text-3xl">Bezug zu</span>
								{#each voteResult.references as refered_by}
									<VoteResultIdBar requiringVotes legis_init_id={refered_by} />
								{/each}
							</div>
						{/if}
					</div>

					{#if voteResult.issued_by_dels.length > 0}
						<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
							<span class="font-bold text-3xl">Eingebracht von</span>
							<span class="font-bold text-xl"></span>
							<div class="flex flex-row flex-wrap mt-3 gap-3">
								{#each voteResult.issued_by_dels as delegateId}
									<FetchDelegateCard
										{delegateId}
										showAI={false}
										showQA={false}
										onlyTop={true}
										showMoreDetailsBtn={true}
										showImg={true}
										showAge={false}
									/>
								{/each}
							</div>
						</div>
					{/if}

				</div>
				{#if circles2d}
					{#if circles2d.length > 0}
						<div class="speeches-item bg-primary-300 dark:bg-primary-500 rounded-xl p-4 gap-3">
							{#if speeches.length > 0}
								<span class="font-bold text-3xl">Reden</span>
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
							{/if}
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
