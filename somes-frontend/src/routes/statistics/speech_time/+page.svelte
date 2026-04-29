<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapSpeechTimeDelegate, mapSpeechTimeCategory } from '$lib/api/statistics-adapter';

	const delegateSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeDelegate(response);
	};

	const partySimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response);
	};

	const genderSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response);
	};

	const ageSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response);
	};

	const legisSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response);
	};

	// Use delegate function by default (category is controlled from parent)
	$: currentFunction = delegateSimpleSpeechTime;

	// Static title
	$: currentTitle = 'Redezeitstatistiken';
</script>

<svelte:head>
	<title>Redezeitstatistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Analyse der Redezeiten im Parlament" />
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

