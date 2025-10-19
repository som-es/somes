<script lang="ts">
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import type {
		Delegate,
		DelegateQA,
		GeneralDelegateInfo,
		GeneralGovOfficialInfo,
		GovProposal,
		InterestShare,
		LegisPeriod,
		PoliticalPosition,
		Speech,
		SpeechesWithMaxPage
	} from '$lib/types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import {
		delegate_by_id,
		errorToNull,
		general_delegate_info,
		general_gov_official_info,
		gov_proposals_by_official,
		speeches_by_delegate_per_page
	} from '$lib/api/api';
	import InterestTiles from '$lib/components/Delegates/InterestTiles.svelte';
	import { get } from 'svelte/store';
	import {
		currentDelegateFilterStore,
		currentDelegateStore,
		hasGoBackStore
	} from '$lib/stores/stores';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';
	import LegisButtons from '$lib/components/Filtering/LegisButtons.svelte';
	import AllBadges from '$lib/components/VoteResults/SimpleYesNo/AllBadges.svelte';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { dashDateToDotDate } from '$lib/date';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import GovProposalPreview from '$lib/components/Proposals/GovProposalPreviewAtDelegate.svelte';
	import SpeechesPreview from '$lib/components/Delegates/Speeches/SpeechesPreview.svelte';
	import SquarePoliticalSpectrum from '$lib/components/Delegates/Spectrum/SquarePoliticalSpectrum.svelte';
	import AbsencesPreview from '$lib/components/Delegates/Absences/AbsencesPreview.svelte';
	import NamedVotePreview from '$lib/components/Delegates/NamedVote/NamedVotePreview.svelte';
	import StanceDiagram from '$lib/components/Delegates/Spectrum/Stance/StanceDiagram.svelte';
	import { topicColors } from '$lib/interestColors';
	import ReactiveGenericBarChart from '$lib/components/GeneralCharts/ReactiveGenericBarChart.svelte';
	import ReactiveRadarChart from '$lib/components/GeneralCharts/ReactiveRadarChart.svelte';
	import TopicsChart from '$lib/components/Delegates/Interests/TopicsChart.svelte';
	import StanceTypeSwitcher from '$lib/components/Delegates/Spectrum/Stance/StanceTypeSwitcher.svelte';
	import PoliticalStanceTitleBar from '$lib/components/Delegates/Spectrum/PoliticalStanceTitleBar.svelte';
	import DecreePreview from '$lib/components/Delegates/Decrees/DecreePreview.svelte';
	import { pushState } from '$app/navigation';

	let delegates: Delegate[];
	let delegate: Delegate | null;

	let selectedPeriod = 'XXVIII';
	let prevSelectedPeriod = selectedPeriod;
	let periods: LegisPeriod[] = [];

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start'
	};

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let speechesPage0: SpeechesWithMaxPage | null = null;
	let generalDelegateInfo: GeneralDelegateInfo | null = null;
	let generalGovOfficialInfo: GeneralGovOfficialInfo | null = null;
	let maxDayOffset = 365 * 5;
	let dayOffset = maxDayOffset;

	let renderStartDate: Date | null;
	let renderEndDate: Date | null;

	let finishedMounting = false;
	let supplyDate: Date | null = null;

	let inputValue = '';
	let prevSelectedDelegateId = 0;

	let maybeCurrentDelegateFilter = get(currentDelegateFilterStore) ?? {
		day_offset: maxDayOffset,
		search_value: '',
		legis_period: selectedPeriod
	};
	inputValue = maybeCurrentDelegateFilter.search_value ?? '';
	dayOffset = maybeCurrentDelegateFilter.day_offset ?? maxDayOffset;
	if (maybeCurrentDelegateFilter.legis_period) {
		selectedPeriod = maybeCurrentDelegateFilter.legis_period;
		prevSelectedPeriod = maybeCurrentDelegateFilter.legis_period;
	}

	function delegateFilter(): AutocompleteOption<string>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return delegateFilterOptions(_options, _inputValue);
	}

	$: if (inputValue) {
		maybeCurrentDelegateFilter.search_value = inputValue;
		currentDelegateFilterStore.set(maybeCurrentDelegateFilter);
	}

	function onDelegateSelection(event: CustomEvent<AutocompleteOption<string>>): void {
		// @ts-ignore
		delegate = event.detail.meta;
		inputValue = event.detail.label;
	}

	onMount(async () => {
		const url = new URL(window.location.href);
		selectedPeriod = url.searchParams.get('gp') || selectedPeriod;

		const cachedPeriods = await cachedAllLegisPeriods();
		if (cachedPeriods) periods = cachedPeriods.reverse();

		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
		// ).toISOString().split('T')[0] as unknown as Date
		const endDate = periods[firstIdx + 1]?.start_date;
		const newDate = new Date(endDate ? endDate : new Date());
		newDate.setDate(newDate.getDate() - 1);

		const paramDate = url.searchParams.get('date');
		if (paramDate) {
			const startDate = new Date(periods[firstIdx]?.start_date);
			const diffTime = Math.abs(new Date(paramDate).getTime() - startDate.getTime());
			dayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
			// this prevents that dayOffset is overwritten with max
			prevSelectedPeriod = selectedPeriod;
		}
		supplyDate = paramDate
			? (paramDate as unknown as Date)
			: (newDate.toISOString().split('T')[0] as unknown as Date);
		// console.log(supplyDate);

		const paramDelegateId = url.searchParams.get('delegate');
		if (paramDelegateId) {
			// setting here currentDelegateStore instead of `delegate` var directly
			// this is important for a single reason: delegates without seat by default (if the backend seat history is too short)
			// wouldn't be selectable by the DataParliament component -> however, there is a reactive update happening,
			// when `delegates` is updated (therefore the client-side seat position generation was complete) and `delegate` is null
			currentDelegateStore.set(errorToNull(await delegate_by_id(+paramDelegateId)));
		}

		finishedMounting = true;
	});

	const updateDelsToDisplay = async () => {
		if (!periods || periods.length == 0) {
			return;
		}
		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
		// delegate = null;
		// const endDate = new Date(periods[firstIdx + 1].start_date);
		renderStartDate = periods[firstIdx].start_date;
		renderEndDate = periods[firstIdx + 1]?.start_date;
		const startDate = new Date(renderStartDate);
		const endDate = new Date(renderEndDate ? renderEndDate : new Date());

		const diffTime = Math.abs(endDate.getTime() - startDate.getTime());
		maxDayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
		if (prevSelectedPeriod !== selectedPeriod) {
			dayOffset = maxDayOffset;
		}
		maybeCurrentDelegateFilter.day_offset = dayOffset;
		currentDelegateFilterStore.set(maybeCurrentDelegateFilter);
		startDate.setDate(startDate.getDate() + dayOffset - 2);

		supplyDate = startDate.toISOString().split('T')[0] as unknown as Date;

		const url = new URL(window.location.href);
		url.searchParams.set('date', supplyDate as unknown as string);
		url.searchParams.set('gp', selectedPeriod);
		pushState(url.toString(), { replaceState: true });

		// console.log(`supply ${supplyDate}`);
		// const fetchedDelsAtDate = await delegates_at(
		// );
		// console.log(fetchedDelsAtDate);

		// if (fetchedDelsAtDate) {
		// delsAtDate = fetchedDelsAtDate;
		// autocompleteOptions = convertDelegatesToAutocompleteOptions(delsAtDate);
		// }
	};

	const onLettingGoOfDaySlider = () => {
		renderEndDate = null;
		renderStartDate = null;
		updateDelsToDisplay();
		if (finishedMounting) prevSelectedPeriod = selectedPeriod;
	};

	const updateStoredPeriod = () => {
		maybeCurrentDelegateFilter.legis_period = selectedPeriod;
		currentDelegateFilterStore.set(maybeCurrentDelegateFilter);
	};

	$: if (selectedPeriod) {
		renderEndDate = null;
		renderStartDate = null;
		// if (window !== null) {
		// 	const url = new URL(window.location.href);
		// 	url.searchParams.set('gp', selectedPeriod);
		// 	replaceState(url, history.state);
		// }

		// delsAtDate = [];
		updateStoredPeriod();
		updateDelsToDisplay();
		if (finishedMounting) prevSelectedPeriod = selectedPeriod;
	}

	$: if (delegates && delegate == null) {
		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			delegate = maybeStoredDelegate;
			const foundDel = delegates.find((del) => del.id === maybeStoredDelegate.id);
			if (foundDel) {
				delegate = foundDel;
			} else {
				delegate = delegates[Math.floor(Math.random() * delegates.length)];
			}
		} else {
			delegate = delegates[Math.floor(Math.random() * delegates.length)];
		}
	}
	$: if (delegates) autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates);

	$: if (delegate && prevSelectedDelegateId != delegate.id) {
		// interests = null;
		if (finishedMounting) currentDelegateStore.set(delegate);

		const url = new URL(window.location.href);
		url.searchParams.set('delegate', delegate.id.toString());
		pushState(url.toString(), { replaceState: true });

		generalDelegateInfo = null;
		general_delegate_info(delegate.id).then((res) => {
			generalDelegateInfo = errorToNull(res);
			if (generalDelegateInfo) {
				generalDelegateInfo.interests.sort((a, b) => b.self_share - a.self_share);
				generalDelegateInfo.detailed_interests.sort((a, b) => b.self_share - a.self_share);
				// console.log(`absences length: ${generalDelegateInfo.absences.length}, ${generalDelegateInfo.named_votes.length}`);
			}
		});

		generalGovOfficialInfo = null;
		general_gov_official_info(delegate.id).then((res) => {
			generalGovOfficialInfo = errorToNull(res);
		});

		// gov_proposals_by_official(delegate.id).then((res) => {
		// 	govProposals = errorToNull(res);
		// });

		speechesPage0 = null;
		speeches_by_delegate_per_page(delegate.id, 0).then((res) => {
			speechesPage0 = errorToNull(res);
		});

		prevSelectedDelegateId = delegate.id;
	}
</script>

<!-- <div class="mx-auto px-10"> -->
<Container>
	{#if get(hasGoBackStore)}
		<SButton class="bg-primary-500 my-3 hidden lg:block" on:click={() => history.back()}
			>Zurück</SButton
		>
	{/if}
	<div class="entry bg-primary-200 dark:bg-primary-400 gap-3 flex flex-wrap">
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<h1 class="font-bold max-lg:text-xl lg:text-4xl">Abgeordnete des Nationalrats</h1>
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<LegisButtons bind:periods bind:selectedPeriod showAllButton={false}></LegisButtons>
		</div>
		<div
			class="flex flex-row flex-wrap title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3"
		>
			<div class="flex justify-between min-w-full">
				<div>
					Anfang ({renderStartDate == null ? '' : dashDateToDotDate(renderStartDate.toString())})
				</div>
				<div>
					Ende ({renderEndDate == null
						? dashDateToDotDate(new Date().toISOString().split('T')[0])
						: dashDateToDotDate(renderEndDate.toString())})
				</div>
			</div>
			<input
				bind:value={dayOffset}
				on:change={onLettingGoOfDaySlider}
				type="range"
				min="2"
				max={maxDayOffset + 2}
				step={1}
				list="steplist"
			/>
			<datalist id="steplist">
				<option>0</option>
				<option>25</option>
				<option>50</option>
				<option>75</option>
				<option>365</option>
			</datalist>
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			{#if periods.length > 0 && delegates}
				<AllBadges delsAtDate={structuredClone(delegates)} />
			{/if}
		</div>
		<!-- {#if delegates} -->
		<div class="text-token w-full space-y-2">
			<input
				class="input w-full h-12 px-2"
				type="search"
				name="ac-demo"
				bind:value={inputValue}
				placeholder="Suchen..."
				use:popup={popupSettings}
			/>

			{#if autocompleteOptions}
				<div
					class="z-10 card w-full max-w-sm max-h-64 p-4 overflow-y-auto"
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
		<div class="flex flex-wrap min-w-full justify-between">
			<div class="rounded-xl w-full parliament-item bg-primary-300 dark:bg-primary-200">
				<div class="px-5">
					{#if supplyDate}
						<VoteParliament2
							againstOpacity={1}
							voteResult={null}
							bind:delegate
							bind:delegates
							gp={selectedPeriod}
							{supplyDate}
							orderingFactor={-1}
							showGovs={true}
							show3D
						/>
					{/if}
				</div>
			</div>
			<div class="rounded-xl delegate-item bg-primary-300 dark:bg-primary-500">
				{#if delegate}
					<DelegateCard {delegate} questions={generalDelegateInfo?.delegate_qa ?? []} showQA />
				{/if}
			</div>
		</div>
		{#if generalGovOfficialInfo?.gov_proposals && generalGovOfficialInfo.gov_proposals.length > 0 && delegate}
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
				<GovProposalPreview govProposals={generalGovOfficialInfo.gov_proposals} {delegate} />
			</div>
		{:else if generalGovOfficialInfo?.gov_proposals == null && delegate && delegate.council == 'gov'}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if generalGovOfficialInfo?.decrees && generalGovOfficialInfo.decrees.length > 0 && delegate}
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
				<DecreePreview decrees={generalGovOfficialInfo.decrees} {delegate} />
			</div>
		{:else if (generalGovOfficialInfo?.decrees == null && delegate && delegate.council == 'gov') || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if delegate && generalDelegateInfo?.political_position}
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
				<PoliticalStanceTitleBar
					{delegate}
					stanceTopicInfluences={generalDelegateInfo.stance_topic_influences}
				/>
			</div>
		{/if}
		<div class="flex max-lg:flex-wrap gap-2">
			{#if delegate && generalDelegateInfo?.political_position}
				<SquarePoliticalSpectrum
					{delegate}
					politicalPosition={generalDelegateInfo.political_position}
				/>
			{:else if !generalDelegateInfo}
				<ExpandablePlaceholder class={'my-3'} />
			{/if}

			{#if delegate && generalDelegateInfo?.left_right_stances.length && generalDelegateInfo.left_right_stances.length > 0}
				<StanceTypeSwitcher delegateInfo={generalDelegateInfo} />
			{:else if !generalDelegateInfo}
				<ExpandablePlaceholder class={'my-3'} />
			{/if}
		</div>

		{#if generalDelegateInfo?.interests && generalDelegateInfo?.detailed_interests}
			{#if generalDelegateInfo?.interests?.length > 0}
				<span class="max-sm:hidden w-full">
					<TopicsChart
						detailedInterests={generalDelegateInfo.detailed_interests}
						interests={generalDelegateInfo.interests}
					/>
				</span>
				<span class="sm:hidden w-full">
					<TopicsChart
						detailedInterests={generalDelegateInfo.detailed_interests}
						interests={generalDelegateInfo.interests.slice(0, 8)}
					/>
				</span>
				<!-- <div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">

					<h1 class="font-bold text-2xl mb-2">Top 4 Interessen</h1>
					<InterestTiles bgColor={"bg-primary-300 dark:bg-primary-500"} squareColor={"dark:bg-primary-300 bg-primary-400"} interests={generalDelegateInfo.interests.slice(0, 4)} />
				</div> -->
			{/if}
		{:else}
			<ExpandablePlaceholder class={'my-3'} />
		{/if}

		{#if speechesPage0 && delegate && speechesPage0.speeches.length > 0}
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
				<SpeechesPreview delegateId={delegate.id} {speechesPage0} />
			</div>
		{:else if speechesPage0 == null && delegate && delegate.council == 'gov'}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if generalDelegateInfo?.absences && delegate && generalDelegateInfo?.absences.length > 0}
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
				<AbsencesPreview delegateId={delegate.id} absences={generalDelegateInfo.absences} />
			</div>
		{:else if generalDelegateInfo?.absences == null || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if generalDelegateInfo?.named_votes && delegate && generalDelegateInfo?.named_votes.length > 0}
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">
				<NamedVotePreview delegateId={delegate.id} namedVotes={generalDelegateInfo.named_votes} />
			</div>
		{:else if generalDelegateInfo?.absences == null || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		<!-- <div class="flex gap-2 w-full">
		<ExpandablePlaceholder class={'my-3 w-full min-w-full'} />
		</div> -->

		<!-- {#if generalDelegateInfo}
			<ReactiveRadarChart title="hi" chartData={[
				{ label: "namentliche Abstimmungen", data: generalDelegateInfo.named_votes.length, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
				{ label: "Reden", data: speechesPage0?.entry_count ?? 0, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
			]} />
		{:else if generalDelegateInfo == null || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if} -->

		<!-- <div class="activity-item bg-primary-300">
                    Activity
                </div> -->
		<!-- {/if} -->
	</div>
</Container>

<!-- </div> -->

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.grid-container {
		box-sizing: border-box;
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 1fr;
		grid-template-rows: 2fr auto;
		grid-template-areas:
			'p d'
			'i .';
		/* "i i i a"; */
		padding: 10px;
	}

	.grid-container > div {
		padding: 20px 0;
	}

	/* :global(.parliament-item) {
		grid-area: p;
	} */

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

	.title-item {
		flex-basis: 100%;
	}

	:global(.interests-item) {
		grid-area: i;
		/* overflow: hidden; */
		/* min-width: 0; */
	}

	.activity-item {
		grid-area: activity;
	}

	.grid-tile-content {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
	}
</style>
