<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapAgeDelegate, mapAgeCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const delegateSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('age_of_delegates', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapAgeDelegate(response);
	};

	const partySimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('age_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapAgeCategory(response);
	};

	const genderSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('age_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapAgeCategory(response);
	};

	const ageSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('age_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapAgeCategory(response);
	};

	const legisSimpleAge = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('age_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapAgeCategory(response);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate':
				return delegateSimpleAge;
			case 'party':
				return partySimpleAge;
			case 'gender':
				return genderSimpleAge;
			case 'age':
				return ageSimpleAge;
			case 'legis':
				return legisSimpleAge;
			default:
				return delegateSimpleAge;
		}
	})();
</script>

<svelte:head>
	<title>Altersstatistiken</title>
	<meta name="description" content="Altersstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Altersstatistiken</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">Altersstruktur der Abgeordneten.</p>
	</div>

	<DelegateBarChartControl
		height={520}
		delegateMakeRequest={currentFunction}
		{selectedCategory}
		valueLabel="Alter"
		normalizedValueLabel="Alter"
		filterConfig={{
			showNormalized: false,
			showPeriod: true,
			showGender: true,
			showParty: true
		}}
	/>
</Container>
