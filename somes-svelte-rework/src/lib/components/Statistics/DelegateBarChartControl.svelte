<script lang="ts">
	import type { DelegateData } from '$lib/types';
	import { ListBox, ListBoxItem, popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import LegisButtons from '../Filtering/LegisButtons.svelte';
	import { onMount } from 'svelte';
	import ReactiveDelegateBarChart from './ReactiveDelegateBarChart.svelte';
	import type { Config, FilterInfoStatistics, FilterOption } from './types';

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


	let singleValues: Record<string, FilterOption> = {};
	let multiValues: Record<string, FilterOption[]> = {};

	$: if (config) {
		for (const filter of config.filters) {
			if (filter.multiple) {
				// Multi-Werte: immer ein Array, niemals undefined
				multiValues[filter.id] = [];
				if (Array.isArray(filter.default)) {
					multiValues[filter.id] = filter.default;
				} else if (filter.default !== undefined) {
					multiValues[filter.id] = [filter.default as FilterOption];
				}
			} else {
				// Single-Werte: immer ein Wert, niemals undefined
				if (Array.isArray(filter.default)) {
					singleValues[filter.id] = filter.default[0] ?? filter.options?.[0] ?? '';
				} else {
					singleValues[filter.id] = filter.default ?? filter.options?.[0] ?? '';
				}
			}
		}
	}

	// Helper um sicher zu gehen, dass bind:group immer ein Wert hat
	const getGroup = (filter: FilterInfoStatistics) => {
		return filter.multiple
			? multiValues[filter.id] || []
			: singleValues[filter.id] ?? '';
	};


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


	const createPopup = (filterId: string, type: 'dropDown' | 'info') => ({
		event: type === 'dropDown' ? 'click' : 'hover',
		target: type === 'dropDown' ? 'popup_'+filterId+id: 'popup_info'+filterId+id,
		placement: type === 'dropDown' ? 'bottom' : 'right',
		closeQuery: type === 'dropDown' ? '.listbox-item' : undefined
	} as const);


	let filterPopups: Record<
		string,
		{ popup: PopupSettings; infoPopup: PopupSettings; label: () => string }
	> = {};

	$: if (config) {
		filterPopups = {};

		for (const filter of config.filters) {
			const labelFn = filter.label ?? ((val: any) => String(val ?? ''));

			filterPopups[filter.id] = {
				popup: createPopup(filter.id, 'dropDown'),
				infoPopup: createPopup(filter.id, 'info'),
				label: () =>
					labelFn(
						filter.multiple
							? multiValues[filter.id]
							: singleValues[filter.id]
					)
			};
		}
	}


const isGroupReady = (filter: any) =>
	filter.multiple
		? Array.isArray(multiValues[filter.id])
		: filter.id in singleValues;



</script>

<LegisButtons bind:selectedPeriod />


{#each config?.filters ?? [] as filter (filter.id)}
	{#if filter.isShown && filter.infoText}
		<div class="z-40 card w-64 p-4 shadow-xl" data-popup={"popup_info" + filter.id + id}>
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
							use:popup={filterPopups[filter.id].infoPopup}
						>
							i
						</button>
					{/if}
				</div>



				<div
					class="z-30 card w-48 shadow-xl py-2"
					data-popup={"popup_" + filter.id + id}
				>
					{#if isGroupReady(filter)}
						{#if filter.multiple}
							<ListBox
								name={filter.id}
								multiple
								bind:group={multiValues[filter.id]}
								rounded="rounded-container-token sm:!rounded-token"
								active="variant-filled-secondary"
								hover="hover:variant-soft-secondary"
							>
								{#each filter.options ?? [] as option}
									<ListBoxItem value={option}>
										{String(filter.label?.(option) ?? option)}
									</ListBoxItem>
								{/each}
							</ListBox>
						{:else}
							<ListBox
								name={filter.id}
								bind:group={singleValues[filter.id]}
								rounded="rounded-container-token sm:!rounded-token"
								active="variant-filled-secondary"
								hover="hover:variant-soft-secondary"
							>
								{#each filter.options ?? [] as option}
									<ListBoxItem value={option}>
										{String(filter.label?.(option) ?? option)}
									</ListBoxItem>
								{/each}
							</ListBox>
						{/if}
					{/if}
				</div>
				<button
					class="btn variant-filled-secondary w-48 justify-between"
					use:popup={filterPopups[filter.id].popup}
				>
					<span class="capitalize">
						{filterPopups[filter.id].label()}
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
