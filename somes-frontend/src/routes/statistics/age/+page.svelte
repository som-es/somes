<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';
	import SButton from '$lib/components/UI/SButton.svelte';

	type AgeForDelegate = {
		delegate_name: string;
		delegate_party: string;
		age: number;
	};

	type AgeByCategory = {
		category: string;
		average_age: number;
		delegate_count: number;
		min_age: number;
		max_age: number;
	};

	// Category selection
	let selectedCategory: string = 'delegate';
	
	const categoryOptions = [
		{ value: 'delegate', label: 'Pro Abgeordneten' },
		{ value: 'party', label: 'Nach Parteien' },
		{ value: 'gender', label: 'Nach Gender' },
		{ value: 'legis', label: 'Nach Legislaturperiode' }
	];

	const delegateSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AgeForDelegate[]>('age_of_delegates', {
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
				data: val.age
			};
		});
	};

	const partySimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AgeByCategory[]>('age_per_party', {
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
				party: val.category,
				data: val.average_age
			};
		});
	};

	const genderSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AgeByCategory[]>('age_per_gender', {
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
				party: val.category,
				data: val.average_age
			};
		});
	};

	const legisSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AgeByCategory[]>('age_per_legis', {
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
				party: val.category,
				data: val.average_age
			};
		});
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleAge;
			case 'party': return partySimpleAge;
			case 'gender': return genderSimpleAge;
			case 'legis': return legisSimpleAge;
			default: return delegateSimpleAge;
		}
	})();

	// Get the title based on selected category
	$: currentTitle = (() => {
		switch (selectedCategory) {
			case 'delegate': return 'Alter pro Abgeordneten';
			case 'party': return 'Alter nach Parteien';
			case 'gender': return 'Alter nach Gender';
			case 'legis': return 'Alter nach Legislaturperiode';
			default: return 'Alter pro Abgeordneten';
		}
	})();
</script>

<svelte:head>
    <title>Altersstatistiken</title>
    <meta name="description" content="Altersstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Altersstatistiken</h1>

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
