<script lang="ts">
	import { Chart, BarChart, Axis, Tooltip } from 'layerchart';
	import { onMount } from 'svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';
	import SButton from '$lib/components/UI/SButton.svelte';	

	export let delegateMakeRequest: (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	) => Promise<DelegateData[]>;
	export let height: number = 400;
	export let title: string;

	let currentData: DelegateData[] = [];
	let filteredData: DelegateData[] = [];
	let loading = false;
	let error: string | null = null;

	// Filter states
	let selectedPeriod: string = 'XXVIII';
	let gender: string = 'all';
	let normalized: string = 'true';
	let isDesc: string = 'true';
	let selectedParties: string[] = [];

	// Select options
	const periodOptions = [
		{ value: 'all', label: 'Alle' },
		{ value: 'XXVIII', label: 'XXVIII' },
		{ value: 'XXVII', label: 'XXVII' }
	];

	const genderOptions = [
		{ value: 'all', label: 'Alle' },
		{ value: 'm', label: 'Männlich' },
		{ value: 'f', label: 'Weiblich' }
	];

	const sortOptions = [
		{ value: 'true', label: 'Absteigend' },
		{ value: 'false', label: 'Aufsteigend' }
	];

	const normalizedOptions = [
		{ value: 'true', label: 'Ja' },
		{ value: 'false', label: 'Nein' }
	];

	// Track previous values to avoid unnecessary reloads
	let prevPeriod = selectedPeriod;
	let prevGender = gender;
	let prevIsDesc = isDesc;
	let prevFunction = delegateMakeRequest;

	// Reactive statements
	$: if (selectedPeriod !== prevPeriod || gender !== prevGender || isDesc !== prevIsDesc || delegateMakeRequest !== prevFunction) {
		prevPeriod = selectedPeriod;
		prevGender = gender;
		prevIsDesc = isDesc;
		prevFunction = delegateMakeRequest;
		loadData();
	}

	$: if (currentData.length > 0 && selectedParties.length > 0) {
		filteredData = currentData.filter((data) => 
			selectedParties.includes(data.party || '')
		);
	} else {
		filteredData = currentData;
	}

	async function loadData() {
		loading = true;
		error = null;
		
		try {
			const gp = selectedPeriod === 'all' ? null : selectedPeriod;
			const genderFilter = gender === 'all' ? null : gender;
			
			const result = await delegateMakeRequest(gp, genderFilter, isDesc === 'true', normalized === 'true');
			currentData = result;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Unknown error occurred';
		} finally {
			loading = false;
		}
	}

	// Get unique parties from current data
	$: uniqueParties = [...new Set(currentData.map(d => d.party).filter(Boolean))];

	// Prepare data for LayerChart
	$: chartData = filteredData.map(item => ({
		category: item.name || item.party || 'Unknown',
		value: item.data,
		party: item.party
	}));

	function toggleParty(party: string) {
		if (selectedParties.includes(party)) {
			selectedParties = selectedParties.filter(p => p !== party);
		} else {
			selectedParties = [...selectedParties, party];
		}
	}

	function clearPartyFilters() {
		selectedParties = [];
	}

	onMount(() => {
		loadData();
	});
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<h2 class="text-2xl font-bold">{title}</h2>
		<div class="flex gap-2">
			{#if uniqueParties.length > 0}
				<SButton class="text-sm px-3 py-1" on:click={clearPartyFilters}>
					Filter löschen
				</SButton>
			{/if}
		</div>
	</div>

	<!-- Filters -->
	<div class="flex flex-wrap gap-4 p-4 bg-muted rounded-lg">
		<div class="space-y-2">
			<label class="text-sm font-medium">Legislaturperiode</label>
			<Select.Root
				type="single"
				bind:value={selectedPeriod}
				items={periodOptions}
			>
				<Select.Trigger class="w-40">
					<span>Periode wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each periodOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>

		<div class="space-y-2">
			<label class="text-sm font-medium">Geschlecht</label>
			<Select.Root
				type="single"
				bind:value={gender}
				items={genderOptions}
			>
				<Select.Trigger class="w-32">
					<span>Geschlecht</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each genderOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>

		<div class="space-y-2">
			<label class="text-sm font-medium">Sortierung</label>
			<Select.Root
				type="single"
				bind:value={isDesc}
				items={sortOptions}
			>
				<Select.Trigger class="w-32">
					<span>Sortierung</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each sortOptions as option}
								<Select.Item value={option.value.toString()} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>

		<div class="space-y-2">
			<label class="text-sm font-medium">Normalisiert</label>
			<Select.Root
				type="single"
				bind:value={normalized}
				items={normalizedOptions}
			>
				<Select.Trigger class="w-32">
					<span>Normalisiert</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each normalizedOptions as option}
								<Select.Item value={option.value.toString()} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
	</div>

	<!-- Party Filters -->
	{#if uniqueParties.length > 0}
		<div class="flex flex-wrap gap-2">
			<span class="text-sm font-medium">Parteien:</span>
			{#each uniqueParties as party}
				<SButton
						on:click={() => toggleParty(party || '')}
						class={`text-sm px-3 py-1 ${selectedParties.includes(party || '') ? "bg-primary text-primary-foreground" : ""}`}
					>
						{party}
					</SButton>
			{/each}
		</div>
	{/if}

	<!-- Chart -->
	<div class="w-full" style="height: {height}px;">
		{#if loading}
			<div class="flex items-center justify-center h-full">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
			</div>
		{:else if error}
			<div class="flex items-center justify-center h-full text-destructive">
				<p>{error}</p>
			</div>
		{:else if chartData.length === 0}
			<div class="flex items-center justify-center h-full text-muted-foreground">
				<p>Keine Daten verfügbar</p>
			</div>
		{:else}
			<BarChart
					data={chartData}
					x="category"
					y="value"
					color="party"
					orientation="vertical"
					padding={{ left: 60, right: 20, top: 20, bottom: 60 }}
			/>
		{/if}
	</div>
</div>

<style>
	:global(.layerchart) {
		font-family: inherit;
	}
</style>
