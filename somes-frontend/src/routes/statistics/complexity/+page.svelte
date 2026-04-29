<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapComplexityDelegate, mapComplexityCategory } from '$lib/api/statistics-adapter';

	export let selectedCategory: string = 'delegate';

	const delegateSimpleComplexity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('complexity_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapComplexityDelegate(response);
	};

	const partySimpleComplexity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('complexity_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapComplexityCategory(response);
	};

	const genderSimpleComplexity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('complexity_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapComplexityCategory(response);
	};

	const ageSimpleComplexity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('complexity_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapComplexityCategory(response);
	};

	const legisSimpleComplexity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('complexity_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapComplexityCategory(response);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleComplexity;
			case 'party': return partySimpleComplexity;
			case 'gender': return genderSimpleComplexity;
			case 'age': return ageSimpleComplexity;
			case 'legis': return legisSimpleComplexity;
			default: return delegateSimpleComplexity;
		}
	})();

	// Static title
	$: currentTitle = 'Komplexitätsstatistiken';
</script>

<svelte:head>
	<title>Komplexitätsstatistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Analyse der Komplexität von parlamentarischen Reden" />
</svelte:head>

<!-- Main Content -->
<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Chart -->
		<div class="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-inner">
			<DelegateBarChartControl
				delegateMakeRequest={currentFunction}
				height={600}
			/>
		</div>
</div>

