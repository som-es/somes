<script lang="ts">
	import type { GovPropFilter, GovProposalsWithMaxPage } from '$lib/types';
	import { onMount, untrack } from 'svelte';
	import Pagination from '../Pagination.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentGovProposalFilterStore } from '$lib/stores/stores';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';
	import { type GenericFilterGroup } from '../Filtering/types';
	import SearchBar from '../Filtering/SearchBar.svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import TopicsFilter from '../Filtering/MultiValuesFilter.svelte';
	import GenericFilters from '../Filtering/GenericFilters.svelte';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { errorToNull, get_eurovoc_topics } from '$lib/api/api';
	import MultiValuesFilter from '../Filtering/MultiValuesFilter.svelte';

	interface Props {
		govProposals: GovProposalsWithMaxPage;
		selectedGp: string | null;
		departmentsPerGp: Record<string, string[]>;
	}

	let {
		govProposals,
		selectedGp,
		departmentsPerGp
	}: Props = $props();

	let genericFilters: [
		GenericFilterGroup<boolean>
	] = $state([
		{
			title: 'Abstimmungsstatus',
			activeValue: undefined,
			hidden: false,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'mit Abstimmung', value: true },
				{ title: 'ohne Abstimmung', value: false }
			]
		},
		
	]);

	let legisPeriodFilter = $state({
		title: 'Legislaturperiode',
		activeValue: 'XXVIII',
		hidden: false,
		options: [{ title: 'Alle', value: 'all' }]
	});

	let searchValue = $state('');

	let updatedAt = $derived(
		govProposals.updated_at
			? new Intl.DateTimeFormat('de-AT', {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric'
				}).format(new Date(govProposals.updated_at))
			: 'Unbekannt'
	);

	let selectedTopics: SvelteSet<string> = $state(new SvelteSet());
	let selectedDepartments: SvelteSet<string> = $state(new SvelteSet());

	let departments = $derived.by(() => {
		if (selectedGp) {
			return departmentsPerGp[selectedGp]
		} else {
			const departments: string[] = [];
			const departmentSet = new Set();
			const keys = Object.keys(departmentsPerGp).sort().reverse();
			keys.forEach((key) => {
				departmentsPerGp[key].forEach((department) => {
					if (!departmentSet.has(department)) {
						departmentSet.add(department);
						departments.push(department);
					}
				});
			});
			return departments
		}
	});

	onMount(() => {
		const maybeStoredFilter = currentGovProposalFilterStore.value;
		if (maybeStoredFilter !== null) {
			if (maybeStoredFilter.has_vote_result)
				genericFilters[0].activeValue = maybeStoredFilter.has_vote_result;
			if (maybeStoredFilter.legis_period) legisPeriodFilter.activeValue = maybeStoredFilter.legis_period;
			if (maybeStoredFilter.topics !== null) {
				selectedTopics = new SvelteSet(maybeStoredFilter.topics)
			}
			if (maybeStoredFilter.departments !== null) {
				selectedDepartments = new SvelteSet(maybeStoredFilter.departments)
			}
		}
	})

	const loadGovProps = async () => {
		let filter: GovPropFilter = {
			has_vote_result:
				genericFilters[0].activeValue == undefined ? null : genericFilters[0].activeValue,
			legis_period: legisPeriodFilter.activeValue == 'all' ? null : legisPeriodFilter.activeValue,
			topics: selectedTopics.size > 0 ? [...selectedTopics] : null,
			departments: selectedDepartments.size > 0 ? [...selectedDepartments] : null,
		};
		currentGovProposalFilterStore.value = filter;
		
		const nextUrl = new URL(page.url);
		
		let paramPage = nextUrl.searchParams.get('page');
		if (paramPage == null) {
			paramPage = '1';
		}

		nextUrl.search = '';
		
		if (paramPage) nextUrl.searchParams.set('page', paramPage);
		if (filter.has_vote_result) {
			nextUrl.searchParams.set("gov_proposal[ministrial_proposal][has_vote_result][eq]", filter.has_vote_result.toString());
		}
		if (filter.legis_period !== null) {
        	nextUrl.searchParams.set('gov_proposal[ministrial_proposal][gp][in][0]', filter.legis_period);
    	}
		filter.topics?.forEach((topic, i) => {
  	    	nextUrl.searchParams.set(`gov_proposal[eurovoc_topics][${i}][topic][cn]`, topic);
    	});
		filter.departments?.forEach((department, i) => {
  	    	nextUrl.searchParams.set(`gov_proposal[ministrial_proposal][ressort][in][${i}]`, department);
    	});
    	
		nextUrl.searchParams.set('search', searchValue);

		goto(nextUrl, {
			keepFocus: true,
			replaceState: true,
			noScroll: true
		});

	};

	const update = () => {
		loadGovProps();
	};	

	$effect(() => {
		void searchValue;
		void selectedTopics.size;
		void selectedDepartments.size;
		void genericFilters[0].activeValue;
		void legisPeriodFilter.activeValue;
		untrack(update);
	});

	let topics: string[] = $state([]);

	onMount(async () => {
		update();
	
		// Generic filter - Legislative period
		const fetchedPeriods = await cachedAllLegisPeriods();
		if (fetchedPeriods) {
			legisPeriodFilter.options = [
				{ title: 'Alle', value: 'all' },
				...fetchedPeriods.map((p) => ({ title: p.gp, value: p.gp }))
			];
		}

		const eurovocTopics = errorToNull(await get_eurovoc_topics());
		if (eurovocTopics) {
			topics = eurovocTopics.map(topic => topic.topic)
		}
	});
</script>

<!-- <FiltersAny bind:filters bind:selectedPeriod bind:searchValue {update} /> -->
<span class="mb-2 ml-1 block text-base text-gray-800 dark:text-gray-300 sm:mt-1 sm:ml-0">
	Ministerialentwürfe aktualisiert am: {updatedAt}
</span>

<div class="mt-7 md:flex">
	<!-- Search bar -->
	<SearchBar bind:searchValue />	
	<MultiValuesFilter title="Ministerien" bind:selectedValues={selectedDepartments} values={departments} />
	<MultiValuesFilter title="Themen" bind:selectedValues={selectedTopics} values={topics} />
	<GenericFilters 
		bind:genericFilters
		bind:legisPeriodFilter
	/>
</div>
<div>
	{#if govProposals}
		<!-- <Pagination bind:page maxPage={govProposals.max_page} /> -->
		{#if govProposals.gov_proposals.length > 0}
			{#each govProposals.gov_proposals as govProposal}
				<GovProposalExpandableBar {govProposal} showDelegate class="" />
			{/each}
		{:else if false}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Ministerialentwürfe gefunden
		{/if}
		<div class="float-right">
			<Pagination maxPage={govProposals.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
