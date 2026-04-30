<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapTotalSpeechesDelegate, mapTotalSpeechesCategory } from '$lib/api/statistics-adapter';

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
			case 'delegate': return delegateSimpleTotalSpeeches;
			case 'party': return partySimpleTotalSpeeches;
			case 'gender': return genderSimpleTotalSpeeches;
			case 'age': return ageSimpleTotalSpeeches;
			case 'legis': return legisSimpleTotalSpeeches;
			default: return delegateSimpleTotalSpeeches;
		}
	})();

	// Static title
	$: currentTitle = 'Anzahl der Reden';
</script>

<svelte:head>
	<title>Anzahl der Reden - Parlamentsinformationssystem</title>
	<meta name="description" content="Analyse der Anzahl der parlamentarischen Reden" />
</svelte:head>

	

<div class="text-base-font-color font-base dark:bg-surface-950 min-h-screen">
	<!-- Header -->
	<header class="sticky top-0 z-10 border-b border-surface-200 bg-surface-50/90 shadow-sm backdrop-blur-md dark:border-surface-700 dark:bg-surface-900/90">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-surface-900 dark:text-surface-50">Anzahl der Reden</h1>
					<p class="mt-2 text-lg text-surface-600 dark:text-surface-400">
						Analyse der Anzahl der parlamentarischen Reden
					</p>
				</div>
				<div class="flex items-center gap-3">
					<div class="w-12 h-12 bg-primary-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
						🎙
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
				delegateMakeRequest={currentFunction}
				height={600}
				selectedCategory={selectedCategory}
				valueLabel="Anzahl Reden"
				normalizedValueLabel="Anzahl Reden"
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
