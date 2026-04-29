<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapAgeDelegate, mapAgeCategory } from '$lib/api/statistics-adapter';

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
			case 'delegate': return delegateSimpleAge;
			case 'party': return partySimpleAge;
			case 'gender': return genderSimpleAge;
			case 'age': return ageSimpleAge;
			case 'legis': return legisSimpleAge;
			default: return delegateSimpleAge;
		}
	})();

	// Static title
	$: currentTitle = 'Altersstatistiken';
</script>

<svelte:head>
    <title>Altersstatistiken</title>
    <meta name="description" content="Altersstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<!-- Chart -->
	<div class="bg-card rounded-xl p-6 shadow-sm">
		<DelegateBarChartControl
			height={500}
			delegateMakeRequest={currentFunction}
		/>
	</div>
</div>
