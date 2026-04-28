<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';

	type DelegateSpeechTime = {
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

	const delegateSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_delegate', {
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
				data: normalized ? val.average_speech_time : val.total_speech_time
			};
		});
	};

	const partySimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_party', {
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
				data: val.total_speech_time
			};
		});
	};

	const genderSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_gender', {
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
				party: val.delegate_gender,
				data: val.total_speech_time
			};
		});
	};

	const ageSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_age', {
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
				data: val.total_speech_time
			};
		});
	};

	const legisSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_legis', {
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
				data: normalized ? val.average_speech_time : val.total_speech_time
			};
		});
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleSpeechTime;
			case 'party': return partySimpleSpeechTime;
			case 'gender': return genderSimpleSpeechTime;
			case 'age': return ageSimpleSpeechTime;
			case 'legis': return legisSimpleSpeechTime;
			default: return delegateSimpleSpeechTime;
		}
	})();

	// Get the title based on selected category
	$: currentTitle = (() => {
		switch (selectedCategory) {
			case 'delegate': return 'Redezeit pro Abgeordneten (in Minuten)';
			case 'party': return 'Redezeit nach Parteien (in Minuten)';
			case 'gender': return 'Redezeit nach Gender (in Minuten)';
			case 'age': return 'Redezeit nach Alter (in Minuten)';
			case 'legis': return 'Redezeit nach Legislaturperiode (in Minuten)';
			default: return 'Redezeit pro Abgeordneten (in Minuten)';
		}
	})();
</script>

<svelte:head>
    <title>Redezeitstatistiken</title>
    <meta name="description" content="Redezeitstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Redezeitstatistiken</h1>

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
