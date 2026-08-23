<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
import { t } from '$lib/i18n/i18n.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapAbsencesDelegate, mapAbsencesCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		'delegate.normalized':
			'Abwesenheiten je Abgeordneter oder Abgeordnetem als Anteil der Sitzungen.',
		'delegate.absolute': 'Gezählte Abwesenheiten je Abgeordneter oder Abgeordnetem.',
		'party.normalized': 'Abwesenheiten als Sitzungsanteil, nach Klubs zusammengefasst.',
		'party.absolute': 'Gezählte Abwesenheiten, nach Klubs zusammengefasst.',
		'gender.normalized': 'Abwesenheiten als Sitzungsanteil im Vergleich nach Geschlecht.',
		'gender.absolute': 'Gezählte Abwesenheiten im Vergleich nach Geschlecht.',
		'age.normalized': 'Abwesenheiten als Sitzungsanteil nach Altersgruppen.',
		'age.absolute': 'Gezählte Abwesenheiten nach Altersgruppen.',
		'legis.normalized': 'Abwesenheiten als Sitzungsanteil je Legislaturperiode.',
		'legis.absolute': 'Gezählte Abwesenheiten je Legislaturperiode.',
		'line.normalized': 'Entwicklung der Abwesenheitsquote über die Perioden.',
		'line.absolute': 'Entwicklung der gezählten Abwesenheiten über die Perioden.',
		'donut.normalized': 'Anteil der höchsten Abwesenheitsquoten in der aktuellen Auswahl.',
		'donut.absolute': 'Anteil der höchsten Abwesenheitszahlen in der aktuellen Auswahl.'
	};

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

		return mapAbsencesDelegate(response, normalized);
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

		return mapAbsencesCategory(response, normalized);
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

		return mapAbsencesCategory(response, normalized);
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

		return mapAbsencesCategory(response, normalized);
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

		return mapAbsencesCategory(response, normalized);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate':
				return delegateSimpleAbsences;
			case 'party':
				return partySimpleAbsences;
			case 'gender':
				return genderSimpleAbsences;
			case 'age':
				return ageSimpleAbsences;
			case 'legis':
				return legisSimpleAbsences;
			default:
				return delegateSimpleAbsences;
		}
	})();
</script>

<svelte:head>
	<title>Abwesenheitsstatistiken</title>
	<meta
		name="description"
		content="Abwesenheitsstatistiken über den Nationalrat und deren Abgeordnete"
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.absences.h1')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Abwesenheiten in Nationalratssitzungen, absolut oder als Anteil an Sitzungen.
		</p>
	</div>

	<StatisticsChartControl
		height={520}
		makeRequest={currentFunction}
		bind:selectedCategory
		valueLabel={t('statistics.absences.valueLabel')}
		normalizedValueLabel="Abwesenheitsquote (Anteil an Sitzungen)"
		{chartDescriptions}
	/>
</Container>
