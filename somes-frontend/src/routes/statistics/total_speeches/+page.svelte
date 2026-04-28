<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';

	type DelegateSpeeches = {
		delegate_name: string;
		delegate_party: string;
		total_speeches: number;
		total_speech_time: number;
		average_speech_time: number;
	};

	// Category selection
	let selectedCategory: string = 'delegate';
	
	const categoryOptions = [
		{ value: 'delegate', label: 'Pro Abgeordneten' },
		{ value: 'party', label: 'Nach Parteien' },
		{ value: 'gender', label: 'Nach Gender' },
		{ value: 'age', label: 'Nach Alter' },
		{ value: 'legis', label: 'Nach Legislaturperiode' }
	];

	const delegateSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: val.total_speeches
			};
		});
	};

	const partySimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.delegate_party,
				data: val.total_speeches
			};
		});
	};

	const genderSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: val.total_speeches
			};
		});
	};

	const ageSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: 'Unknown',
				data: val.total_speeches
			};
		});
	};

	const legisSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: 'All Periods',
				data: normalized ? val.average_speech_time : val.total_speeches
			};
		});
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleSpeeches;
			case 'party': return partySimpleSpeeches;
			case 'gender': return genderSimpleSpeeches;
			case 'age': return ageSimpleSpeeches;
			case 'legis': return legisSimpleSpeeches;
			default: return delegateSimpleSpeeches;
		}
	})();

	// Get the title based on selected category
	$: currentTitle = (() => {
		switch (selectedCategory) {
			case 'delegate': return 'Reden pro Abgeordneten';
			case 'party': return 'Reden nach Parteien';
			case 'gender': return 'Reden nach Gender';
			case 'age': return 'Reden nach Alter';
			case 'legis': return 'Reden nach Legislaturperiode';
			default: return 'Reden pro Abgeordneten';
		}
	})();
</script>

<svelte:head>
    <title>Redenzahlstatistiken</title>
    <meta name="description" content="Redenzahlstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Redenzahlstatistiken</h1>

	<!-- Category Selection -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center gap-4">
			<label class="text-lg font-medium">Analyse-Kategorie:</label>
			<Select.Root
				type="single"
				bind:value={selectedCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
	</div>

	<!-- Dynamic Chart -->
	<div class="bg-card rounded-xl p-6 shadow-sm">
		<DelegateBarChartControl
			height={500}
			delegateMakeRequest={currentFunction}
			title={currentTitle}
		/>
	</div>
</div>
