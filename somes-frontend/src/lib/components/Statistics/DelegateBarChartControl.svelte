<script lang="ts">
	import { BarChart, Tooltip } from 'layerchart';
	import { onMount } from 'svelte';
	import type { StatisticsData } from '$lib/types';
	import { Select } from 'bits-ui';
	import SButton from '$lib/components/UI/SButton.svelte';

	export let delegateMakeRequest: (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	) => Promise<StatisticsData[]>;
	export let height: number = 400;
	export let selectedCategory: string = 'delegate';
	export let valueLabel: string = 'Wert';
	export let normalizedValueLabel: string = 'Wert (normalisiert)';
	export let infoQuestion: string | null = null;
	export let infoAnswer: string | null = null;
	export let filterConfig: {
		showNormalized?: boolean;
		showPeriod?: boolean;
		showGender?: boolean;
		showParty?: boolean;
	} = {
		showNormalized: true,
		showPeriod: true,
		showGender: true,
		showParty: true
	};

	let currentData: StatisticsData[] = [];
	let filteredData: StatisticsData[] = [];
	let loading = false;
	let error: string | null = null;

	
	// All available periods in order (oldest to newest)
	const periods = ['XX', 'XXI', 'XXII', 'XXIII', 'XXIV', 'XXV', 'XXVI', 'XXVII', 'XXVIII'];
	let selectedGenders: string[] = ['m', 'f']; 
	let normalized: boolean = true;
	let isDesc: boolean = true;
	let selectedParties: string[] = [];

	// Slider states
	let selectedPeriod: string = 'XXVIII';
	let showAllPeriods: boolean = false;
	let sliderValue: number = periods.indexOf(selectedPeriod);

	// Automatically set showAllPeriods to true when category is 'legis'
	$: if (selectedCategory === 'legis' && !showAllPeriods) {
		showAllPeriods = true;
	}

	// Update selected period when slider changes
	$: if (!showAllPeriods && sliderValue >= 0 && sliderValue < periods.length) {
		selectedPeriod = periods[sliderValue];
	}
	
	// Track previous values to avoid unnecessary reloads
	let prevPeriod = selectedPeriod;
	let prevShowAll = showAllPeriods;
	let prevGenders = [...selectedGenders];
	let prevIsDesc = isDesc;
	let prevNormalized = normalized;
	let prevFunction = delegateMakeRequest;

	// Initial load
	onMount(() => {
		loadData();
	});
	$: if (selectedPeriod !== prevPeriod || showAllPeriods !== prevShowAll || JSON.stringify(selectedGenders) !== JSON.stringify(prevGenders) || isDesc !== prevIsDesc || normalized !== prevNormalized || delegateMakeRequest !== prevFunction) {
		prevPeriod = selectedPeriod;
		prevShowAll = showAllPeriods;
		prevGenders = [...selectedGenders];
		prevIsDesc = isDesc;
		prevNormalized = normalized;
		prevFunction = delegateMakeRequest;
		loadData();
	}

	$: if (currentData.length > 0 && selectedParties.length > 0) {
		filteredData = currentData.filter((data) => 
			data.type === 'delegate' && data.party && selectedParties.includes(data.party)
		);
	} else {
		filteredData = currentData;
	}

	async function loadData() {
		loading = true;
		error = null;
		
		try {
			const gp = showAllPeriods ? null : selectedPeriod;
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

	// Get unique parties from current data (only for delegate type)
	$: uniqueParties = partyOrder.filter(party => 
		currentData.filter(d => d.type === 'delegate').map(d => d.party).filter(Boolean).includes(party)
	);

	// Austrian party color mapping
	const partyColors: Record<string, string> = {
		'ÖVP': '#00CED1', // Türkis
		'SPÖ': '#E53935', // Rot
		'FPÖ': '#0D47A1', // Dunkelblau
		'Die Grünen': '#2E7D32', // Grün
		'GRÜNE': '#2E7D32', // Grün (alternative)
		'NEOS': '#E91E63', // Pink/Rose
		'KPÖ': '#E53935', // Rot
		'Regierungsmitglied': '#757575', // Grau
		'ÖVP.Österreichische Volkspartei': '#00CED1', // ÖVP (vollständig)
		'SPÖ.Sozialdemokratische Partei Österreichs': '#E53935', // SPÖ (vollständig)
		'FPÖ.Freiheitliche Partei Österreichs': '#0D47A1', // FPÖ (vollständig)
		'Die Grünen.Die Grüne Alternative': '#2E7D32', // Grüne (vollständig)
		'NEOS.Das Neue Österreich': '#E91E63', // NEOS (vollständig)
	};

	// Create stable party order and color range
	const partyOrder = Object.keys(partyColors);
	$: uniquePartiesInData = [...new Set(chartData.map(d => d.party))];
	$: colorRange = uniquePartiesInData.map((p) => partyColors[p] || '#6366f1');

	// Prepare data for LayerChart with all available metrics
	$: chartData = filteredData.map(item => {
		// Handle old format (name, party, data) and new format (StatisticsData)
		if (item.type === 'delegate') {
			const normalizedParty = item.party?.trim() || 'Unbekannt';
			const label = normalized ? normalizedValueLabel : valueLabel;
			return {
				category: `${item.label} (${normalizedParty})`,
				value: item.value,
				party: normalizedParty,
				color: partyColors[normalizedParty] || '#6366f1',
				valueLabel: label,
				metadata: item.metadata
			};
		} else if (item.type === 'category') {
			const label = normalized ? normalizedValueLabel : valueLabel;
			return {
				category: item.label,
				value: item.value,
				party: 'default',
				color: '#6366f1',
				valueLabel: label,
				metadata: item.metadata
			};
		} else {
			// Old format fallback - cast to any to handle legacy data
			const legacyItem = item as any;
			const normalizedParty = legacyItem.party?.trim() || 'Unbekannt';
			const label = normalized ? normalizedValueLabel : valueLabel;
			return {
				category: `${legacyItem.name} (${normalizedParty})`,
				value: legacyItem.data || 0,
				party: normalizedParty,
				color: partyColors[normalizedParty] || '#6366f1',
				valueLabel: label,
				metadata: {}
			};
		}
	});

	

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



	function toggleAllPeriods() {
		showAllPeriods = !showAllPeriods;
	}
	
	function handleSliderChange(event: Event) {
		const target = event.target as HTMLInputElement;
		sliderValue = parseInt(target.value);
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
	<div class="bg-gradient-to-r from-surface-50 to-surface-100 dark:from-surface-800 dark:to-surface-900 rounded-xl p-6 border border-surface-200 dark:border-surface-700">
		<div class="flex items-center gap-2 mb-4">
			<div class="w-1 h-5 bg-primary-500 rounded-full"></div>
			<h3 class="text-lg font-semibold text-surface-800 dark:text-surface-200">Filter & Einstellungen</h3>
		</div>
		
		<!-- Legislaturperiode Slider - Full Width -->
		{#if filterConfig.showPeriod}
		<div class="space-y-4 mb-6">
			<div class="text-sm font-medium text-surface-700 dark:text-surface-300 flex items-center gap-2">
				<span class="w-2 h-2 bg-primary-500 rounded-full"></span>
				Legislaturperiode
			</div>
			
			<!-- Alle Perioden Toggle -->
			<div class="flex items-center gap-3">
				<button
					onclick={toggleAllPeriods}
					class={`px-4 py-2 rounded-lg font-medium text-sm transition-all duration-200 ${
						showAllPeriods 
							? 'bg-primary-500 text-white shadow-lg shadow-primary-500/30' 
							: 'bg-surface-100 dark:bg-surface-700 text-surface-700 dark:text-surface-300 hover:bg-surface-200 dark:hover:bg-surface-600'
					}`}
					role="switch"
					aria-checked={showAllPeriods}
					aria-label="Alle Legislaturperioden auswählen"
				>
					<span class="flex items-center gap-2">
						{#if showAllPeriods}
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
							</svg>
							Alle Perioden
						{:else}
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
							</svg>
							Alle Perioden
						{/if}
					</span>
				</button>
			</div>
			
			<!-- Large Slider Container (hidden when category is legis) -->
			{#if selectedCategory !== 'legis'}
			<div class={`space-y-3 transition-all duration-300 ${showAllPeriods ? 'opacity-50 pointer-events-none' : ''}`}>
				<div class="relative">
					<!-- Slider Track -->
					<div class="relative h-3 bg-surface-200 dark:bg-surface-700 rounded-full">
						<!-- Progress Bar -->
						<div 
							class="absolute h-full bg-gradient-to-r from-primary-400 to-primary-600 rounded-full transition-all duration-300"
							style="width: {(sliderValue / (periods.length - 1)) * 100}%"
						></div>
					</div>
					
					<!-- Slider Input -->
					<input
						type="range"
						min="0"
						max={periods.length - 1}
						bind:value={sliderValue}
						onchange={handleSliderChange}
						disabled={showAllPeriods}
						class="absolute top-0 w-full h-3 opacity-0 cursor-pointer disabled:cursor-not-allowed"
						aria-label="Legislaturperiode auswählen"
					/>
					
					<!-- Slider Thumb -->
					<div 
						class="absolute top-1/2 -translate-y-1/2 w-8 h-8 bg-white border-4 border-primary-500 rounded-full shadow-lg transition-all duration-300 hover:scale-110 disabled:opacity-50 disabled:hover:scale-100"
						style="left: {(sliderValue / (periods.length - 1)) * 100}%; transform: translateX(-50%);"
					></div>
				</div>
				
				<!-- Period Labels -->
				<div class="relative h-6">
					{#each periods as period, index}
						<span
							class="absolute text-sm text-surface-600 dark:text-surface-400 font-medium transition-all duration-200"
							style="left: {(index / (periods.length - 1)) * 100}%; transform: translateX(-50%);"
						>
							<span class={`${index === sliderValue && !showAllPeriods 
								? 'text-primary-600 dark:text-primary-400 font-bold text-lg' 
								: ''}`}>
								{period}
							</span>
						</span>
					{/each}
				</div>
				
				<!-- Current Selection Display -->
				<div class="text-center">
					<span class="inline-flex items-center gap-2 px-4 py-2 bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300 rounded-full text-base font-medium">
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
						</svg>
						{showAllPeriods ? 'Alle Perioden' : periods[sliderValue]}
					</span>
				</div>
			</div>
			{/if}
		</div>
		{/if}
		<!-- Other Filters - Centered Container -->
		<div class="flex flex-col items-center space-y-4">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 w-full max-w-2xl" class:md:grid-cols-3={selectedCategory !== 'gender'}>
				<!-- Geschlecht (hidden when category is gender) -->
				{#if filterConfig.showGender && selectedCategory !== 'gender'}
					<div class="space-y-2">
						<div class="text-sm font-medium text-surface-700 dark:text-surface-300 flex items-center gap-2">
							<span class="w-2 h-2 bg-secondary-500 rounded-full"></span>
							Geschlecht
						</div>
						<div class="flex gap-2" role="group" aria-label="Geschlecht filtern">
							<button
								class="px-4 py-2 rounded-lg font-medium text-sm transition-all duration-200 {selectedGenders.includes('m') ? 'bg-primary-500 text-white shadow-lg shadow-primary-500/30' : 'bg-surface-100 dark:bg-surface-700 text-surface-700 dark:text-surface-300 hover:bg-surface-200 dark:hover:bg-surface-600'}"
								onclick={() => toggleGender('m')}
							>
								Männlich
							</button>
							<button
								class="px-4 py-2 rounded-lg font-medium text-sm transition-all duration-200 {selectedGenders.includes('f') ? 'bg-tertiary-500 text-white shadow-lg shadow-tertiary-500/30' : 'bg-surface-100 dark:bg-surface-700 text-surface-700 dark:text-surface-300 hover:bg-surface-200 dark:hover:bg-surface-600'}"
								onclick={() => toggleGender('f')}
							>
								Weiblich
							</button>
						</div>
					</div>
				{/if}

				<!-- Sortierung -->
				<div class="space-y-2">
					<div class="text-sm font-medium text-surface-700 dark:text-surface-300 flex items-center gap-2">
						<span class="w-2 h-2 bg-secondary-600 rounded-full"></span>
						Sortierung
					</div>
					<button
						onclick={toggleSort}
						class="h-10 bg-white dark:bg-surface-800 border border-surface-300 dark:border-surface-600 rounded-lg px-3 hover:bg-surface-50 dark:hover:bg-surface-700 transition-all duration-200 flex items-center justify-between group w-40"
						aria-label="Sortierung umkehren"
					>
						<span class="text-surface-700 dark:text-surface-300 font-medium text-sm">
							{isDesc ? 'Absteigend' : 'Aufsteigend'}
						</span>
						<div class={`transition-transform duration-300 ${isDesc ? 'rotate-0' : 'rotate-180'}`}>
							<svg class="w-5 h-5 text-surface-500 group-hover:text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
							</svg>
						</div>
					</button>
				</div>

				<!-- Normalisiert -->
				{#if filterConfig.showNormalized}
				<div class="space-y-2">
					<div class="text-sm font-medium text-surface-700 dark:text-surface-300 flex items-center gap-2">
						<span class="w-2 h-2 bg-tertiary-600 rounded-full"></span>
						Normalisiert
					</div>
					<button
						onclick={() => normalized = !normalized}
						class={`h-10 rounded-lg px-3 transition-all duration-200 flex items-center justify-between w-40 ${
							normalized
								? 'bg-tertiary-500 text-white shadow-md'
								: 'bg-white dark:bg-surface-800 text-surface-700 dark:text-surface-300 border border-surface-300 dark:border-surface-600 hover:bg-surface-50 dark:hover:bg-surface-700 shadow-sm'
						}`}
						role="switch"
						aria-checked={normalized}
						aria-label="Normalisierung umschalten"
					>
						<span class="font-medium text-sm">{normalized ? 'Ja' : 'Nein'}</span>
						<div class="relative">
							<div class={`w-12 h-6 rounded-full transition-colors duration-200 ${
								normalized ? 'bg-tertiary-600' : 'bg-surface-300 dark:bg-surface-600'
							}`}></div>
							<div class={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform duration-200 ${
								normalized ? 'translate-x-7' : 'translate-x-1'
							}`}></div>
						</div>
					</button>
				</div>
				{/if}
			</div>
		</div>

	<!-- Party Filters -->
	{#if uniqueParties.length > 0}
		<div class="mt-6 bg-gradient-to-r from-surface-50 to-surface-100 dark:from-surface-800 dark:to-surface-900 rounded-xl p-6 border border-surface-200 dark:border-surface-700">
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-2">
					<div class="w-1 h-5 bg-primary-500 rounded-full"></div>
					<h3 class="text-lg font-semibold text-surface-800 dark:text-surface-200">Parteien-Filter</h3>
				</div>
				{#if selectedParties.length > 0}
					<SButton 
						class="text-sm px-4 py-2 bg-white dark:bg-surface-800 text-surface-700 dark:text-surface-300 border border-surface-300 dark:border-surface-600 hover:bg-surface-50 dark:hover:bg-surface-700 transition-colors" 
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
								? 'bg-primary-500 text-primary-foreground shadow-lg ring-2 ring-primary-500/20' 
								: 'bg-white dark:bg-surface-800 text-surface-700 dark:text-surface-300 border border-surface-300 dark:border-surface-600 hover:bg-surface-50 dark:hover:bg-surface-700 shadow-sm'
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
	</div>

	<!-- Chart -->
	<div class="w-full bg-white dark:bg-surface-800 rounded-2xl shadow-xl border border-surface-200 dark:border-surface-700 overflow-hidden" style="min-height: {height}px;">
		{#if loading}
			<div class="flex flex-col items-center justify-center p-8 gap-4" style="min-height: {height}px;">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-surface-200 dark:border-surface-700 border-t-primary-500 rounded-full animate-spin"></div>
					<div class="absolute inset-0 w-12 h-12 border-4 border-transparent border-t-primary-500/30 rounded-full animate-spin animation-delay-150"></div>
				</div>
				<div class="text-center">
					<p class="text-lg font-medium text-surface-700 dark:text-surface-300 animate-pulse">Daten werden geladen</p>
					<p class="text-sm text-surface-500 dark:text-surface-400">Bitte warten Sie einen Moment...</p>
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
				<div class="w-16 h-16 bg-surface-100 dark:bg-surface-800 rounded-full flex items-center justify-center">
					<svg class="w-8 h-8 text-surface-400 dark:text-surface-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
					</svg>
				</div>
				<div class="text-center">
					<p class="text-lg font-medium text-surface-700 dark:text-surface-300">Keine Daten verfügbar</p>
					<p class="text-sm text-surface-500 dark:text-surface-400">Versuchen Sie, die Filter anzupassen oder eine andere Kategorie zu wählen</p>
				</div>
			</div>
		{:else if chartData && chartData.length > 0}
			<div class="p-4" style="height: {height}px;">
				<div class="flex items-start justify-between mb-2">
					<div class="text-sm font-medium text-surface-600 dark:text-surface-400">
						{normalized ? normalizedValueLabel : valueLabel}
					</div>
					{#if infoQuestion && infoAnswer}
						<div class="flex items-center gap-2">
							{#if infoQuestion}
								<div class="text-sm font-medium text-surface-600 dark:text-surface-400">
									{infoQuestion}
								</div>
							{/if}
							<div class="relative group">
								<button 
									type="button" 
									class="w-4 h-4 rounded-full bg-primary-100 hover:bg-primary-200 text-primary-600 flex items-center justify-center text-xs font-medium transition-colors"
									onclick={(e) => e.preventDefault()}
								>
									i
								</button>
								<div class="absolute right-0 top-6 w-80 p-3 bg-white dark:bg-surface-800 border border-surface-200 dark:border-surface-600 rounded-lg shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-50">
									<div class="text-sm">
										<div class="text-surface-700 dark:text-surface-300 space-y-1">
											{@html infoAnswer}
										</div>
									</div>
								</div>
							</div>
						</div>
					{/if}
				</div>
				{console.log(chartData)}
				<BarChart
					data={chartData}
					x="category"
					y="value"
					c="party"
					cRange={colorRange}
					orientation="vertical"
					padding={{ left: 80, right: 20, top: 20, bottom: 60 }}
					tooltip={true}
				/>
			</div>
		{:else}
			<div class="flex flex-col items-center justify-center p-8 gap-4" style="min-height: {height}px;">
				<div class="w-16 h-16 bg-surface-100 dark:bg-surface-800 rounded-full flex items-center justify-center">
					<svg class="w-8 h-8 text-surface-400 dark:text-surface-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
					</svg>
				</div>
				<div class="text-center">
					<p class="text-lg font-medium text-surface-700 dark:text-surface-300">Keine Daten verfügbar</p>
					<p class="text-sm text-surface-500 dark:text-surface-400">Versuchen Sie, die Filter anzupassen oder eine andere Kategorie zu wählen</p>
				</div>
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
