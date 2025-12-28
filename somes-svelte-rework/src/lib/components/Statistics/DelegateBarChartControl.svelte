<script lang="ts">
	import type { DelegateData } from '$lib/types';
	import { ListBox, ListBoxItem, popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import LegisButtons from '../Filtering/LegisButtons.svelte';
	import { onMount } from 'svelte';
	import ReactiveDelegateBarChart from './ReactiveDelegateBarChart.svelte';
	import type { Config } from './types';

	export let delegateMakeRequest: (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	) => Promise<DelegateData[]>;
	export let height: number;
	export let title: string;
	export let id: number;
	export let config: Config | null


	let currentData: DelegateData[] = [];
	let filteredData: DelegateData[] = [];
	let filterPrimaries: string[] = [];

	



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

	const popupPrimary: PopupSettings = {
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

	$: if (currentData && filterPrimaries && filterPrimaries.length > 0) {
		filteredData = currentData.filter((data) => {
			return filterPrimaries.includes(data.party ?? '');
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

	const translatePrimaryFilter = (filterParties: string[]) => {
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
		if (text.length > 15) {
			return `${text.slice(0, 15)}...`;
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

const popupInfoPrimary: PopupSettings = {
    event: 'hover',
    target: 'popupInfoPrimary' + id,
    placement: 'right'
};

const popupInfoGender: PopupSettings = {
    event: 'hover',
    target: 'popupInfoGender' + id,
    placement: 'right'
};

const popupInfoNormalized: PopupSettings = {
    event: 'hover',
    target: 'popupInfoNormalized' + id,
    placement: 'right'
};

const popupInfoDesc: PopupSettings = {
    event: 'hover',
    target: 'popupInfoDesc' + id,
    placement: 'right'
};

const filterMap = {
		primary: {
			popup: popupPrimary,
			infoPopup: popupInfoPrimary,
			label: () => translatePrimaryFilter(filterPrimaries)
		},
		gender: {
			popup: popupGender,
			infoPopup: popupInfoGender,
			label: () => translateGenderFilter(gender)
		},
		normalized: {
			popup: popupNormalized,
			infoPopup: popupInfoNormalized,
			label: () => translateNormalizationFilter(normalized)
		},
		desc: {
			popup: popupDesc,
			infoPopup: popupInfoDesc,
			label: () => translateDescFilter(isDesc)
		}
	} as const;



</script>

<LegisButtons bind:selectedPeriod />

<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupParty{id}">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
		multiple
	>
		{#each uniqueParties as party}
			<ListBoxItem bind:group={filterPrimaries} name="partyFilter" value={party}>{party}</ListBoxItem>
		{/each}
	</ListBox>
</div>

<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupGender{id}">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={gender} name="genderName" value={undefined}>egal</ListBoxItem>
		<ListBoxItem bind:group={gender} name="genderName" value={'f'}>weiblich</ListBoxItem>
		<ListBoxItem bind:group={gender} name="genderName" value={'m'}>männlich</ListBoxItem>
	</ListBox>
</div>
<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupNormalized{id}">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={normalized} name="normalization" value={true}>Ja</ListBoxItem>
		<ListBoxItem bind:group={normalized} name="normalization" value={false}>Nein</ListBoxItem>
	</ListBox>
</div>
<div class="z-30 card w-48 shadow-xl py-2" data-popup="popupDesc{id}">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={isDesc} name="isDesc" value={true}>Ja</ListBoxItem>
		<ListBoxItem bind:group={isDesc} name="isDesc" value={false}>Nein</ListBoxItem>
	</ListBox>
</div>


{#each config?.filters ?? [] as filter (filter.id)}
	{#if filter.isShown && filter.infoText}
		<div class="z-40 card w-64 p-4 shadow-xl" data-popup={"popupInfo" + filter.id[0].toUpperCase() + filter.id.slice(1) + id}>
			<p class="text-sm">{filter.infoText}</p>
		</div>
	{/if}
{/each}


<div class="flex flex-wrap gap-6">
	<div class="flex flex-wrap gap-6">
	{#each config?.filters ?? [] as filter (filter.id)}
		{#if filter.isShown}
			<div>
				<div class="flex items-center gap-2">
					<h1 class="text-2xl font-bold">{filter.name}</h1>

					{#if filter.infoText}
						<button
							class="btn variant-soft-primary btn-circle btn-sm"
							use:popup={filterMap[filter.id].infoPopup}
						>
							i
						</button>
					{/if}
				</div>

				<button
					class="btn variant-filled-secondary w-48 justify-between"
					use:popup={filterMap[filter.id].popup}
				>
					<span class="capitalize">
						{filterMap[filter.id].label()}
					</span>
					<span>↓</span>
				</button>
			</div>
		{/if}
	{/each}
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
