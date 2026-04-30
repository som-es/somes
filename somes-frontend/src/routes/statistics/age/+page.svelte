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

<div class="text-base-font-color font-base dark:bg-surface-950 min-h-screen">
	<!-- Header -->
	<header class="sticky top-0 z-10 border-b border-surface-200 bg-surface-50/90 shadow-sm backdrop-blur-md dark:border-surface-700 dark:bg-surface-900/90">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-surface-900 dark:text-surface-50">Altersstatistiken</h1>
					<p class="mt-2 text-lg text-surface-600 dark:text-surface-400">
						Altersstruktur der Abgeordneten
					</p>
				</div>
				<div class="flex items-center gap-3">
					<div class="w-12 h-12 bg-primary-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
						👥
					</div>
				</div>
			</div>
		</div>
	</header>

	<!-- Main Content -->
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Chart -->
		<div class="bg-white dark:bg-surface-800 rounded-xl p-6 shadow-sm border border-surface-200 dark:border-surface-700">
			<DelegateBarChartControl
				height={500}
				delegateMakeRequest={currentFunction}
				selectedCategory={selectedCategory}
				valueLabel="Alter"
				normalizedValueLabel="Alter"
				filterConfig={{
					showNormalized: false,
					showPeriod: true,
					showGender: true,
					showParty: true
				}}
			/>
		</div>
	</div>
</div>
