<script lang="ts">
	import type { DelegateData } from '$lib/types';
	import { type PopupSettings } from '@skeletonlabs/skeleton-svelte';
	import LegisButtons from '../Filtering/LegisButtons.svelte';
	import { onMount } from 'svelte';
	import ReactiveDelegateBarChart from './ReactiveDelegateBarChart.svelte';

	export let delegateMakeRequest: (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	) => Promise<DelegateData[]>;
	export let height: number;
	export let title: string;
	export let id: number;

	let currentData: DelegateData[] = [];
	let filteredData: DelegateData[] = [];
	let filterParties: string[] = [];

	const addUniqueParties = () => {
		uniqueParties = [];
		currentData.forEach((data) => {
			const party = data.party ?? ''; // Setze `party` auf einen leeren String, falls `null`
			if (!uniqueParties.includes(party)) {
				uniqueParties.push(party);
			}
		});
	};

	onMount(async () => {
		// currentData = await delegateMakeRequest(null, null, true, true);
		// filteredData = currentData;
	});

	const popupParty: PopupSettings = {
		event: 'click',
		target: 'popupParty' + id,
		placement: 'bottom'
	};

	const popupGender: PopupSettings = {
		event: 'click',
		target: 'popupGender' + id,
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const popupNormalized: PopupSettings = {
		event: 'click',
		target: 'popupNormalized' + id,
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const popupDesc: PopupSettings = {
		event: 'click',
		target: 'popupDesc' + id,
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	let selectedPeriod: string = 'XXVIII';
	let gender: string | undefined = undefined;
	let uniqueParties: string[] = [];
	let normalized: boolean = true;
	let isDesc: boolean = true;

	$: if (currentData && filterParties && filterParties.length > 0) {
		filteredData = currentData.filter((data) => {
			return filterParties.includes(data.party ?? '');
		});
	} else {
		filteredData = currentData;
	}

	$: if (selectedPeriod || gender || isDesc) {
		let gp: string | null = selectedPeriod;
		if (selectedPeriod == 'all') {
			gp = null;
		}
		delegateMakeRequest(gp, gender ?? null, isDesc, normalized).then((res) => {
			currentData = res;
		});
	}

	$: if (currentData) {
		addUniqueParties();
	}

	const translatePartyFilter = (filterParties: string[]) => {
		if (filterParties.length == 0) {
			return 'Alle';
		}
		if (filterParties.length == 1) {
			return filterParties[0];
		}

		let idx = 0;
		let text = `${filterParties[idx]}`;
		if (text.length > 15) {
			return `${text.slice(0, 15)}...`;
		}
		while (text.length < 15 && idx <= filterParties.length - 2) {
			idx += 1;
			text = `${text}, ${filterParties[idx]}`;
			if (text.length > 15) {
				return `${text.slice(0, 15)}...`;
			}
		}
		return `${text}`;
	};

	const translateGenderFilter = (gender: string | undefined) => {
		if (!gender) {
			return 'egal';
		}
		if (gender == 'f') {
			return 'weiblich';
		}
		if (gender == 'm') {
			return 'männlich';
		}
	};

	const translateNormalizationFilter = (normalized: boolean | undefined) => {
		if (normalized) {
			return 'Ja';
		}
		return 'Nein';
	};

	const translateDescFilter = (isDesc: boolean | undefined) => {
		if (isDesc) {
			return 'Ja';
		}
		return 'Nein';
	};
</script>

<LegisButtons bind:selectedPeriod />

<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupParty{id}">
	<ListBox
		rounded="rounded-container sm:!rounded-base"
		active="preset-filled-secondary-500"
		hover="hover:preset-tonal-secondary"
		multiple
	>
		{#each uniqueParties as party}
			<ListBoxItem bind:group={filterParties} name="partyFilter" value={party}>{party}</ListBoxItem>
		{/each}
	</ListBox>
</div>
<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupGender{id}">
	<ListBox
		rounded="rounded-container sm:!rounded-base"
		active="preset-filled-secondary-500"
		hover="hover:preset-tonal-secondary"
	>
		<ListBoxItem bind:group={gender} name="genderName" value={undefined}>egal</ListBoxItem>
		<ListBoxItem bind:group={gender} name="genderName" value={'f'}>weiblich</ListBoxItem>
		<ListBoxItem bind:group={gender} name="genderName" value={'m'}>männlich</ListBoxItem>
	</ListBox>
</div>
<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupNormalized{id}">
	<ListBox
		rounded="rounded-container sm:!rounded-base"
		active="preset-filled-secondary-500"
		hover="hover:preset-tonal-secondary"
	>
		<ListBoxItem bind:group={normalized} name="normalization" value={true}>Ja</ListBoxItem>
		<ListBoxItem bind:group={normalized} name="normalization" value={false}>Nein</ListBoxItem>
	</ListBox>
</div>
<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupDesc{id}">
	<ListBox
		rounded="rounded-container sm:!rounded-base"
		active="preset-filled-secondary-500"
		hover="hover:preset-tonal-secondary"
	>
		<ListBoxItem bind:group={isDesc} name="isDesc" value={true}>Ja</ListBoxItem>
		<ListBoxItem bind:group={isDesc} name="isDesc" value={false}>Nein</ListBoxItem>
	</ListBox>
</div>

<div class="flex flex-wrap gap-6">
	<div>
		<h1 class="text-2xl font-bold">Partei</h1>
		<button class="btn preset-filled-secondary-500 w-48 justify-between" use:popup={popupParty}>
			<span class="capitalize">{translatePartyFilter(filterParties)}</span>
			<span>↓</span>
		</button>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Geschlecht</h1>
		<button class="btn preset-filled-secondary-500 w-48 justify-between" use:popup={popupGender}>
			<span class="capitalize">{translateGenderFilter(gender)}</span>
			<span>↓ </span>
		</button>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Normalisiert</h1>
		<button class="btn preset-filled-secondary-500 w-48 justify-between" use:popup={popupNormalized}>
			<span class="capitalize">{translateNormalizationFilter(normalized)}</span>
			<span>↓</span>
		</button>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Absteigend</h1>
		<button class="btn preset-filled-secondary-500 w-48 justify-between" use:popup={popupDesc}>
			<span class="capitalize">{translateDescFilter(isDesc)}</span>
			<span>↓</span>
		</button>
	</div>
</div>

<div class="graphic-container">
	<ReactiveDelegateBarChart {height} delegateData={filteredData} {title} />
</div>

<style>
	.graphic-container {
		min-height: 300px;
		max-height: 300px;
		overflow-y: auto;
		overflow-x: hidden;
		padding: 30px;
	}
</style>
