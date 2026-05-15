<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapTotalSpeechesDelegate, mapTotalSpeechesCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

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
	<meta name="description" content="Analyse der Anzahl der parlamentarischen Reden" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Anzahl der Reden</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Analyse der Anzahl parlamentarischer Reden.
		</p>
	</div>

	<DelegateBarChartControl
		delegateMakeRequest={currentFunction}
		height={560}
		{selectedCategory}
		valueLabel="Anzahl Reden"
		normalizedValueLabel="Anzahl Reden"
		filterConfig={{
			showNormalized: false,
			showPeriod: true,
			showGender: true,
			showParty: true
		}}
	/>
</Container>
