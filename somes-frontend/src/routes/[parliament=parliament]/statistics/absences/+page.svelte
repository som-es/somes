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
			t('statistics.absences.desc.delegateNormalized'),
		'delegate.absolute': t('statistics.absences.desc.delegateAbsolute'),
		'party.normalized': t('statistics.absences.desc.partyNormalized'),
		'party.absolute': t('statistics.absences.desc.partyAbsolute'),
		'gender.normalized': t('statistics.absences.desc.genderNormalized'),
		'gender.absolute': t('statistics.absences.desc.genderAbsolute'),
		'age.normalized': t('statistics.absences.desc.ageNormalized'),
		'age.absolute': t('statistics.absences.desc.ageAbsolute'),
		'legis.normalized': t('statistics.absences.desc.legisNormalized'),
		'legis.absolute': t('statistics.absences.desc.legisAbsolute'),
		'line.normalized': t('statistics.absences.desc.lineNormalized'),
		'line.absolute': t('statistics.absences.desc.lineAbsolute'),
		'donut.normalized': t('statistics.absences.desc.donutNormalized'),
		'donut.absolute': t('statistics.absences.desc.donutAbsolute')
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
	<title>{t('statistics.absences.title')}</title>
	<meta
		name="description"
		content={t('statistics.absences.description')}
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.absences.h1')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.absences.intro')}
		</p>
	</div>

	<StatisticsChartControl
		height={520}
		makeRequest={currentFunction}
		bind:selectedCategory
		valueLabel={t('statistics.absences.valueLabel')}
		normalizedValueLabel={t('statistics.absences.normalizedValueLabel')}
		{chartDescriptions}
	/>
</Container>
