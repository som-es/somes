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

<div class="text-base-font-color font-base dark:bg-surface-950 min-h-screen">
	<!-- Header -->
	<header class="sticky top-0 z-10 border-b border-surface-200 bg-surface-50/90 shadow-sm backdrop-blur-md dark:border-surface-700 dark:bg-surface-900/90">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-surface-900 dark:text-surface-50">Komplexitätsstatistiken</h1>
					<p class="mt-2 text-lg text-surface-600 dark:text-surface-400">
						Analyse der Komplexität von parlamentarischen Reden
					</p>
				</div>
				<div class="flex items-center gap-3">
					<div class="w-12 h-12 bg-primary-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
						🧠
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
				valueLabel="Komplexitäts-Score"
				normalizedValueLabel="Komplexitäts-Score"
				filterConfig={{
					showNormalized: false,
					showPeriod: true,
					showGender: true,
					showParty: true
				}}
				infoQuestion="Wie wird der Komplexitäts-Score berechnet?"
				infoAnswer="
					<p><strong>Komplexitäts-Score:</strong> Durchschnittliche Komplexität der Vorstösse</p>
					<ul class='ml-4 space-y-1 text-xs'>
						<li>• Motion: <strong>1.0 Punkte</strong></li>
						<li>• Postulat: <strong>1.2 Punkte</strong></li>
						<li>• Anfrage: <strong>1.2 Punkte</strong></li>
						<li>• Dringliche Anfrage: <strong>1.15 Punkte</strong></li>
						<li>• Initiative: <strong>1.3 Punkte</strong></li>
					</ul>
					<p class='mt-2'><strong>Berechnung:</strong> Durchschnitt der Komplexitätswerte aller Vorstösse des Delegierten</p>
				"
			/>
		</div>
	</div>
</div>
