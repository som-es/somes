<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapComplexityDelegate, mapComplexityCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

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
		const response = await justPostStatistics<any[]>('complexity_at_age', {
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
			case 'delegate':
				return delegateSimpleComplexity;
			case 'party':
				return partySimpleComplexity;
			case 'gender':
				return genderSimpleComplexity;
			case 'age':
				return ageSimpleComplexity;
			case 'legis':
				return legisSimpleComplexity;
			default:
				return delegateSimpleComplexity;
		}
	})();
</script>

<svelte:head>
	<title>Komplexitätsstatistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Analyse der Komplexität von parlamentarischen Reden" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Komplexitätsstatistiken</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Analyse der Komplexität parlamentarischer Reden.
		</p>
	</div>

	<DelegateBarChartControl
		height={520}
		delegateMakeRequest={currentFunction}
		bind:selectedCategory
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
				<li>Motion: <strong>1.0 Punkte</strong></li>
				<li>Postulat: <strong>1.2 Punkte</strong></li>
				<li>Anfrage: <strong>1.2 Punkte</strong></li>
				<li>Dringliche Anfrage: <strong>1.15 Punkte</strong></li>
				<li>Initiative: <strong>1.3 Punkte</strong></li>
			</ul>
			<p class='mt-2'><strong>Berechnung:</strong> Durchschnitt der Komplexitätswerte aller Vorstösse des Delegierten</p>
		"
	/>
</Container>
