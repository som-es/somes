<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapTotalSpeechesDelegate, mapTotalSpeechesCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		delegate: 'Gezählte Reden je Abgeordneter oder Abgeordnetem.',
		party: 'Gezählte Reden, nach Parteien zusammengefasst.',
		gender: 'Gezählte Reden im Vergleich nach Geschlecht.',
		age: 'Gezählte Reden nach Altersgruppen.',
		legis: 'Gezählte Reden je Legislaturperiode.',
		line: 'Entwicklung der gezählten Reden über die Legislaturperioden.',
		donut: 'Anteil der höchsten Redeanzahlen in der aktuellen Auswahl.'
	};

	const delegateSimpleTotalSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('total_speeches_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapTotalSpeechesDelegate(response);
	};

	const partySimpleTotalSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('total_speeches_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapTotalSpeechesCategory(response);
	};

	const genderSimpleTotalSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('total_speeches_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapTotalSpeechesCategory(response);
	};

	const ageSimpleTotalSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('total_speeches_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapTotalSpeechesCategory(response);
	};

	const legisSimpleTotalSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('total_speeches_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapTotalSpeechesCategory(response);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate':
				return delegateSimpleTotalSpeeches;
			case 'party':
				return partySimpleTotalSpeeches;
			case 'gender':
				return genderSimpleTotalSpeeches;
			case 'age':
				return ageSimpleTotalSpeeches;
			case 'legis':
				return legisSimpleTotalSpeeches;
			default:
				return delegateSimpleTotalSpeeches;
		}
	})();
</script>

<svelte:head>
	<title>Anzahl der Reden - Parlamentsinformationssystem</title>
	<meta name="description" content="Anzahl parlamentarischer Reden nach Personen und Gruppen" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Anzahl der Reden</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Wie oft im Parlament gesprochen wurde, nach Personen und Gruppen.
		</p>
	</div>

	<StatisticsChartControl
		makeRequest={currentFunction}
		height={560}
		bind:selectedCategory
		valueLabel="Anzahl Reden"
		normalizedValueLabel="Anzahl Reden"
		{chartDescriptions}
		filterConfig={{
			showNormalized: false,
			showPeriod: true,
			showGender: true,
			showParty: true
		}}
	/>
</Container>
