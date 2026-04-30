<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapCallToOrdersDelegate, mapCallToOrdersCategory } from '$lib/api/statistics-adapter';

	export let selectedCategory: string = 'delegate';

	const delegateSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_by_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersDelegate(response, normalized);
	};

	const partySimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	const genderSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	const ageSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	const legisSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
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

	// Static title
	$: currentTitle = 'Ordnungsrufstatistiken';
</script>

<svelte:head>
    <title>Ordnungsrufstatistiken</title>
    <meta name="description" content="Ordnungsrufstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<!-- Chart -->
	<div class="bg-card rounded-xl p-6 shadow-sm">
		<DelegateBarChartControl
			delegateMakeRequest={currentFunction}
			height={500}
			selectedCategory={selectedCategory}
			valueLabel="Ordnungsrufe"
			normalizedValueLabel="Durchschnittliche Ordnungsrufe pro Sitzung"
		/>
	</div>
</div>
