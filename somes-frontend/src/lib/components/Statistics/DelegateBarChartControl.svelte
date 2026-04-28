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

	let currentData: DelegateData[] = [];
	let filteredData: DelegateData[] = [];
	let loading = false;
	let error: string | null = null;

	// Filter states
	let selectedPeriod: string = 'XXVIII';
	let selectedGenders: string[] = ['m', 'f']; // Beide am Anfang ausgewählt
	let normalized: boolean = true;
	let isDesc: boolean = true;
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
	let prevGenders = [...selectedGenders];
	let prevIsDesc = isDesc;
	let prevNormalized = normalized;
	let prevFunction = delegateMakeRequest;

	// Initial load
	onMount(() => {
		loadData();
	});
	$: if (selectedPeriod !== prevPeriod || JSON.stringify(selectedGenders) !== JSON.stringify(prevGenders) || isDesc !== prevIsDesc || normalized !== prevNormalized || delegateMakeRequest !== prevFunction) {
		prevPeriod = selectedPeriod;
		prevGenders = [...selectedGenders];
		prevIsDesc = isDesc;
		prevNormalized = normalized;
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
			// Wenn beide Geschlechter ausgewählt sind, sende null (wie kein Filter)
			const genderFilter = selectedGenders.length === 2 ? null : selectedGenders.join(',');
			
			const result = await delegateMakeRequest(gp, genderFilter, isDesc, normalized);
			currentData = result;
			loading = false;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Unknown error occurred';
			loading = false;
		}
	}

	// Get unique parties from current data
	$: uniqueParties = [...new Set(currentData.map(d => d.party).filter(Boolean))];

	// Prepare data for LayerChart with all available metrics
	$: chartData = filteredData.map(item => ({
		category: item.name || item.party || 'Unknown',
		value: item.data,
		// Additional metadata for tooltips (if available in API response)
		...item
	}));

	function toggleGender(genderValue: string) {
		if (selectedGenders.includes(genderValue)) {
			// Verhindern dass das letzte Geschlecht abgewählt wird
			if (selectedGenders.length > 1) {
				selectedGenders = selectedGenders.filter(g => g !== genderValue);
			}
		} else {
			selectedGenders = [...selectedGenders, genderValue];
		}
	}

	function clearGenderFilters() {
		selectedGenders = [];
	}

	function toggleSort() {
		isDesc = !isDesc;
	}

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
	<!-- Filters -->
	<div class="bg-gradient-to-r from-slate-50 to-slate-100 dark:from-slate-800 dark:to-slate-900 rounded-xl p-6 border border-slate-200 dark:border-slate-700">
		<div class="flex items-center gap-2 mb-4">
			<div class="w-1 h-5 bg-primary rounded-full"></div>
			<h3 class="text-lg font-semibold text-slate-800 dark:text-slate-200">Filter & Einstellungen</h3>
		</div>
		
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
			<!-- Legislaturperiode -->
			<div class="space-y-2">
				<div class="text-sm font-medium text-slate-700 dark:text-slate-300 flex items-center gap-2">
					<span class="w-2 h-2 bg-blue-500 rounded-full"></span>
					Legislaturperiode
				</div>
				<Select.Root
					type="single"
					bind:value={selectedPeriod}
					items={periodOptions}
				>
					<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors" aria-label="Legislaturperiode wählen">
						<span class="text-slate-500 dark:text-slate-400">Periode wählen</span>
					</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg shadow-lg">
							<Select.Viewport>
								{#each periodOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-3 py-2 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer">
										{option.label}
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>

			<!-- Geschlecht -->
			<div class="space-y-2">
				<div class="text-sm font-medium text-slate-700 dark:text-slate-300 flex items-center gap-2">
					<span class="w-2 h-2 bg-green-500 rounded-full"></span>
					Geschlecht
				</div>
				<div class="flex gap-2" role="group" aria-label="Geschlecht filtern">
					<button
						class="px-4 py-2 rounded-lg font-medium text-sm transition-all duration-200 {selectedGenders.includes('m') ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30' : 'bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-600'}"
						onclick={() => toggleGender('m')}
					>
						Männlich
					</button>
					<button
						class="px-4 py-2 rounded-lg font-medium text-sm transition-all duration-200 {selectedGenders.includes('f') ? 'bg-pink-500 text-white shadow-lg shadow-pink-500/30' : 'bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-600'}"
						onclick={() => toggleGender('f')}
					>
						Weiblich
					</button>
				</div>
			</div>

			<!-- Sortierung -->
			<div class="space-y-2">
				<div class="text-sm font-medium text-slate-700 dark:text-slate-300 flex items-center gap-2">
					<span class="w-2 h-2 bg-purple-500 rounded-full"></span>
					Sortierung
				</div>
				<button
					onclick={toggleSort}
					class="w-full h-10 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-all duration-200 flex items-center justify-between group"
					aria-label="Sortierung umkehren"
				>
					<span class="text-slate-700 dark:text-slate-300 font-medium">
						{isDesc ? 'Absteigend' : 'Aufsteigend'}
					</span>
					<div class={`transition-transform duration-300 ${isDesc ? 'rotate-0' : 'rotate-180'}`}>
						<svg class="w-5 h-5 text-slate-500 group-hover:text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
						</svg>
					</div>
				</button>
			</div>

			<!-- Normalisiert -->
			<div class="space-y-2">
				<div class="text-sm font-medium text-slate-700 dark:text-slate-300 flex items-center gap-2">
					<span class="w-2 h-2 bg-orange-500 rounded-full"></span>
					Normalisiert
				</div>
				<button
					onclick={() => normalized = !normalized}
					class={`w-full h-10 rounded-lg px-3 transition-all duration-200 flex items-center justify-between ${
						normalized
							? 'bg-orange-500 text-white shadow-md'
							: 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-300 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700 shadow-sm'
					}`}
					role="switch"
					aria-checked={normalized}
					aria-label="Normalisierung umschalten"
				>
					<span class="font-medium">{normalized ? 'Ja' : 'Nein'}</span>
					<div class="relative">
						<div class={`w-12 h-6 rounded-full transition-colors duration-200 ${
							normalized ? 'bg-orange-600' : 'bg-slate-300 dark:bg-slate-600'
						}`}></div>
						<div class={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform duration-200 ${
							normalized ? 'translate-x-7' : 'translate-x-1'
						}`}></div>
					</div>
				</button>
			</div>
		</div>
	</div>

	<!-- Party Filters -->
	{#if uniqueParties.length > 0}
		<div class="bg-gradient-to-r from-slate-50 to-slate-100 dark:from-slate-800 dark:to-slate-900 rounded-xl p-6 border border-slate-200 dark:border-slate-700">
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-2">
					<div class="w-1 h-5 bg-primary rounded-full"></div>
					<h3 class="text-lg font-semibold text-slate-800 dark:text-slate-200">Parteien-Filter</h3>
				</div>
				{#if selectedParties.length > 0}
					<SButton 
						class="text-sm px-4 py-2 bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-300 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors" 
						onclick={clearPartyFilters}
					>
						<span class="flex items-center gap-2">
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
							</svg>
							Filter löschen ({selectedParties.length})
						</span>
					</SButton>
				{/if}
			</div>
			<div class="flex flex-wrap gap-2">
				{#each uniqueParties as party}
					<button
						onclick={() => toggleParty(party || '')}
						class={`inline-flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium transition-all duration-200 transform hover:scale-105 ${
							selectedParties.includes(party || '') 
								? 'bg-primary text-primary-foreground shadow-lg ring-2 ring-primary/20' 
								: 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border border-slate-300 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700 shadow-sm'
						}`}
					>
						{#if selectedParties.includes(party || '')}
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
							</svg>
						{/if}
						{party || 'Unbekannt'}
					</button>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Chart -->
	<div class="w-full bg-white dark:bg-slate-800 rounded-2xl shadow-xl border border-slate-200 dark:border-slate-700 overflow-hidden" style="min-height: {height}px;">
		{#if loading}
			<div class="flex flex-col items-center justify-center p-8 gap-4" style="min-height: {height}px;">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-slate-200 dark:border-slate-700 border-t-primary rounded-full animate-spin"></div>
					<div class="absolute inset-0 w-12 h-12 border-4 border-transparent border-t-primary/30 rounded-full animate-spin animation-delay-150"></div>
				</div>
				<div class="text-center">
					<p class="text-lg font-medium text-slate-700 dark:text-slate-300 animate-pulse">Daten werden geladen</p>
					<p class="text-sm text-slate-500 dark:text-slate-400">Bitte warten Sie einen Moment...</p>
				</div>
			</div>
		{:else if error}
			<div class="flex flex-col items-center justify-center p-8 gap-4" style="min-height: {height}px;">
				<div class="w-16 h-16 bg-red-100 dark:bg-red-900/20 rounded-full flex items-center justify-center">
					<svg class="w-8 h-8 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
					</svg>
				</div>
				<div class="text-center max-w-md">
					<p class="text-lg font-medium text-red-700 dark:text-red-400 mb-2">Fehler beim Laden der Daten</p>
					<p class="text-sm text-red-600 dark:text-red-500">{error}</p>
					<button 
						class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors shadow-lg shadow-red-500/30"
						onclick={loadData}
					>
						Erneut versuchen
					</button>
				</div>
			</div>
		{:else if chartData.length === 0}
			<div class="flex flex-col items-center justify-center p-8 gap-4" style="min-height: {height}px;">
				<div class="w-16 h-16 bg-slate-100 dark:bg-slate-800 rounded-full flex items-center justify-center">
					<svg class="w-8 h-8 text-slate-400 dark:text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
					</svg>
				</div>
				<div class="text-center">
					<p class="text-lg font-medium text-slate-700 dark:text-slate-300">Keine Daten verfügbar</p>
					<p class="text-sm text-slate-500 dark:text-slate-400">Versuchen Sie, die Filter anzupassen oder eine andere Kategorie zu wählen</p>
				</div>
			</div>
		{:else}
			<div class="p-4" style="height: {height}px;">
				<BarChart
					data={chartData}
					x="category"
					y="value"
					color="party"
					orientation="vertical"
					padding={{ left: 80, right: 20, top: 20, bottom: 60 }}
				/>
			</div>
		{/if}
	</div>
</div>

<style>
	:global(.layerchart) {
		font-family: inherit;
	}
	
	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
	
	.animation-delay-150 {
		animation-delay: 150ms;
	}
</style>
