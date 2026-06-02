<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapCallToOrdersDelegate, mapCallToOrdersCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		'delegate.normalized': 'Ordnungsrufe je besuchter Sitzung der Abgeordneten.',
		'delegate.absolute': 'Gezählte Ordnungsrufe je Abgeordneter oder Abgeordnetem.',
		'party.normalized': 'Ordnungsrufe je besuchter Sitzung, nach Parteien zusammengefasst.',
		'party.absolute': 'Gezählte Ordnungsrufe, nach Parteien zusammengefasst.',
		'gender.normalized': 'Ordnungsrufe je besuchter Sitzung im Vergleich nach Geschlecht.',
		'gender.absolute': 'Gezählte Ordnungsrufe im Vergleich nach Geschlecht.',
		'age.normalized': 'Ordnungsrufe je besuchter Sitzung nach Altersgruppen.',
		'age.absolute': 'Gezählte Ordnungsrufe nach Altersgruppen.',
		'legis.normalized': 'Ordnungsrufe je besuchter Sitzung, je Legislaturperiode.',
		'legis.absolute': 'Gezählte Ordnungsrufe je Legislaturperiode.',
		'line.normalized': 'Entwicklung der Ordnungsrufe je besuchter Sitzung über die Perioden.',
		'line.absolute': 'Entwicklung der gezählten Ordnungsrufe über die Perioden.',
		'donut.normalized': 'Anteil der höchsten Ordnungsrufquoten in der aktuellen Auswahl.',
		'donut.absolute': 'Anteil der höchsten Ordnungsrufzahlen in der aktuellen Auswahl.'
	};

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
			case 'delegate':
				return delegateSimpleCallsToOrder;
			case 'party':
				return partySimpleCallsToOrder;
			case 'gender':
				return genderSimpleCallsToOrder;
			case 'age':
				return ageSimpleCallsToOrder;
			case 'legis':
				return legisSimpleCallsToOrder;
			default:
				return delegateSimpleCallsToOrder;
		}
	})();
</script>

<svelte:head>
	<title>Ordnungsrufstatistiken</title>
	<meta
		name="description"
		content="Ordnungsrufstatistiken über den Nationalrat und deren Abgeordnete"
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Ordnungsrufstatistiken</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Wer im Parlament wie oft ermahnt wurde.
		</p>
	</div>

	<StatisticsChartControl
		makeRequest={currentFunction}
		height={520}
		bind:selectedCategory
		valueLabel="Ordnungsrufe"
		normalizedValueLabel="Ordnungsrufe pro besuchter Sitzung"
		{chartDescriptions}
	/>
</Container>
