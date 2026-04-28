<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';

	type DelegateCallsToOrder = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_order_calls: number;
		normalized_calls_to_order: number;
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

	const delegateSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_by_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls
			};
		});
	};

	const partySimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.delegate_party,
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls
			};
		});
	};

	const genderSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.delegate_gender,
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls
			};
		});
	};

	const ageSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.age_group,
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls
			};
		});
	};

	const legisSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.legislative_period,
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls
			};
		});
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleCallsToOrder;
			case 'party': return partySimpleCallsToOrder;
			case 'gender': return genderSimpleCallsToOrder;
			case 'age': return ageSimpleCallsToOrder;
			case 'legis': return legisSimpleCallsToOrder;
			default: return delegateSimpleCallsToOrder;
		}
	})();

	// Get the title based on selected category
	$: currentTitle = (() => {
		switch (selectedCategory) {
			case 'delegate': return 'Ordnungsrufe pro Abgeordneten';
			case 'party': return 'Ordnungsrufe nach Parteien';
			case 'gender': return 'Ordnungsrufe nach Gender';
			case 'age': return 'Ordnungsrufe nach Alter';
			case 'legis': return 'Ordnungsrufe nach Legislaturperiode';
			default: return 'Ordnungsrufe pro Abgeordneten';
		}
	})();
</script>

<svelte:head>
    <title>Ordnungsrufstatistiken</title>
    <meta name="description" content="Ordnungsrufstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Ordnungsrufstatistiken</h1>

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
