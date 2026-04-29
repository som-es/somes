<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapAbsencesDelegate, mapAbsencesCategory } from '$lib/api/statistics-adapter';

	export let selectedCategory: string = 'delegate';

	const delegateSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('absences_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapAbsencesDelegate(response);
	};

	const partySimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('absences_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapAbsencesCategory(response);
	};

	const genderSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('absences_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapAbsencesCategory(response);
	};

	const ageSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('absences_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapAbsencesCategory(response);
	};

	const legisSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('absences_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapAbsencesCategory(response);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleAbsences;
			case 'party': return partySimpleAbsences;
			case 'gender': return genderSimpleAbsences;
			case 'age': return ageSimpleAbsences;
			case 'legis': return legisSimpleAbsences;
			default: return delegateSimpleAbsences;
		}
	})();

	// Static title
	$: currentTitle = 'Abwesenheitsstatistiken';
</script>

<svelte:head>
    <title>Abwesenheitsstatistiken</title>
    <meta name="description" content="Abwesenheitsstatistiken über den Nationalrat und deren Abgeordnete" />
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
